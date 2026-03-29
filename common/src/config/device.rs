use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{ZmqConfig, MqttConfig, KafkaConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub device_name: String,
    pub device_type: String,
    pub poll_interval_ms: u64,
    #[serde(default)]
    pub zmq: ZmqConfig,
    #[serde(default)]
    pub mqtt: MqttConfig,
    #[serde(default)]
    pub kafka: KafkaConfig,
    #[serde(default)]
    pub driver: DriverClientConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl DeviceConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DeviceConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn from_env() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("DEVICE"))
            .build()?;
        Ok(config.try_deserialize()?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverClientConfig {
    pub enabled: bool,
    pub server_address: String,
    #[serde(default)]
    pub router_address: Option<String>,
    #[serde(default)]
    pub router_sub_port: Option<u16>,
}

impl Default for DriverClientConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_address: String::new(),
            router_address: Some("tcp://localhost".to_string()),
            router_sub_port: Some(5550),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
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
