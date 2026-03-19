use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventLevel {
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParam {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl EventParam {
    pub fn new(identifier: &str, name: &str, data_type: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            data_type: data_type.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub identifier: String,
    pub name: String,
    pub level: EventLevel,
    #[serde(default)]
    pub output_params: Vec<EventParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, serde_json::Value>,
}

impl Event {
    pub fn new(identifier: &str, name: &str, level: EventLevel) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            level,
            output_params: Vec::new(),
            description: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_output_param(mut self, param: EventParam) -> Self {
        self.output_params.push(param);
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_attribute(mut self, key: &str, value: serde_json::Value) -> Self {
        self.attributes.insert(key.to_string(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub event_id: String,
    pub event_identifier: String,
    pub device_name: String,
    pub level: EventLevel,
    pub data: HashMap<String, serde_json::Value>,
    pub timestamp: i64,
}

impl EventData {
    pub fn new(event_identifier: &str, device_name: &str, level: EventLevel, data: HashMap<String, serde_json::Value>) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_identifier: event_identifier.to_string(),
            device_name: device_name.to_string(),
            level,
            data,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
