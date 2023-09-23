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

    match decode(input) {
        Err(_) => eprintln!("oopsy woopsy"),
        Ok(data) => {
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
    };
}
