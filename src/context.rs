use config::Config;
use rumqttc::{AsyncClient, MqttOptions};

#[derive(Debug, Clone)]
pub struct Context {
    pub mqtt_client: AsyncClient,
    //pub config: Config,
}

impl Context {
    pub fn new() -> Self {
        //let config = Config::builder().build().unwrap();

        let mqttoptions = MqttOptions::new("loginator", "localhost", 1883);

        let (mqtt_client, _) = AsyncClient::new(mqttoptions, 10);

        Self { mqtt_client }
    }
}
