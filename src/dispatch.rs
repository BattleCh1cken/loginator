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
    DecodeError(),
}

pub async fn dispatch(input: Vec<u8>, mut ctx: Context) {
    // TODO: get these from the config
    let routes = vec![
        ("pid/linear", vec!["P", "I", "D"]),
        ("pid/angular", vec!["P", "I", "D"]),
        (
            "motors/temperature",
            vec![
                "First left motor",
                "Second left motor",
                "Third left motor",
                "First right motor",
                "Second right motor",
                "Third right motor",
            ],
        ),
        ("joysticks", vec!["Left", "Right"]),
    ];

    match ctx.decoder.push(input) {
        Err(_) => eprintln!("oopsy woopsy"),
        Ok(None) => println!("data is valid so far, but not complete yet"),
        Ok(Some(data)) => {
            let data = Decoder::parse(data).unwrap();
            println!("{:?}", data);

            let mut payload = HashMap::new();
            let route = &routes[data[0] as usize];

            for (index, label) in route.1.iter().enumerate() {
                println!("{} {}", label, data[index + 1]);
                payload.insert(label.to_string(), data[index + 1]);
            }

            let json_data = serde_json::to_string(&payload).unwrap();
            println!("topic: {:?}, data: {:?}", route.0, json_data);
            let data_bytes = json_data.into_bytes();

            ctx.mqtt_client
                .publish(route.0, rumqttc::QoS::AtLeastOnce, true, data_bytes)
                .await
                .unwrap();
        }
    };
}
