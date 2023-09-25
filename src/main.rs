mod bluetooth;
mod context;
mod decoder;
mod dispatch;
mod tests;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

use tokio::{task, time};

use context::Context;

#[tokio::main]
async fn main() {
    let context = Context::new().await;

    let mut controller = bluetooth::BrainController::new().await;
    controller.search().await;
    controller.connect().await;
    controller.ping_brain_for_code().await.unwrap();
    controller.authenticate("2381").await.unwrap();
    controller.poll(dispatch::dispatch, context).await.unwrap();
}
