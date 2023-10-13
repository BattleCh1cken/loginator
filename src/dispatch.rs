use crate::context::Context;
use crate::decoder::Decoder;
use anyhow::Result;
use regex::Regex;
use rumqttc::{AsyncClient, Client, MqttOptions};
use thiserror::Error;

use std::collections::HashMap;

#[derive(Error, Debug, Clone)]
pub enum DispatchError {
    #[error("Couldn't decode")]
    DecodeError,
}

pub async fn dispatch(input: Vec<u8>, mut ctx: Context) {
    match ctx.decoder.push(input) {
        Err(_) => eprintln!("There was an error while decoding cobs input"),
        Ok(None) => println!("data is valid so far, but not complete yet"),
        Ok(Some(data)) => {
            if let Some(data) = Decoder::parse(data) {
                let mut payload = HashMap::new();

                let route = &ctx.config.routes[data[0] as usize];
                println!("route: {:?}", route);

                for (index, label) in route.1.iter().enumerate() {
                    payload.insert(label.to_string(), data[index + 1]);
                }

                let json_data = serde_json::to_string(&payload).unwrap();

                println!("topic: {:?}, data: {:?}", route.0, json_data);
                let data_bytes = json_data.into_bytes();

                ctx.mqtt_client
                    .publish(route.0.clone(), rumqttc::QoS::AtLeastOnce, true, data_bytes)
                    .await
                    .unwrap();
            } else {
                println!("recieved data wasn't telemetry :)")
            }
        }
    };
}
