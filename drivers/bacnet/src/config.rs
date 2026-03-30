use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type BacnetContext = Arc<Mutex<Option<BacnetClient>>>;

pub fn create_context() -> BacnetContext {
    Arc::new(Mutex::new(None))
}

#[derive(Debug, Clone)]
pub struct BacnetAttributes {
    pub object_type: BacnetObjectType,
    pub object_identifier: u32,
    pub property_id: BacnetPropertyId,
    pub data_type: BacnetDataType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BacnetObjectType {
    #[serde(alias = "analogInput", alias = "AI")]
    AnalogInput,
    #[serde(alias = "analogOutput", alias = "AO")]
    AnalogOutput,
    #[serde(alias = "analogValue", alias = "AV")]
    AnalogValue,
    #[serde(alias = "binaryInput", alias = "BI")]
    BinaryInput,
    #[serde(alias = "binaryOutput", alias = "BO")]
    BinaryOutput,
    #[serde(alias = "binaryValue", alias = "BV")]
    BinaryValue,
    #[serde(alias = "multiStateInput", alias = "MSI")]
    MultiStateInput,
    #[serde(alias = "multiStateOutput", alias = "MSO")]
    MultiStateOutput,
    #[serde(alias = "multiStateValue", alias = "MSV")]
    MultiStateValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BacnetPropertyId {
    #[serde(alias = "presentValue", alias = "PV")]
    PresentValue,
    #[serde(alias = "statusFlags", alias = "SF")]
    StatusFlags,
    #[serde(alias = "description", alias = "DESC")]
    Description,
    #[serde(alias = "units", alias = "UNIT")]
    Units,
    #[serde(alias = "outOfService", alias = "OOS")]
    OutOfService,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BacnetDataType {
    Boolean,
    Unsigned,
    Signed,
    Real,
    Double,
}

#[derive(Debug)]
pub struct BacnetClient {
    host: String,
    port: u16,
    device_id: u32,
}

impl BacnetClient {
    pub fn new(host: String, port: u16, device_id: u32) -> Self {
        Self {
            host,
            port,
            device_id,
        }
    }

    pub async fn connect(&self) -> Result<Self, anyhow::Error> {
        tracing::info!("Connecting to BACnet device at {}:{}", self.host, self.port);
        Ok(Self {
            host: self.host.clone(),
            port: self.port,
            device_id: self.device_id,
        })
    }

    pub async fn read_property(
        &self,
        object_type: BacnetObjectType,
        object_identifier: u32,
        property_id: BacnetPropertyId,
    ) -> Result<BacnetValue, anyhow::Error> {
        tracing::debug!(
            "Reading property: {:?} {} {:?}",
            object_type,
            object_identifier,
            property_id
        );

        Ok(BacnetValue::Null)
    }

    pub async fn write_property(
        &self,
        object_type: BacnetObjectType,
        object_identifier: u32,
        property_id: BacnetPropertyId,
        value: BacnetValue,
    ) -> Result<(), anyhow::Error> {
        tracing::debug!(
            "Writing property: {:?} {} {:?} = {:?}",
            object_type,
            object_identifier,
            property_id,
            value
        );
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BacnetValue {
    Null,
    Boolean(bool),
    Unsigned(u32),
    Signed(i32),
    Real(f32),
    Double(f64),
}

pub struct BacnetAttributeParser;

impl BacnetAttributeParser {
    pub fn parse(attributes: &HashMap<String, serde_json::Value>) 
        -> Result<BacnetAttributes, String> {
        let object_type = Self::parse_object_type(
            attributes.get("objectType")
                .or_else(|| attributes.get("object_type"))
        )?;

        let object_identifier = Self::parse_identifier(
            attributes.get("objectIdentifier")
                .or_else(|| attributes.get("object_identifier"))
        )?;

        let property_id = Self::parse_property_id(
            attributes.get("propertyId")
                .or_else(|| attributes.get("property_id"))
                .or_else(|| attributes.get("propertyId"))
        )?;

        let data_type = Self::parse_data_type(
            attributes.get("dataType")
                .or_else(|| attributes.get("data_type"))
        )?;

        Ok(BacnetAttributes {
            object_type,
            object_identifier,
            property_id,
            data_type,
        })
    }

    fn parse_object_type(value: Option<&serde_json::Value>) -> Result<BacnetObjectType, String> {
        let type_str = value
            .and_then(|v| v.as_str())
            .ok_or("Missing objectType attribute".to_string())?;

        serde_json::from_str::<BacnetObjectType>(&format!("\"{}\"", type_str))
            .map_err(|e| format!("Unknown object type '{}': {}", type_str, e))
    }

    fn parse_identifier(value: Option<&serde_json::Value>) -> Result<u32, String> {
        value
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .ok_or_else(|| "Missing or invalid objectIdentifier attribute".to_string())
    }

    fn parse_property_id(value: Option<&serde_json::Value>) -> Result<BacnetPropertyId, String> {
        let prop_str = value
            .and_then(|v| v.as_str())
            .ok_or("Missing propertyId attribute".to_string())?;

        serde_json::from_str::<BacnetPropertyId>(&format!("\"{}\"", prop_str))
            .map_err(|e| format!("Unknown property id '{}': {}", prop_str, e))
    }

    fn parse_data_type(value: Option<&serde_json::Value>) -> Result<BacnetDataType, String> {
        let type_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("real");

        match type_str.to_lowercase().as_str() {
            "bool" | "boolean" => Ok(BacnetDataType::Boolean),
            "uint" | "unsigned" | "unsigned32" => Ok(BacnetDataType::Unsigned),
            "int" | "signed" | "signed32" => Ok(BacnetDataType::Signed),
            "float" | "real" | "float32" => Ok(BacnetDataType::Real),
            "double" | "float64" => Ok(BacnetDataType::Double),
            _ => Err(format!("Unknown data type: {}", type_str)),
        }
    }
}
