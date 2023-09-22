use btleplug::api::CharPropFlags;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use thiserror::Error;
use tokio::time;
use uuid::Uuid;

use crate::context;

pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13d5);

pub const WRITE_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1306); // Write
pub const READ_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1316); // Read
pub const SYSTEM_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13e5); // System

fn find_characteristic(brain: &Peripheral, uuid: Uuid) -> Option<Characteristic> {
    for characteristic in brain.characteristics() {
        if characteristic.uuid == uuid {
            return Some(characteristic);
        }
    }
    None
}

type PollingFunction = fn(Vec<u8>, &mut context::Context);

#[derive(Error, Debug, Clone)]
pub enum BrainControllerError {
    #[error("Incorrect code provided")]
    IncorrectCode,

    #[error("No brain is currently connected")]
    NoBrainConnected,
}

// Acts as a layer of abstraction over the btleplug library. Handles authentication and connection with the brain
#[derive(Debug, Clone)]
pub struct BrainController {
    central: Adapter,
    available_brains: Vec<Peripheral>,
    connected_brain: Option<Peripheral>,
}

impl BrainController {
    pub async fn new() -> Self {
        let manager = Manager::new().await.unwrap();
        let adapters = manager
            .adapters()
            .await
            .expect("Your device does not support bluetooth");
        let central = adapters.into_iter().nth(0).unwrap();

        BrainController {
            central,
            available_brains: vec![],
            connected_brain: None,
        }
    }

    // search for brains
    pub async fn search(&mut self) {
        println!("Starting scan...");

        let brain_filter = ScanFilter {
            // FIXME: this filter does not appear to work
            services: vec![SERVICE_UUID],
        };

        self.central
            .start_scan(brain_filter)
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;

        self.available_brains = self.central.peripherals().await.unwrap();
    }

    pub async fn connect(&mut self) {
        let brain = self.available_brains[0].clone();
        brain.connect().await;
        self.connected_brain = Some(brain);
    }
    // make brain reveal code
    pub async fn ping_brain_for_code(&mut self) -> Result<(), BrainControllerError> {
        if self.connected_brain.is_none() {
            return Err(BrainControllerError::NoBrainConnected);
        }
        let brain = self.connected_brain.clone().unwrap();

        let characteristic = find_characteristic(&brain, SYSTEM_CHAR).unwrap();
        brain
            .write(
                &characteristic,
                &[0xff, 0xff, 0xff, 0xff], // This tells the brain to display the verification code on it's screen
                WriteType::WithResponse,
            )
            .await
            .unwrap();
        Ok(())
    }
    // get user input (get code)

    // send code to brain
    // verify that code was correct
    pub async fn authenticate(&mut self, code: &str) -> Result<(), BrainControllerError> {
        let brain = self.connected_brain.clone().unwrap();

        match &self.connected_brain {
            Some(brain) => {
                let code: Vec<u8> = code
                    .trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect();

                let characteristic = find_characteristic(&brain, SYSTEM_CHAR).unwrap();

                brain
                    .write(&characteristic, &code, WriteType::WithResponse)
                    .await
                    .unwrap();

                // If the code was correct the brain will echo it back to us from the same characteristic
                let respose = brain.read(&characteristic).await.unwrap();

                if respose != code {
                    return Err(BrainControllerError::IncorrectCode);
                }
            }
            None => return Err(BrainControllerError::NoBrainConnected),
        }
        Ok(())
    }

    pub async fn poll(&mut self, func: PollingFunction) -> Result<(), BrainControllerError> {
        if self.connected_brain.is_none() {
            return Err(BrainControllerError::NoBrainConnected);
        }

        let brain = self.connected_brain.clone().unwrap();
        brain.discover_services().await.unwrap();

        for characteristic in brain.characteristics() {
            println!("Checking characteristic {:?}", characteristic);
            // Subscribe to notifications from the characteristic with the selected
            // UUID.
            if characteristic.uuid == READ_CHAR
                && characteristic.properties.contains(CharPropFlags::NOTIFY)
            {
                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                brain.subscribe(&characteristic).await.unwrap();
                let mut notification_stream = brain.notifications().await.unwrap();
                while let Some(data) = notification_stream.next().await {
                    func(data.value)
                }
            }
        }

        Ok(())
    }
}
