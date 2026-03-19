use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    Int,
    Float,
    Bool,
    String,
    Enum,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PropertyAccess {
    R,
    RW,
    W,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyRange {
    pub min: Option<serde_json::Value>,
    pub max: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub identifier: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: PropertyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub access: PropertyAccess,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<PropertyRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, serde_json::Value>,
}

impl Property {
    pub fn new(identifier: &str, name: &str, data_type: PropertyType, access: PropertyAccess) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            data_type,
            unit: None,
            access,
            range: None,
            default_value: None,
            enum_values: None,
            description: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    pub fn with_range(mut self, min: serde_json::Value, max: serde_json::Value) -> Self {
        self.range = Some(PropertyRange {
            min: Some(min),
            max: Some(max),
        });
        self
    }

    pub fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
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

    pub fn can_read(&self) -> bool {
        matches!(self.access, PropertyAccess::R | PropertyAccess::RW)
    }

    pub fn can_write(&self) -> bool {
        matches!(self.access, PropertyAccess::RW | PropertyAccess::W)
    }

    pub fn validate_value(&self, value: &serde_json::Value) -> Result<(), String> {
        if let Some(range) = &self.range {
            if let (Some(min), Some(max)) = (&range.min, &range.max) {
                let valid = match (&self.data_type, value, min, max) {
                    (PropertyType::Int, serde_json::Value::Number(v), serde_json::Value::Number(min_v), serde_json::Value::Number(max_v)) => {
                        if let (Some(v), Some(min_v), Some(max_v)) = (v.as_i64(), min_v.as_i64(), max_v.as_i64()) {
                            v >= min_v && v <= max_v
                        } else {
                            true
                        }
                    }
                    (PropertyType::Float, serde_json::Value::Number(v), serde_json::Value::Number(min_v), serde_json::Value::Number(max_v)) => {
                        if let (Some(v), Some(min_v), Some(max_v)) = (v.as_f64(), min_v.as_f64(), max_v.as_f64()) {
                            v >= min_v && v <= max_v
                        } else {
                            true
                        }
                    }
                    _ => true,
                };
                if !valid {
                    return Err(format!("Value {:?} out of range [{:?}, {:?}]", value, min, max));
                }
            }
        }

        if let Some(enum_vals) = &self.enum_values {
            if !enum_vals.contains(value) {
                return Err(format!("Value {:?} not in enum values {:?}", value, enum_vals));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValue {
    pub identifier: String,
    pub value: serde_json::Value,
    pub timestamp: i64,
    pub quality: String,
}

impl PropertyValue {
    pub fn new(identifier: &str, value: serde_json::Value) -> Self {
        Self {
            identifier: identifier.to_string(),
            value,
            timestamp: chrono::Utc::now().timestamp_millis(),
            quality: "Good".to_string(),
        }
    }

    pub fn with_quality(mut self, quality: &str) -> Self {
        self.quality = quality.to_string();
        self
    }
}
