use btleplug::api::CharPropFlags;
use btleplug::api::{
    Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use colored::Colorize;
use futures::stream::StreamExt;
use std::fmt::Display;
use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tokio::time;
use uuid::Uuid;

use crate::context::Context;

/// This is a unique identifier for the brain. If a device has a service with this UUID we know that its a brain.
pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13d5);

/// These are the UUIDs for the different characteristics that the brain has. These are essentially the ports that the brain wants us to communicate with, and reading/writing to different ones means different things.

/// Writing to this characteristic allows you to write to stdin on the brain.
pub const WRITE_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1306);

/// Reading from this characteristic allows you to read stdout from the brain.
pub const READ_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb1316);

/// This characteristic is used exclusively for connecting and authenticating with the brain.
pub const SYSTEM_CHAR: Uuid = Uuid::from_u128(0x08590f7edb05467e875772f6faeb13e5);

/// This struct is meant for sending data about a brain to other parts of the program.
#[derive(Debug, Clone)]
pub struct BrainData {
    pub name: String,
    pub address: String,
}

impl Display for BrainData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} \n Name: {} \n Address: {}",
            "===============".blue(),
            self.name,
            self.address
        )
    }
}

#[derive(Error, Debug, Clone)]
pub enum BrainControllerError {
    #[error("Incorrect code provided")]
    IncorrectCode,

    #[error("No brain is currently connected")]
    NoBrainConnected,

    #[error("Could not connect to the brain")]
    CouldNotConnect,
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

    fn find_characteristic(brain: &Peripheral, uuid: Uuid) -> Option<Characteristic> {
        for characteristic in brain.characteristics() {
            if characteristic.uuid == uuid {
                return Some(characteristic);
            }
        }
        None
    }

    // search for brains
    pub async fn search(&mut self) -> Result<Vec<BrainData>, BrainControllerError> {
        self.central
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");

        time::sleep(Duration::from_secs(2)).await;

        let mut brains: Vec<Peripheral> = vec![];

        for peripheral in self.central.peripherals().await.unwrap() {
            let properties = peripheral.properties().await.unwrap().unwrap();
            if properties
                .services
                .iter()
                .any(|service| service == &SERVICE_UUID)
            {
                brains.push(peripheral);
            }
        }

        let mut brain_data = vec![];

        for brain in brains.iter() {
            let properties = brain.properties().await.unwrap().unwrap();
            let name = properties.local_name.unwrap();
            let address = properties.address.to_string();

            let data = BrainData { name, address };

            brain_data.push(data);
        }

        self.available_brains = brains;

        Ok(brain_data)
    }

    // This function should only be run on peripheral that are verified to be brains.
    async fn find_brain_with_name(brains: &Vec<Peripheral>, name: String) -> Option<Peripheral> {
        for brain in brains {
            let local_name = brain
                .properties()
                .await
                .unwrap()
                .unwrap()
                .local_name
                .unwrap();

            let trimmed_name = local_name.trim();

            if trimmed_name == name {
                return Some(brain.clone());
            }
        }
        None
    }

    pub async fn connect(&mut self, name: String) -> Result<(), BrainControllerError> {
        let brain = match Self::find_brain_with_name(&self.available_brains, name).await {
            Some(brain) => brain,
            None => return Err(BrainControllerError::CouldNotConnect),
        };

        match brain.connect().await {
            Ok(_) => {
                self.connected_brain = Some(brain.clone());
            }
            Err(_) => {
                return Err(BrainControllerError::CouldNotConnect);
            }
        };

        brain.discover_services().await.unwrap();

        Ok(())
    }

    // make brain reveal code
    pub async fn ping_brain_for_code(&mut self) -> Result<(), BrainControllerError> {
        if self.connected_brain.is_none() {
            return Err(BrainControllerError::NoBrainConnected);
        }
        let brain = self.connected_brain.clone().unwrap();

        let characteristic = Self::find_characteristic(&brain, SYSTEM_CHAR).unwrap();
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

                let characteristic = Self::find_characteristic(&brain, SYSTEM_CHAR).unwrap();

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

    pub async fn poll<F, Fut>(&mut self, func: F, ctx: Context) -> Result<(), BrainControllerError>
    where
        F: Fn(Vec<u8>, Context) -> Fut,
        Fut: Future<Output = ()>,
    {
        if self.connected_brain.is_none() {
            return Err(BrainControllerError::NoBrainConnected);
        }

        let brain = self.connected_brain.clone().unwrap();
        brain.discover_services().await.unwrap();

        for characteristic in brain.characteristics() {
            // Subscribe to notifications from the characteristic with the selected
            // UUID.
            if characteristic.uuid == READ_CHAR
                && characteristic.properties.contains(CharPropFlags::NOTIFY)
            {
                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                brain.subscribe(&characteristic).await.unwrap();
                let mut notification_stream = brain.notifications().await.unwrap();
                while let Some(data) = notification_stream.next().await {
                    func(data.value, ctx.clone()).await;
                }
            }
        }

        Ok(())
    }
}
