use crate::context::Context;
use anyhow::Result;
use regex::Regex;

use std::collections::HashMap;

pub fn decode(data: Vec<u8>) -> Result<Vec<f32>> {
    let mut buffer = String::new();

    for byte in data {
        buffer.push(byte as char);
    }
    buffer = buffer.replace("\n", "");

    let regex = Regex::new(r"[-+]?\d*\.\d+|[-+]?\d+").unwrap();

    let mut result = vec![];

    for capture in regex.captures_iter(&buffer) {
        let number: f32 = capture.get(0).unwrap().as_str().parse()?;
        result.push(number);
    }

    println!("{:#?}", result);

    Ok(vec![0.0])
}

pub async fn dispatch(input: Vec<u8>, ctx: Context) {
    // TODO: get these from the config
    let routes = vec![
        ("pid/linear", vec!["P", "I", "D"]),
        ("pid/angular", vec!["P", "I", "D"]),
        (
            "left_motors/temperature",
            vec!["First motor", "Second motor", "Third motor"],
        ),
    ];

    let data = decode(input).unwrap();
    let mut payload = HashMap::new();

    let route = &routes[data[0] as usize];

    for (index, label) in route.1.iter().enumerate() {
        payload.insert(label.to_string(), data[index + 1]);
    }

    let json_data = serde_json::to_string(&route).unwrap();
    let data_bytes = json_data.into_bytes();

    ctx.mqtt_client
        .publish(route.0, rumqttc::QoS::AtMostOnce, false, data_bytes)
        .await
        .unwrap();
}

/*
use bincode::ErrorKind;
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::TryFrom;
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    p: usize,
    i: usize,
    d: usize,
}

impl From<&Message> for Vec<u8> {
    fn from(value: &Message) -> Self {
        let json_str = serde_json::to_string(&value).unwrap();
        json_str.into_bytes()
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        let json_str = serde_json::to_string(&value).unwrap();
        json_str.into_bytes()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Box<ErrorKind>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let json_str = String::from_utf8(value.to_vec()).unwrap();
        let result: Self = serde_json::from_str(&json_str).unwrap();
        Ok(result)
    }
}

fn main() {
    let mqqt_opts = MqttOptions::new("test-1", "localhost", 1883);

    let (mut client, mut connection) = Client::new(mqqt_opts, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    thread::spawn(move || loop {
        let message = Message {
            p: rand::random(),
            i: rand::random(),
            d: rand::random(),
        };

        client
            .publish("hello/rumqtt", QoS::AtLeastOnce, false, message)
            .unwrap();
        thread::sleep(Duration::from_millis(100));
    });

    // Iterate to poll the eventloop for connection progress
    for notification in connection.iter() {
        if let Ok(Event::Incoming(Incoming::Publish(packet))) = notification {
            match Message::try_from(packet.payload.as_ref()) {
                Ok(message) => println!("Payload = {message:?}"),
                Err(error) => println!("Error = {error}"),
            }
        }
    }
}
*/
