use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::DeviceConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroupConfig {
    pub tenant_id: String,
    pub org_id: String,
    pub site_id: String,
    pub namespace_id: String,
    pub remote_transport: RemoteTransportConfig,
    pub devices: Vec<DeviceInGroupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteTransportConfig {
    pub r#type: String,
    pub broker: Option<String>,
    pub brokers: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInGroupConfig {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub poll_interval_ms: u64,
    pub driver: DriverInDeviceConfig,
    pub thing_model: ThingModelConfig,
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverInDeviceConfig {
    pub driver_name: String,
    pub driver_type: String,
    pub poll_interval_ms: u64,
    pub zmq: ZmqDriverConfig,
    pub logging: LoggingConfig,
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZmqDriverConfig {
    pub enabled: bool,
    pub publisher_address: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingModelConfig {
    pub model_id: String,
    pub model_version: String,
    pub device_type: String,
    #[serde(default)]
    pub manufacturer: String,
    pub description: String,
    pub properties: Vec<PropertyConfig>,
    #[serde(default)]
    pub events: Vec<EventConfig>,
    #[serde(default)]
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyConfig {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub unit: Option<String>,
    pub access: String,
    #[serde(default)]
    pub range: Option<Vec<f64>>,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub modbus: Option<ModbusPropertyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusPropertyConfig {
    pub primary_table: String,
    pub starting_address: u16,
    pub raw_type: String,
    pub scale: Option<f64>,
    pub offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    pub identifier: String,
    pub name: String,
    pub level: String,
    #[serde(default)]
    pub output_params: Vec<EventOutputParamConfig>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOutputParamConfig {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub identifier: String,
    pub name: String,
    pub conditions: Vec<RuleConditionConfig>,
    #[serde(default = "default_condition_logic")]
    pub condition_logic: String,
    pub actions: Vec<RuleActionConfig>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_condition_logic() -> String {
    "and".to_string()
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionConfig {
    pub property_identifier: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleActionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<TriggerEventAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerEventAction {
    pub event_identifier: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub input_params: Vec<ParameterConfig>,
    #[serde(default)]
    pub output_params: Vec<ParameterConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConfig {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub unit: Option<String>,
    pub required: Option<bool>,
    pub description: Option<String>,
    pub default_value: Option<serde_json::Value>,
}

impl DeviceGroupConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DeviceGroupConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}
