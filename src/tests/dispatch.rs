use crate::{
    context,
    dispatch::{self, dispatch},
};

#[tokio::test]
async fn data() {
    let context = context::Context::new().await;
    let test_data = vec![
        37, 115, 111, 117, 116, 50, 44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 44, 51, 48, 46, 48, 48,
        48, 48, 48, 48, 44, 51, 48, 46, 48, 48, 48, 48, 48, 48, 10, 0,
    ];

    context
        .mqtt_client
        .subscribe("left_motors/temperature", rumqttc::QoS::AtLeastOnce)
        .await
        .unwrap();

    dispatch(test_data, context).await;
    // FIXME: actually listen for data
}
