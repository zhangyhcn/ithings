use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::config::topics::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverConfig {
    pub driver_name: String,
    pub driver_type: String,
    pub device_instance_id: String,
    pub poll_interval_ms: u64,
    #[serde(default)]
    pub zmq: ZmqConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

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
            topic: DATA_PUBLISH_TOPIC.to_string(),
            subscriber_enabled: true,
            subscriber_address: "tcp://localhost:5556".to_string(),
            write_topic: WRITE_REQUEST_TOPIC.to_string(),
            config_update_topic: CONFIG_UPDATE_TOPIC.to_string(),
            high_water_mark: Some(1000),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
        }
    }
}

impl DriverConfig {
    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let builder = match extension.to_lowercase().as_str() {
            "json" => {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| config::ConfigError::Message(format!("Failed to read file: {}", e)))?;
                config::Config::builder()
                    .add_source(config::File::from_str(&content, config::FileFormat::Json))
            }
            _ => {
                config::Config::builder()
                    .add_source(config::File::with_name(path))
            }
        };
        
        let settings = builder
            .add_source(config::Environment::with_prefix("DRIVER"))
            .build()?;
        
        settings.try_deserialize()
    }

    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("DRIVER"))
            .build()?;
        
        settings.try_deserialize()
    }
}