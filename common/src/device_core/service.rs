use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::device_core::property::PropertyValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CallType {
    #[default]
    Sync,
    Async,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceParam {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ServiceParam {
    pub fn new(identifier: &str, name: &str, data_type: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            data_type: data_type.to_string(),
            required: Some(true),
            default_value: None,
            description: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOutput {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ServiceOutput {
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
pub struct Service {
    pub identifier: String,
    pub name: String,
    #[serde(default)]
    pub input_params: Vec<ServiceParam>,
    #[serde(default)]
    pub output_params: Vec<ServiceOutput>,
    #[serde(default)]
    pub call_type: CallType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, serde_json::Value>,
}

impl Service {
    pub fn new(identifier: &str, name: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            input_params: Vec::new(),
            output_params: Vec::new(),
            call_type: CallType::Sync,
            description: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_input_param(mut self, param: ServiceParam) -> Self {
        self.input_params.push(param);
        self
    }

    pub fn with_output_param(mut self, result: ServiceOutput) -> Self {
        self.output_params.push(result);
        self
    }

    pub fn with_call_type(mut self, call_type: CallType) -> Self {
        self.call_type = call_type;
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

    pub fn validate_input(&self, params: &ServiceParams) -> Result<(), String> {
        for input_param in &self.input_params {
            if input_param.required.unwrap_or(true) {
                if !params.params.contains_key(&input_param.identifier) {
                    return Err(format!("Missing required parameter: {}", input_param.identifier));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceParams {
    pub params: HashMap<String, PropertyValue>,
}

impl ServiceParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: &str, value: PropertyValue) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn from_json(params: HashMap<String, serde_json::Value>) -> Self {
        let mut result = HashMap::new();
        for (k, v) in params {
            result.insert(k, PropertyValue::from_json_value(&v));
        }
        Self { params: result }
    }
}

impl Default for ServiceParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResult {
    pub msg_id: String,
    pub service_id: String,
    pub code: i32,
    pub message: String,
    pub data: HashMap<String, PropertyValue>,
}

impl ServiceResult {
    pub fn success(msg_id: &str, service_id: &str, data: HashMap<String, PropertyValue>) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            service_id: service_id.to_string(),
            code: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn error(msg_id: &str, service_id: &str, code: i32, message: &str) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            service_id: service_id.to_string(),
            code,
            message: message.to_string(),
            data: HashMap::new(),
        }
    }

    pub fn not_found(msg_id: &str, service_id: &str) -> Self {
        Self::error(msg_id, service_id, 404, "service not found")
    }

    pub fn bad_request(msg_id: &str, service_id: &str, reason: &str) -> Self {
        Self::error(msg_id, service_id, 400, reason)
    }

    pub fn internal_error(msg_id: &str, service_id: &str, reason: &str) -> Self {
        Self::error(msg_id, service_id, 500, reason)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCallRequest {
    pub msg_id: String,
    pub service_id: String,
    pub params: HashMap<String, PropertyValue>,
}

impl ServiceCallRequest {
    pub fn new(service_id: &str, params: HashMap<String, PropertyValue>) -> Self {
        Self {
            msg_id: uuid::Uuid::new_v4().to_string(),
            service_id: service_id.to_string(),
            params,
        }
    }

    pub fn from_json(msg_id: &str, service_id: &str, params: HashMap<String, serde_json::Value>) -> Self {
        let mut result = HashMap::new();
        for (k, v) in params {
            result.insert(k, PropertyValue::from_json_value(&v));
        }
        Self {
            msg_id: msg_id.to_string(),
            service_id: service_id.to_string(),
            params: result,
        }
    }
}

pub type ServiceHandler = fn(&str, &str, ServiceParams) -> ServiceResult;
