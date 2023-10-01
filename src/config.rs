use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub brain_name: String,
    pub brain_code: String,
    pub routes: Vec<(String, Vec<String>)>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            brain_name: "VEX_V5".to_string(),
            brain_code: "0000".to_string(),
            routes: vec![
                (
                    "pid/linear".into(),
                    vec!["P".into(), "I".into(), "D".into()],
                ),
                (
                    "pid/angular".into(),
                    vec!["P".into(), "I".into(), "D".into()],
                ),
                (
                    "joysticks/x".to_string(),
                    vec!["Left".to_string(), "Right".to_string()],
                ),
                (
                    "joysticks/y".to_string(),
                    vec!["Left".to_string(), "Right".to_string()],
                ),
            ],
        }
    }
}
