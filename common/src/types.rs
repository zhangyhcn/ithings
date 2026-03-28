use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub id: String,
    pub name: String,
    pub value: DataValue,
    pub quality: Quality,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub units: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataValue {
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<DataValue>),
    Object(HashMap<String, DataValue>),
    Null,
}

impl DataValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Float64(v) => Some(*v),
            DataValue::Float32(v) => Some(*v as f64),
            DataValue::Int64(v) => Some(*v as f64),
            DataValue::Int32(v) => Some(*v as f64),
            DataValue::Int16(v) => Some(*v as f64),
            DataValue::Int8(v) => Some(*v as f64),
            DataValue::UInt64(v) => Some(*v as f64),
            DataValue::UInt32(v) => Some(*v as f64),
            DataValue::UInt16(v) => Some(*v as f64),
            DataValue::UInt8(v) => Some(*v as f64),
            DataValue::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
            DataValue::String(v) => v.parse().ok(),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DataValue::Int64(v) => Some(*v),
            DataValue::Int32(v) => Some(*v as i64),
            DataValue::Int16(v) => Some(*v as i64),
            DataValue::Int8(v) => Some(*v as i64),
            DataValue::UInt64(v) => Some(*v as i64),
            DataValue::UInt32(v) => Some(*v as i64),
            DataValue::UInt16(v) => Some(*v as i64),
            DataValue::UInt8(v) => Some(*v as i64),
            DataValue::Bool(v) => Some(if *v { 1 } else { 0 }),
            DataValue::Float64(v) => Some(v.to_bits() as i64),
            DataValue::Float32(v) => Some(v.to_bits() as i64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DataValue::Bool(v) => Some(*v),
            DataValue::Int8(v) => Some(*v != 0),
            DataValue::Int16(v) => Some(*v != 0),
            DataValue::Int32(v) => Some(*v != 0),
            DataValue::Int64(v) => Some(*v != 0),
            DataValue::UInt8(v) => Some(*v != 0),
            DataValue::UInt16(v) => Some(*v != 0),
            DataValue::UInt32(v) => Some(*v != 0),
            DataValue::UInt64(v) => Some(*v != 0),
            _ => None,
        }
    }

    pub fn from_json(value: &serde_json::Value) -> DataValue {
        match value {
            serde_json::Value::Bool(b) => DataValue::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    DataValue::Int64(i)
                } else if let Some(f) = n.as_f64() {
                    DataValue::Float64(f)
                } else {
                    DataValue::Null
                }
            }
            serde_json::Value::String(s) => DataValue::String(s.clone()),
            serde_json::Value::Array(arr) => {
                let data: Vec<DataValue> = arr.iter().map(|v| Self::from_json(v)).collect();
                DataValue::Array(data)
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::from_json(v));
                }
                DataValue::Object(map)
            }
            serde_json::Value::Null => DataValue::Null,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Quality {
    Good,
    Bad,
    Uncertain,
}

impl Default for Quality {
    fn default() -> Self {
        Quality::Good
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DriverStatus {
    Starting,
    Running,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverMetadata {
    pub name: String,
    pub version: String,
    pub driver_type: String,
    pub description: String,
    pub author: String,
    pub tags: Vec<String>,
}

fn default_api_version() -> String {
    "v1".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    #[serde(alias = "apiVersion", alias = "apiversion", default = "default_api_version")]
    pub api_version: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub labels: Vec<String>,
    pub description: Option<String>,
    #[serde(alias = "deviceResources", alias = "deviceresources")]
    pub device_resources: Vec<DeviceResource>,
    #[serde(alias = "deviceCommands", alias = "devicecommands")]
    pub device_commands: Vec<DeviceCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceResource {
    pub name: String,
    #[serde(rename = "isHidden")]
    pub is_hidden: Option<bool>,
    pub description: Option<String>,
    pub attributes: HashMap<String, serde_json::Value>,
    pub properties: ResourceProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename = "value")]
    pub value: ValueProperties,
}

use serde::{self, Deserializer};

fn deserialize_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Value {
        Number(f64),
        String(String),
        Null,
    }

    match Value::deserialize(deserializer)? {
        Value::Number(n) => Ok(Some(n)),
        Value::String(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse().map(Some).map_err(serde::de::Error::custom)
            }
        }
        Value::Null => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueProperties {
    #[serde(rename = "type")]
    pub value_type: ValueType,
    #[serde(rename = "readWrite")]
    pub read_write: ReadWrite,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub scale: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub offset: Option<f64>,
    pub units: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub minimum: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub maximum: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ValueType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    #[serde(alias = "Uint8")]
    UInt8,
    #[serde(alias = "Uint16")]
    UInt16,
    #[serde(alias = "Uint32")]
    UInt32,
    #[serde(alias = "Uint64")]
    UInt64,
    Float32,
    Float64,
    String,
    Binary,
}

impl ValueType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bool" | "boolean" => Some(ValueType::Bool),
            "int8" | "int8_t" => Some(ValueType::Int8),
            "int16" | "int16_t" => Some(ValueType::Int16),
            "int32" | "int32_t" => Some(ValueType::Int32),
            "int64" | "int64_t" => Some(ValueType::Int64),
            "uint8" | "uint8_t" | "byte" => Some(ValueType::UInt8),
            "uint16" | "uint16_t" => Some(ValueType::UInt16),
            "uint32" | "uint32_t" => Some(ValueType::UInt32),
            "uint64" | "uint64_t" => Some(ValueType::UInt64),
            "float32" | "float" => Some(ValueType::Float32),
            "float64" | "double" => Some(ValueType::Float64),
            "string" | "str" => Some(ValueType::String),
            "binary" | "bytes" => Some(ValueType::Binary),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ReadWrite {
    #[serde(alias = "R", alias = "r")]
    R,
    #[serde(alias = "W", alias = "w")]
    W,
    #[serde(alias = "RW", alias = "R/W", alias = "r/w", alias = "rw")]
    RW,
}

impl ReadWrite {
    pub fn can_read(&self) -> bool {
        matches!(self, ReadWrite::R | ReadWrite::RW)
    }

    pub fn can_write(&self) -> bool {
        matches!(self, ReadWrite::W | ReadWrite::RW)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "R" => Some(ReadWrite::R),
            "W" => Some(ReadWrite::W),
            "RW" | "R/W" | "r/w" | "rw" => Some(ReadWrite::RW),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub name: String,
    #[serde(rename = "readWrite")]
    pub read_write: ReadWrite,
    #[serde(rename = "resourceOperations")]
    pub resource_operations: Vec<ResourceOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceOperation {
    #[serde(rename = "deviceResource")]
    pub device_resource: String,
    pub mappings: Option<HashMap<String, String>>,
}

pub struct DataValueConverter;

impl DataValueConverter {
    pub fn convert(raw_value: i64, properties: &ValueProperties) -> DataValue {
        let value = if let Some(scale) = properties.scale {
            raw_value as f64 * scale
        } else {
            raw_value as f64
        };

        let value = if let Some(offset) = properties.offset {
            value + offset
        } else {
            value
        };

        let value = if let (Some(min), Some(max)) = (properties.minimum, properties.maximum) {
            value.max(min).min(max)
        } else {
            value
        };

        match properties.value_type {
            ValueType::Bool => DataValue::Bool(value != 0.0),
            ValueType::Int8 => DataValue::Int8(value as i8),
            ValueType::Int16 => DataValue::Int16(value as i16),
            ValueType::Int32 => DataValue::Int32(value as i32),
            ValueType::Int64 => DataValue::Int64(value as i64),
            ValueType::UInt8 => DataValue::UInt8(value as u8),
            ValueType::UInt16 => DataValue::UInt16(value as u16),
            ValueType::UInt32 => DataValue::UInt32(value as u32),
            ValueType::UInt64 => DataValue::UInt64(value as u64),
            ValueType::Float32 => DataValue::Float32(value as f32),
            ValueType::Float64 => DataValue::Float64(value),
            ValueType::String => DataValue::String(value.to_string()),
            ValueType::Binary => DataValue::Bytes(value.to_string().into_bytes()),
        }
    }

    pub fn convert_from_raw_bool(raw: bool, properties: &ValueProperties) -> DataValue {
        match properties.value_type {
            ValueType::Bool => DataValue::Bool(raw),
            ValueType::Int8 => DataValue::Int8(if raw { 1 } else { 0 }),
            ValueType::Int16 => DataValue::Int16(if raw { 1 } else { 0 }),
            ValueType::Int32 => DataValue::Int32(if raw { 1 } else { 0 }),
            ValueType::Int64 => DataValue::Int64(if raw { 1 } else { 0 }),
            ValueType::UInt8 => DataValue::UInt8(if raw { 1 } else { 0 }),
            ValueType::UInt16 => DataValue::UInt16(if raw { 1 } else { 0 }),
            ValueType::UInt32 => DataValue::UInt32(if raw { 1 } else { 0 }),
            ValueType::UInt64 => DataValue::UInt64(if raw { 1 } else { 0 }),
            ValueType::Float32 => DataValue::Float32(if raw { 1.0 } else { 0.0 }),
            ValueType::Float64 => DataValue::Float64(if raw { 1.0 } else { 0.0 }),
            ValueType::String => DataValue::String(raw.to_string()),
            ValueType::Binary => DataValue::Bytes(vec![if raw { 1 } else { 0 }]),
        }
    }

    pub fn convert_to_raw(value: &DataValue) -> Option<i64> {
        value.as_i64()
    }

    pub fn apply_default(properties: &ValueProperties) -> DataValue {
        if let Some(default) = &properties.default_value {
            match properties.value_type {
                ValueType::Bool => {
                    if let Some(v) = default.as_bool() {
                        return DataValue::Bool(v);
                    }
                }
                ValueType::Int8
                | ValueType::Int16
                | ValueType::Int32
                | ValueType::Int64 => {
                    if let Some(v) = default.as_i64() {
                        return DataValueConverter::convert(v, properties);
                    } else {
                        return DataValue::Null;
                    }
                }
                ValueType::UInt8
                | ValueType::UInt16
                | ValueType::UInt32
                | ValueType::UInt64 => {
                    if let Some(v) = default.as_u64() {
                        return DataValueConverter::convert(v as i64, properties);
                    } else {
                        return DataValue::Null;
                    }
                }
                ValueType::Float32
                | ValueType::Float64 => {
                    if let Some(v) = default.as_f64() {
                        return DataValueConverter::convert(v as i64, properties);
                    } else {
                        return DataValue::Null;
                    }
                }
                ValueType::String => {
                    if let Some(v) = default.as_str() {
                        return DataValue::String(v.to_string());
                    } else {
                        return DataValue::Null;
                    }
                }
                _ => {}
            }
        }
        DataValue::Null
    }
}
