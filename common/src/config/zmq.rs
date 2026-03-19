use serde::{Deserialize, Serialize};
use crate::config::topics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZmqConfig {
    pub enabled: bool,
    #[serde(default)]
    pub publisher_address: String,
    #[serde(default)]
    pub topic: String,
    #[serde(default)]
    pub subscriber_enabled: bool,
    #[serde(default)]
    pub subscriber_address: String,
    #[serde(default)]
    pub write_topic: String,
    #[serde(default)]
    pub config_update_topic: String,
    #[serde(default)]
    pub high_water_mark: Option<u32>,
}

impl Default for ZmqConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            publisher_address: "tcp://*:5555".to_string(),
            topic: topics::DATA_PUBLISH_TOPIC.to_string(),
            subscriber_enabled: true,
            subscriber_address: "tcp://localhost:5556".to_string(),
            write_topic: topics::WRITE_REQUEST_TOPIC.to_string(),
            config_update_topic: topics::CONFIG_UPDATE_TOPIC.to_string(),
            high_water_mark: Some(1000),
        }
    }
}
