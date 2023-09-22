use rumqttc::Client;

use crate::context::Context;

pub fn dispatch(input: Vec<u8>, ctx: &mut Context) {
    ctx.mqtt_client
        .publish("bob", rumqttc::QoS::AtMostOnce, false, input);
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
