use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct ServiceResult {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ServiceResult {
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
    pub output_params: Vec<ServiceResult>,
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

    pub fn with_output_param(mut self, result: ServiceResult) -> Self {
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

    pub fn validate_input(&self, params: &HashMap<String, serde_json::Value>) -> Result<(), String> {
        for input_param in &self.input_params {
            if input_param.required.unwrap_or(true) {
                if !params.contains_key(&input_param.identifier) {
                    return Err(format!("Missing required parameter: {}", input_param.identifier));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub request_id: String,
    pub service_identifier: String,
    pub params: HashMap<String, serde_json::Value>,
    pub timestamp: i64,
}

impl ServiceRequest {
    pub fn new(service_identifier: &str, params: HashMap<String, serde_json::Value>) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            service_identifier: service_identifier.to_string(),
            params,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub request_id: String,
    pub service_identifier: String,
    pub success: bool,
    pub result: HashMap<String, serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: i64,
}

impl ServiceResponse {
    pub fn success(request_id: &str, service_identifier: &str, result: HashMap<String, serde_json::Value>) -> Self {
        Self {
            request_id: request_id.to_string(),
            service_identifier: service_identifier.to_string(),
            success: true,
            result,
            error: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn failure(request_id: &str, service_identifier: &str, error: &str) -> Self {
        Self {
            request_id: request_id.to_string(),
            service_identifier: service_identifier.to_string(),
            success: false,
            result: HashMap::new(),
            error: Some(error.to_string()),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}
