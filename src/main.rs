mod bluetooth;
mod context;
mod dispatch;

#[tokio::main]
async fn main() {
    let mut context = context::Context::new();
    let mut controller = bluetooth::BrainController::new().await;
    controller.connect().await;
    controller.ping_brain_for_code().await.unwrap();
    controller.authenticate("8321").await.unwrap();
    controller.poll(dispatch::dispatch).await.unwrap();
}
