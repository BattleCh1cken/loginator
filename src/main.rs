mod bluetooth;
mod context;
mod dispatch;

use context::Context;

#[tokio::main]
async fn main() {
    let mut context = Context::new();

    let mut controller = bluetooth::BrainController::new().await;
    controller.connect().await;
    controller.ping_brain_for_code().await.unwrap();
    controller.authenticate("8321").await.unwrap();

    controller.poll(dispatch::dispatch, context).await.unwrap();
}
