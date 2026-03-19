use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use driver_core::{
    config::DriverConfig,
    driver::{BaseDriver, Driver},
    types::{
        DataPoint, DataValue, DataValueConverter, DeviceProfile, DeviceResource,
        Quality, ValueProperties,
    },
};
use std::collections::HashMap;
use tokio_modbus::prelude::*;
use tokio_modbus::client::Context as ModbusContext;
use tokio::sync::MutexGuard;

use crate::config;

#[derive(Debug)]
struct BatchRequest<'a> {
    start_address: u16,
    count: u16,
    resources: Vec<(&'a DeviceResource, config::ModbusAttributes)>,
}

fn group_continuous_addresses(
    mut resources: Vec<(&DeviceResource, config::ModbusAttributes)>
) -> Vec<BatchRequest> {
    if resources.is_empty() {
        return Vec::new();
    }

    resources.sort_by_key(|(_, attrs)| attrs.starting_address);

    let mut batches = Vec::new();
    let mut current_batch: Option<(u16, u16, Vec<(&DeviceResource, config::ModbusAttributes)>)> = None;

    for (resource, attrs) in resources.drain(..) {
        let count = config::ModbusAttributeParser::get_register_count(attrs.raw_type);
        let end_address = attrs.starting_address + count - 1;

        match current_batch {
            None => {
                current_batch = Some((attrs.starting_address, end_address, vec![(resource, attrs)]));
            }
            Some((batch_start, batch_end, mut batch_resources)) => {
                if attrs.starting_address == batch_end + 1 {
                    let new_end = end_address;
                    batch_resources.push((resource, attrs));
                    current_batch = Some((batch_start, new_end, batch_resources));
                } else {
                    let count = batch_end - batch_start + 1;
                    batches.push(BatchRequest {
                        start_address: batch_start,
                        count,
                        resources: batch_resources,
                    });
                    current_batch = Some((attrs.starting_address, end_address, vec![(resource, attrs)]));
                }
            }
        }
    }

    if let Some((batch_start, batch_end, batch_resources)) = current_batch {
        let count = batch_end - batch_start + 1;
        batches.push(BatchRequest {
            start_address: batch_start,
            count,
            resources: batch_resources,
        });
    }

    tracing::debug!("Grouped {} resources into {} batches", resources.len(), batches.len());
    batches
}

#[derive(Debug)]
pub struct ModbusDriver {
    base: BaseDriver,
    profiles: Vec<DeviceProfile>,
    context: config::ModbusContext,
    host: String,
    port: u16,
    slave_id: u8,
    connection_mode: config::ConnectionMode,
}

impl ModbusDriver {
    pub fn new() -> Self {
        Self {
            base: BaseDriver::new(),
            profiles: Vec::new(),
            context: config::create_context(),
            host: "localhost".to_string(),
            port: 502,
            slave_id: 1,
            connection_mode: config::ConnectionMode::LongLived,
        }
    }

    pub fn connection_mode(&self) -> config::ConnectionMode {
        self.connection_mode
    }

    pub fn add_profile(&mut self, profile: DeviceProfile) {
        self.profiles.push(profile);
    }

    fn device_name(&self) -> Option<&str> {
        None
    }

    async fn ensure_connected(&self) -> Result<()> {
        let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
        
        if ctx_lock.is_none() {
            tracing::debug!("Connecting to Modbus server at {}:{}", self.host, self.port);
            
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                async {
                    tracing::debug!("Resolving host: {}", self.host);
                    let mut addrs = tokio::net::lookup_host((self.host.as_str(), self.port)).await
                        .map_err(|e| anyhow::anyhow!("Failed to resolve host '{}': {}", self.host, e))?;
                    
                    tracing::debug!("Resolved host, connecting...");
                    let socket_addr = addrs
                        .next()
                        .ok_or_else(|| anyhow::anyhow!("No addresses found for host '{}'", self.host))?;
                    
                    let mut ctx: ModbusContext = tokio_modbus::client::tcp::connect(socket_addr).await
                        .map_err(|e| anyhow::anyhow!("Failed to connect: {}", e))?;
                    
                    let slave = Slave(self.slave_id);
                    ctx.set_slave(slave);
                    
                    *ctx_lock = Some(ctx);
                    tracing::info!("Connected to Modbus server at {}:{} successfully", self.host, self.port);
                    Ok::<_, anyhow::Error>(())
                }
            ).await;

            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => {
                    tracing::error!("Failed to connect to Modbus server: {}", e);
                    *ctx_lock = None;
                    Err(e)
                }
                Err(_) => {
                    tracing::error!("Connection timeout after 10 seconds connecting to {}:{}", self.host, self.port);
                    *ctx_lock = None;
                    Err(anyhow::anyhow!("Connection timeout"))
                }
            }
        } else {
            tracing::debug!("Already connected, reusing existing connection");
            Ok(())
        }
    }

    async fn read_batch(
        &self,
        table: config::ModbusTable,
        start_address: u16,
        count: u16,
    ) -> Result<RawBatchData> {
        if let Err(e) = self.ensure_connected().await {
            let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
            *ctx_lock = None;
            return Err(e);
        }

        let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
        let ctx = ctx_lock.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not connected to Modbus server"))?;

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            read_batch_internal(ctx, table, start_address, count)
        ).await;

        match result {
            Ok(Ok(data)) => Ok(data),
            Ok(Err(e)) => {
                tracing::error!("Failed to read batch: {}", e);
                *ctx_lock = None;
                Err(e)
            }
            Err(_) => {
                tracing::error!("Batch read timeout");
                *ctx_lock = None;
                Err(anyhow::anyhow!("Read timeout"))
            }
        }
    }
}

#[derive(Debug)]
enum RawBatchData {
    Bits(Vec<bool>),
    Registers(Vec<u16>),
}

async fn read_batch_internal(
    ctx: &mut ModbusContext,
    table: config::ModbusTable,
    start_address: u16,
    count: u16,
) -> Result<RawBatchData> {
    let result = match table {
        config::ModbusTable::Coils => {
            let data = ctx.read_coils(start_address, count).await?;
            RawBatchData::Bits(data)
        }
        config::ModbusTable::DiscreteInputs => {
            let data = ctx.read_discrete_inputs(start_address, count).await?;
            RawBatchData::Bits(data)
        }
        config::ModbusTable::InputRegisters => {
            let data = ctx.read_input_registers(start_address, count).await?;
            RawBatchData::Registers(data)
        }
        config::ModbusTable::HoldingRegisters => {
            let data = ctx.read_holding_registers(start_address, count).await?;
            RawBatchData::Registers(data)
        }
    };
    Ok(result)
}

fn convert_raw_data(
    raw_data: &RawBatchData,
    base_address: u16,
    attrs: config::ModbusAttributes,
    properties: &ValueProperties,
) -> Result<DataValue> {
    match raw_data {
        RawBatchData::Bits(bits) => {
            let offset = (attrs.starting_address - base_address) as usize;
            let raw_bool = bits[offset];
            Ok(DataValueConverter::convert_from_raw_bool(raw_bool, properties))
        }
        RawBatchData::Registers(regs) => {
            let offset = (attrs.starting_address - base_address) as usize;
            let count = config::ModbusAttributeParser::get_register_count(attrs.raw_type);
            let slice = &regs[offset..offset + count as usize];
            convert_registers(slice, attrs.raw_type, properties)
        }
    }
}

fn convert_registers(data: &[u16], raw_type: config::ModbusRawType, properties: &ValueProperties) -> Result<DataValue> {
    match raw_type {
        config::ModbusRawType::Bool => {
            let raw = data[0] != 0;
            Ok(DataValueConverter::convert_from_raw_bool(raw, properties))
        }
        config::ModbusRawType::Int16 => {
            let raw = data[0] as i16 as i64;
            Ok(DataValueConverter::convert(raw, properties))
        }
        config::ModbusRawType::UInt16 => {
            let raw = data[0] as i64;
            Ok(DataValueConverter::convert(raw, properties))
        }
        config::ModbusRawType::Int32 => {
            let raw = ((data[1] as i32) << 16) | (data[0] as i32);
            Ok(DataValueConverter::convert(raw as i64, properties))
        }
        config::ModbusRawType::UInt32 => {
            let raw = ((data[1] as u32) << 16) | (data[0] as u32);
            Ok(DataValueConverter::convert(raw as i64, properties))
        }
        config::ModbusRawType::Float32 => {
            let bits = ((data[1] as u32) << 16) | (data[0] as u32);
            let raw = f32::from_bits(bits) as i64;
            Ok(DataValueConverter::convert(raw, properties))
        }
        config::ModbusRawType::Float64 => {
            if data.len() >= 4 {
                let bits = ((data[3] as u64) << 48) 
                         | ((data[2] as u64) << 32) 
                         | ((data[1] as u64) << 16) 
                         | (data[0] as u64);
                let raw = f64::from_bits(bits) as i64;
                Ok(DataValueConverter::convert(raw, properties))
            } else {
                Err(anyhow::anyhow!("Float64 requires 4 registers"))
            }
        }
    }
}

unsafe impl Send for ModbusDriver {}
unsafe impl Sync for ModbusDriver {}

#[async_trait]
impl Driver for ModbusDriver {
    fn metadata(&self) -> driver_core::types::DriverMetadata {
        driver_core::types::DriverMetadata {
            name: "modbus-driver".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: "modbus".to_string(),
            description: "Modbus TCP/RTU driver for industrial devices".to_string(),
            author: "iThings Team".to_string(),
            tags: vec!["modbus".to_string(), "industrial".to_string(), "plc".to_string()],
        }
    }

    async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        tracing::debug!("Initializing Modbus driver with config");
        
        if let Some(profile_json) = config.custom.get("profile") {
            tracing::debug!("Found single profile in config, parsing...");
            let profile: DeviceProfile = serde_json::from_value::<DeviceProfile>(profile_json.clone())
                .map_err(|e| {
                    tracing::error!("Failed to parse device profile. JSON structure: {}", serde_json::to_string_pretty(profile_json).unwrap_or_default());
                    anyhow::anyhow!("Failed to parse device profile: {}", e)
                })?;
            tracing::info!("Profile loaded: {}, {} device resources", profile.name, profile.device_resources.len());
            self.add_profile(profile);
        }
        
        if let Some(profiles_json) = config.custom.get("profiles") {
            if let serde_json::Value::Array(profiles_array) = profiles_json {
                tracing::debug!("Found multiple profiles in config, parsing {} profiles", profiles_array.len());
                for (i, profile_json) in profiles_array.iter().enumerate() {
                    let profile: DeviceProfile = serde_json::from_value::<DeviceProfile>(profile_json.clone())
                        .map_err(|e| {
                            tracing::error!("Failed to parse device profile #{}: {}", i, e);
                            anyhow::anyhow!("Failed to parse device profile #{}: {}", i, e)
                        })?;
                    tracing::info!("Profile #{} loaded: {}, {} device resources", i, profile.name, profile.device_resources.len());
                    self.add_profile(profile);
                }
            }
        }

        if self.profiles.is_empty() {
            tracing::warn!("No profiles found in config");
        }

        if let Some(host) = config.custom.get("host").and_then(|v: &serde_json::Value| v.as_str()) {
            self.host = host.to_string();
        }
        if let Some(port) = config.custom.get("port").and_then(|v: &serde_json::Value| v.as_u64()) {
            self.port = port as u16;
        }
        if let Some(slave_id) = config.custom.get("slave_id").and_then(|v: &serde_json::Value| v.as_u64()) {
            self.slave_id = slave_id as u8;
        }
        if let Some(connection_mode) = config.custom.get("connectionMode").and_then(|v: &serde_json::Value| v.as_str()) {
            match connection_mode.to_lowercase().as_str() {
                "long" | "longlived" => self.connection_mode = config::ConnectionMode::LongLived,
                "short" | "shortlived" => self.connection_mode = config::ConnectionMode::ShortLived,
                _ => {
                    tracing::warn!("Unknown connection mode '{}', using default long-lived connection", connection_mode);
                }
            }
        }

        tracing::debug!("Modbus config: host={}, port={}, slave_id={}, connection_mode={:?}, {} profiles loaded", 
            self.host, self.port, self.slave_id, self.connection_mode, self.profiles.len());

        self.base.initialize(config).await?;
        Ok(())
    }

    async fn connect(&mut self) -> Result<()> {
        match self.ensure_connected().await {
            result @ Ok(_) => {
                self.base.set_status(driver_core::types::DriverStatus::Running).await;
                tracing::info!("Modbus driver connected successfully, starting data collection");
                result
            }
            Err(e) => {
                self.base.set_status(driver_core::types::DriverStatus::Error).await;
                Err(e)
            }
        }
    }

    async fn disconnect(&mut self) -> Result<()> {
        let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
        *ctx_lock = None;
        self.base.set_status(driver_core::types::DriverStatus::Stopped).await;
        tracing::info!("Disconnected from Modbus server");
        Ok(())
    }

    async fn read(&self) -> Result<Vec<DataPoint>> {
        tracing::debug!("read() called, {} profiles loaded", self.profiles.len());

        if self.profiles.is_empty() {
            tracing::warn!("No profiles loaded, returning empty data");
            return Ok(Vec::new());
        }

        if self.connection_mode == config::ConnectionMode::ShortLived {
            let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
            *ctx_lock = None;
        }

        let mut data_points = Vec::new();

        for profile in &self.profiles {
            tracing::debug!("Reading device: {}, {} resources", profile.name, profile.device_resources.len());
            
            let mut grouped_requests: HashMap<config::ModbusTable, Vec<(&DeviceResource, config::ModbusAttributes)>> = HashMap::new();
            
            for resource in &profile.device_resources {
                if !resource.properties.value.read_write.can_read() {
                    continue;
                }

                let attrs = match config::ModbusAttributeParser::parse(&resource.attributes) {
                    Ok(a) => a,
                    Err(e) => {
                        tracing::error!("Failed to parse attributes for resource {}: {}", resource.name, e);
                        data_points.push(DataPoint {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: resource.name.clone(),
                            value: DataValue::Null,
                            quality: Quality::Bad,
                            timestamp: Utc::now(),
                            metadata: HashMap::new(),
                            units: None,
                        });
                        continue;
                    }
                };

                grouped_requests.entry(attrs.primary_table)
                    .or_default()
                    .push((resource, attrs));
            }

            for (table, resources) in grouped_requests {
                let requests = group_continuous_addresses(resources);
                
                for batch in requests {
                    tracing::debug!("Batch read: {} -> table {:?}, start address {}, count {}", 
                        profile.name, table, batch.start_address, batch.count);
                    
                    match self.read_batch(table, batch.start_address, batch.count).await {
                         Ok(raw_data) => {
                             for (resource, attrs) in batch.resources {
                                 match convert_raw_data(&raw_data, batch.start_address, attrs, &resource.properties.value) {
                                     Ok(value) => {
                                         let dp = DataPoint {
                                             id: uuid::Uuid::new_v4().to_string(),
                                             name: resource.name.clone(),
                                             value,
                                             quality: Quality::Good,
                                             timestamp: Utc::now(),
                                             metadata: HashMap::new(),
                                             units: resource.properties.value.units.clone(),
                                         };
                                         tracing::debug!("{} = {:?} [{:?}]", resource.name, dp.value, dp.quality);
                                         data_points.push(dp);
                                     }
                                     Err(e) => {
                                         tracing::error!("Failed to convert resource {}: {}", resource.name, e);
                                         data_points.push(DataPoint {
                                             id: uuid::Uuid::new_v4().to_string(),
                                             name: resource.name.clone(),
                                             value: DataValue::Null,
                                             quality: Quality::Bad,
                                             timestamp: Utc::now(),
                                             metadata: HashMap::new(),
                                             units: None,
                                         });
                                     }
                                 }
                             }
                         }
                         Err(e) => {
                             tracing::error!("Failed batch read on table {:?} start {} count {}: {}", 
                                 table, batch.start_address, batch.count, e);
                             for (resource, _) in batch.resources {
                                 data_points.push(DataPoint {
                                     id: uuid::Uuid::new_v4().to_string(),
                                     name: resource.name.clone(),
                                     value: DataValue::Null,
                                     quality: Quality::Bad,
                                     timestamp: Utc::now(),
                                     metadata: HashMap::new(),
                                     units: None,
                                 });
                             }
                         }
                     }
                }
            }
        }

        if self.connection_mode == config::ConnectionMode::ShortLived {
            let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
            *ctx_lock = None;
            tracing::debug!("Short-lived connection completed, connection closed");
        }

        if let Some(publisher) = self.base.publisher() {
            for profile in &self.profiles {
                let device_data: Vec<DataPoint> = data_points
                    .iter()
                    .filter(|dp| {
                        profile.device_resources.iter()
                            .any(|r| r.name == dp.name)
                    })
                    .cloned()
                    .collect();
                
                if !device_data.is_empty() {
                    if let Err(e) = publisher.publish_batch(&profile.name, &device_data).await {
                        tracing::error!("Failed to publish data for device {}: {}", profile.name, e);
                    }
                }
            }
        }

        let total_resources: usize = self.profiles.iter()
            .map(|p| p.device_resources.len())
            .sum();
        tracing::debug!("Read {} data points from {} devices ({} total resources)", 
            data_points.len(), self.profiles.len(), total_resources);
        Ok(data_points)
    }

    async fn write(&self, data_point: &DataPoint) -> Result<()> {
        self.ensure_connected().await?;

        let mut ctx_lock: MutexGuard<'_, Option<ModbusContext>> = self.context.lock().await;
        let ctx = ctx_lock.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not connected to Modbus server"))?;

        for profile in &self.profiles {
            if let Some(resource) = profile.device_resources.iter()
                .find(|r| r.name == data_point.name)
            {
                let attrs = config::ModbusAttributeParser::parse(&resource.attributes)
                    .map_err(|e| anyhow::anyhow!("Failed to parse attributes: {}", e))?;

                match attrs.primary_table {
                    config::ModbusTable::Coils | config::ModbusTable::HoldingRegisters => {
                        let raw_value = data_point.value.as_bool()
                            .ok_or_else(|| anyhow::anyhow!("Cannot convert to bool for write"))?;
                        
                        if attrs.primary_table == config::ModbusTable::Coils {
                            ctx.write_single_coil(attrs.starting_address, raw_value).await?;
                            tracing::debug!("Wrote coil {} on device {}: {}", resource.name, profile.name, raw_value);
                        } else {
                            let reg_value = if let Some(raw_int) = data_point.value.as_i64() {
                                raw_int as u16
                            } else {
                                return Err(anyhow::anyhow!("Unsupported type for register write"));
                            };
                            ctx.write_single_register(attrs.starting_address, reg_value).await?;
                            tracing::debug!("Wrote register {} on device {}: {}", resource.name, profile.name, reg_value);
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Cannot write to {:?}", attrs.primary_table));
                    }
                }
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Resource not found: {}", data_point.name))
    }

    async fn add_device_profile(&mut self, profile: DeviceProfile) -> Result<()> {
        tracing::info!("Adding device profile: {}, {} device resources", profile.name, profile.device_resources.len());
        self.add_profile(profile);
        Ok(())
    }

    async fn status(&self) -> driver_core::types::DriverStatus {
        self.base.status().await
    }
}

impl Default for ModbusDriver {
    fn default() -> Self {
        Self::new()
    }
}
