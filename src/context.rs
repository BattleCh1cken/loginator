use config::Config;
use rumqttc::{Client, MqttOptions};

pub struct Context {
    pub mqtt_client: Client,
    pub config: Config,
}

impl Context {
    pub fn new() {
        let config = Config::builder().build().unwrap();

        let mut mqttoptions = MqttOptions::new("loginator", "localhost", 1883);

        let (mut mqtt_client, mut connection) = Client::new(mqttoptions, 10);

        Self {
            mqtt_client,
            config,
        };
    }
}
