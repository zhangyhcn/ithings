use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub enabled: bool,
    pub brokers: String,
    pub topic_prefix: String,
    pub write_topic: String,
    pub consumer_group: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            brokers: "localhost:9092".to_string(),
            topic_prefix: "devices".to_string(),
            write_topic: "driver-write".to_string(),
            consumer_group: "device-group".to_string(),
            username: None,
            password: None,
        }
    }
}
