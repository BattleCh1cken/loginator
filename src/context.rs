use config::Config;
use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;
use tokio::{task, time};

use crate::decoder::Decoder;

#[derive(Debug, Clone)]
pub struct Context {
    pub mqtt_client: AsyncClient,
    pub decoder: Decoder,
}

impl Context {
    pub async fn new() -> Self {
        // Create the MQTT client.
        let mut mqttoptions = MqttOptions::new("loginator", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // The eventloop needs to be polled repeatedly, otherwise mqtt actions cannot take place.
        task::spawn(async move {
            loop {
                let event = eventloop.poll().await;
                match &event {
                    Ok(_v) => {
                        //println!("Event = {v:?}");
                    }
                    Err(e) => {
                        println!("Error = {e:?}");
                    }
                };
                time::sleep(Duration::from_millis(10)).await;
            }
        });

        let decoder = Decoder::default();

        Self {
            mqtt_client,
            decoder,
        }
    }
}
