use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type ModbusContext = Arc<Mutex<Option<tokio_modbus::client::Context>>>;

pub fn create_context() -> ModbusContext {
    Arc::new(Mutex::new(None))
}

#[derive(Debug, Clone)]
pub struct ModbusAttributes {
    pub primary_table: ModbusTable,
    pub starting_address: u16,
    pub raw_type: ModbusRawType,
    pub count: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModbusTable {
    Coils,
    DiscreteInputs,
    InputRegisters,
    HoldingRegisters,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModbusRawType {
    Bool,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Float32,
    Float64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConnectionMode {
    #[serde(alias = "long", alias = "LONG")]
    LongLived,
    #[serde(alias = "short", alias = "SHORT")]
    ShortLived,
}

pub struct ModbusAttributeParser;

impl ModbusAttributeParser {
    pub fn parse(attributes: &HashMap<String, serde_json::Value>) -> Result<ModbusAttributes, String> {
        let primary_table = Self::parse_primary_table(
            attributes.get("primaryTable")
                .or_else(|| attributes.get("primary_table"))
        )?;

        let starting_address = Self::parse_address(
            attributes.get("startingAddress")
                .or_else(|| attributes.get("starting_address"))
        )?;

        let raw_type = match primary_table {
            ModbusTable::Coils | ModbusTable::DiscreteInputs => {
                ModbusRawType::Bool
            }
            ModbusTable::InputRegisters | ModbusTable::HoldingRegisters => {
                Self::parse_raw_type(
                    attributes.get("rawType")
                        .or_else(|| attributes.get("raw_type"))
                )?
            }
        };

        let count = attributes.get("count")
            .and_then(|v| v.as_u64())
            .map(|v| v as u16);

        Ok(ModbusAttributes {
            primary_table,
            starting_address,
            raw_type,
            count,
        })
    }

    fn parse_primary_table(value: Option<&serde_json::Value>) -> Result<ModbusTable, String> {
        let table_str = value
            .and_then(|v| v.as_str())
            .ok_or("Missing primaryTable attribute".to_string())?;

        match table_str.to_uppercase().as_str() {
            "COILS" | "COIL" => Ok(ModbusTable::Coils),
            "DISCRETE_INPUTS" | "DISCRETE_INPUT" | "DI" => Ok(ModbusTable::DiscreteInputs),
            "INPUT_REGISTERS" | "INPUT_REGISTER" | "IR" => Ok(ModbusTable::InputRegisters),
            "HOLDING_REGISTERS" | "HOLDING_REGISTER" | "HR" => Ok(ModbusTable::HoldingRegisters),
            _ => Err(format!("Unknown Modbus table: {}", table_str)),
        }
    }

    fn parse_address(value: Option<&serde_json::Value>) -> Result<u16, String> {
        let addr = match value {
            Some(v) if v.is_string() => v.as_str().ok_or("Invalid address".to_string())?,
            Some(v) if v.is_number() => {
                return v.as_u64()
                    .ok_or("Invalid address number".to_string())?
                    .try_into()
                    .map_err(|_| "Address out of range".to_string())
            }
            _ => return Err("Missing startingAddress attribute".to_string()),
        };

        let addr = addr.trim_start_matches("0x");
        u16::from_str_radix(addr, 10)
            .or_else(|_| u16::from_str_radix(addr, 16))
            .map_err(|e| format!("Invalid address '{}': {}", addr, e))
    }

    fn parse_raw_type(value: Option<&serde_json::Value>) -> Result<ModbusRawType, String> {
        let raw_type = value
            .and_then(|v| v.as_str())
            .ok_or("Missing rawType attribute".to_string())?;

        match raw_type.to_uppercase().as_str() {
            "BOOL" | "BOOLEAN" => Ok(ModbusRawType::Bool),
            "INT16" | "INT16_T" | "SHORT" => Ok(ModbusRawType::Int16),
            "UINT16" | "UINT16_T" | "WORD" => Ok(ModbusRawType::UInt16),
            "INT32" | "INT32_T" | "DINT" => Ok(ModbusRawType::Int32),
            "UINT32" | "UINT32_T" | "UDINT" => Ok(ModbusRawType::UInt32),
            "FLOAT32" | "FLOAT" | "REAL" => Ok(ModbusRawType::Float32),
            "FLOAT64" | "DOUBLE" => Ok(ModbusRawType::Float64),
            _ => Err(format!("Unknown raw type: {}", raw_type)),
        }
    }

    pub fn get_register_count(raw_type: ModbusRawType) -> u16 {
        match raw_type {
            ModbusRawType::Bool => 1,
            ModbusRawType::Int16 | ModbusRawType::UInt16 => 1,
            ModbusRawType::Int32 | ModbusRawType::UInt32 | ModbusRawType::Float32 => 2,
            ModbusRawType::Float64 => 4,
        }
    }
}
