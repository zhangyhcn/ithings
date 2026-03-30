use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use driver_core::{
    config::DriverConfig,
    driver::{BaseDriver, Driver},
    types::{DataPoint, DataValue, DataValueConverter, DeviceProfile, Quality},
};
use std::collections::HashMap;

use crate::config::{BacnetContext, BacnetValue, BacnetAttributeParser};

#[derive(Debug)]
pub struct BacnetDriver {
    base: BaseDriver,
    profiles: Vec<DeviceProfile>,
    context: BacnetContext,
    host: String,
    port: u16,
    device_id: u32,
}

impl Default for BacnetDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl BacnetDriver {
    pub fn new() -> Self {
        Self {
            base: BaseDriver::new(),
            profiles: Vec::new(),
            context: crate::config::create_context(),
            host: "localhost".to_string(),
            port: 47808,
            device_id: 1,
        }
    }

    pub fn add_profile(&mut self, profile: DeviceProfile) {
        self.profiles.push(profile);
    }

    async fn ensure_connected(&self) -> Result<()> {
        let mut ctx_lock = self.context.lock().await;
        
        if ctx_lock.is_none() {
            tracing::debug!("Connecting to BACnet device at {}:{}", self.host, self.port);
            
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                async {
                    let client = crate::config::BacnetClient::new(
                        self.host.clone(),
                        self.port,
                        self.device_id,
                    ).connect().await?;
                    
                    *ctx_lock = Some(client);
                    tracing::info!("Connected to BACnet device at {}:{}", self.host, self.port);
                    Ok::<_, anyhow::Error>(())
                }
            ).await;

            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => {
                    tracing::error!("Failed to connect to BACnet device: {}", e);
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
            Ok(())
        }
    }

    async fn read_property(
        &self,
        object_type: crate::config::BacnetObjectType,
        object_identifier: u32,
        property_id: crate::config::BacnetPropertyId,
    ) -> Result<BacnetValue> {
        if let Err(e) = self.ensure_connected().await {
            let mut ctx_lock = self.context.lock().await;
            *ctx_lock = None;
            return Err(e);
        }

        let mut ctx_lock = self.context.lock().await;
        let client = ctx_lock.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not connected to BACnet device"))?;

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            client.read_property(object_type, object_identifier, property_id)
        ).await;

        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => {
                tracing::error!("Failed to read property: {}", e);
                *ctx_lock = None;
                Err(e)
            }
            Err(_) => {
                tracing::error!("Read timeout");
                *ctx_lock = None;
                Err(anyhow::anyhow!("Read timeout"))
            }
        }
    }

    async fn write_property(
        &self,
        object_type: crate::config::BacnetObjectType,
        object_identifier: u32,
        property_id: crate::config::BacnetPropertyId,
        value: BacnetValue,
    ) -> Result<()> {
        self.ensure_connected().await?;

        let mut ctx_lock = self.context.lock().await;
        let client = ctx_lock.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not connected to BACnet device"))?;

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            client.write_property(object_type, object_identifier, property_id, value)
        ).await;

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                tracing::error!("Failed to write property: {}", e);
                *ctx_lock = None;
                Err(e)
            }
            Err(_) => {
                tracing::error!("Write timeout");
                *ctx_lock = None;
                Err(anyhow::anyhow!("Write timeout"))
            }
        }
    }
}

fn convert_bacnet_value(
    bacnet_value: &BacnetValue,
    properties: &driver_core::types::ValueProperties,
) -> Result<DataValue> {
    match bacnet_value {
        BacnetValue::Null => Ok(DataValue::Null),
        BacnetValue::Boolean(b) => Ok(DataValueConverter::convert_from_raw_bool(*b, properties)),
        BacnetValue::Unsigned(u) => Ok(DataValueConverter::convert(*u as i64, properties)),
        BacnetValue::Signed(i) => Ok(DataValueConverter::convert(*i as i64, properties)),
        BacnetValue::Real(f) => {
            let bits = f.to_bits();
            let raw = bits as i64;
            Ok(DataValueConverter::convert(raw, properties))
        }
        BacnetValue::Double(d) => {
            let bits = d.to_bits();
            let raw = bits as i64;
            Ok(DataValueConverter::convert(raw, properties))
        }
    }
}

fn data_value_to_bacnet(value: &DataValue, data_type: crate::config::BacnetDataType) -> Result<BacnetValue> {
    match (value, data_type) {
        (DataValue::Null, _) => Ok(BacnetValue::Null),
        (DataValue::Bool(b), crate::config::BacnetDataType::Boolean) => Ok(BacnetValue::Boolean(*b)),
        (DataValue::Int64(i), crate::config::BacnetDataType::Signed) => Ok(BacnetValue::Signed(*i as i32)),
        (DataValue::Int64(i), crate::config::BacnetDataType::Unsigned) => Ok(BacnetValue::Unsigned(*i as u32)),
        (DataValue::Float64(f), crate::config::BacnetDataType::Real) => Ok(BacnetValue::Real(*f as f32)),
        (DataValue::Float64(f), crate::config::BacnetDataType::Double) => Ok(BacnetValue::Double(*f)),
        _ => Err(anyhow::anyhow!("Cannot convert {:?} to {:?}", value, data_type)),
    }
}

unsafe impl Send for BacnetDriver {}
unsafe impl Sync for BacnetDriver {}

#[async_trait]
impl Driver for BacnetDriver {
    fn metadata(&self) -> driver_core::types::DriverMetadata {
        driver_core::types::DriverMetadata {
            name: "bacnet-driver".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: "bacnet".to_string(),
            description: "BACnet protocol driver for building automation".to_string(),
            author: "iThings Team".to_string(),
            tags: vec!["bacnet".to_string(), "building-automation".to_string()],
        }
    }

    async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        tracing::debug!("Initializing BACnet driver");
        
        if let Some(profile_json) = config.custom.get("profile") {
            let profile: DeviceProfile = serde_json::from_value(profile_json.clone())
                .map_err(|e| {
                    tracing::error!("Failed to parse device profile: {}", e);
                    anyhow::anyhow!("Failed to parse device profile: {}", e)
                })?;
            tracing::info!("Profile loaded: {}, {} device resources", profile.name, profile.device_resources.len());
            self.add_profile(profile);
        }
        
        if let Some(profiles_json) = config.custom.get("profiles") {
            if let serde_json::Value::Array(profiles_array) = profiles_json {
                for (i, profile_json) in profiles_array.iter().enumerate() {
                    let profile: DeviceProfile = serde_json::from_value(profile_json.clone())
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
        if let Some(device_id) = config.custom.get("deviceId").and_then(|v: &serde_json::Value| v.as_u64()) {
            self.device_id = device_id as u32;
        }

        tracing::debug!("BACnet config: host={}, port={}, device_id={}, {} profiles loaded", 
            self.host, self.port, self.device_id, self.profiles.len());

        self.base.initialize(config).await?;
        Ok(())
    }

    async fn connect(&mut self) -> Result<()> {
        match self.ensure_connected().await {
            result @ Ok(_) => {
                self.base.set_status(driver_core::types::DriverStatus::Running).await;
                tracing::info!("BACnet driver connected successfully, starting data collection");
                result
            }
            Err(e) => {
                self.base.set_status(driver_core::types::DriverStatus::Error).await;
                Err(e)
            }
        }
    }

    async fn disconnect(&mut self) -> Result<()> {
        let mut ctx_lock = self.context.lock().await;
        *ctx_lock = None;
        self.base.set_status(driver_core::types::DriverStatus::Stopped).await;
        tracing::info!("Disconnected from BACnet device");
        Ok(())
    }

    async fn read(&self) -> Result<Vec<DataPoint>> {
        tracing::info!("read() called, {} profiles loaded", self.profiles.len());

        if self.profiles.is_empty() {
            tracing::warn!("No profiles loaded, returning empty data");
            return Ok(Vec::new());
        }

        let mut data_points = Vec::new();

        for profile in &self.profiles {
            tracing::debug!("Reading device: {}, {} resources", profile.name, profile.device_resources.len());
            
            for resource in &profile.device_resources {
                if !resource.properties.value.read_write.can_read() {
                    continue;
                }

                let attrs = match BacnetAttributeParser::parse(&resource.attributes) {
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

                match self.read_property(attrs.object_type, attrs.object_identifier, attrs.property_id).await {
                    Ok(bacnet_value) => {
                        match convert_bacnet_value(&bacnet_value, &resource.properties.value) {
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
                    Err(e) => {
                        tracing::error!("Failed to read resource {}: {}", resource.name, e);
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

        Ok(data_points)
    }

    async fn write(&self, data_point: &DataPoint) -> Result<()> {
        tracing::info!("Writing data point: {:?}", data_point);

        for profile in &self.profiles {
            if let Some(resource) = profile.device_resources.iter().find(|r| r.name == data_point.name) {
                if !resource.properties.value.read_write.can_write() {
                    return Err(anyhow::anyhow!("Resource {} is not writable", data_point.name));
                }

                let attrs = BacnetAttributeParser::parse(&resource.attributes)
                    .map_err(|e| anyhow::anyhow!("Failed to parse attributes: {}", e))?;

                let bacnet_value = data_value_to_bacnet(&data_point.value, attrs.data_type)
                    .map_err(|e| anyhow::anyhow!("Failed to convert value: {}", e))?;

                self.write_property(attrs.object_type, attrs.object_identifier, attrs.property_id, bacnet_value)
                    .await?;

                tracing::info!("Successfully wrote data point '{}' = {:?}", data_point.name, data_point.value);
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Resource {} not found in any profile", data_point.name))
    }

    async fn status(&self) -> driver_core::types::DriverStatus {
        self.base.status().await
    }
}
