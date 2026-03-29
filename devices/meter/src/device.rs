use common::{
    DeviceConfig, DataPoint, DriverMetadata, DriverStatus,
    PropertyValue, ServiceParams, ServiceResult,
    DeviceBuilder,
};
use driver_core::driver::{BaseDriver, Driver};
use driver_core::config::DriverConfig;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

pub struct MeterDevice {
    base: BaseDriver,
    config: Option<DeviceConfig>,
    runtime: Option<Arc<common::DeviceRuntime>>,
}

impl Default for MeterDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl MeterDevice {
    pub fn new() -> Self {
        Self {
            base: BaseDriver::new(),
            config: None,
            runtime: None,
        }
    }

    pub async fn initialize_with_device_config(&mut self, config: DeviceConfig) -> Result<()> {
        tracing::info!("Initializing electricity meter device with thing model: {}", config.device_name);
        
        let thing_model = DeviceBuilder::load_thing_model_from_config(&config)?;
        
        let runtime = DeviceBuilder::new(config.clone())
            .with_thing_model(thing_model)
            .with_service("test_write_property", Self::test_write_property)
            .with_service("set_threshold", Self::set_threshold)
            .build()
            .await?;

        self.runtime = Some(runtime);
        self.config = Some(config);

        tracing::info!("Electricity meter device initialized with thing model");
        Ok(())
    }

    pub async fn poll_and_process(&self) -> Result<()> {
        if let Some(runtime) = &self.runtime {
            runtime.read_properties().await?;
        }
        Ok(())
    }

    pub async fn start_processing(&self, interval_ms: u64) -> Result<()> {
        if let Some(runtime) = &self.runtime {
            runtime.start().await?;
            runtime.start_processing_loop(interval_ms).await;
            tracing::info!("Started processing loop with {}ms interval", interval_ms);
        }
        Ok(())
    }

    pub fn get_runtime(&self) -> Option<&Arc<common::DeviceRuntime>> {
        self.runtime.as_ref()
    }

    pub fn poll_interval_ms(&self) -> u64 {
        self.config.as_ref().map(|c| c.poll_interval_ms).unwrap_or(1000)
    }

    pub fn test_write_property(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("test_write_property called: msg_id={}, service_id={}", msg_id, service_id);
        
        let property_name = params.params.get("property_name")
            .and_then(|v| v.value.as_str())
            .unwrap_or("test_property");
        
        let property_value = params.params.get("property_value")
            .map(|v| v.value.clone())
            .unwrap_or(serde_json::Value::Null);

        tracing::info!("Writing property: {} = {:?}", property_name, property_value);

        std::thread::sleep(std::time::Duration::from_millis(100));

        let mut result_data = HashMap::new();
        result_data.insert(
            "property_name".to_string(),
            PropertyValue::new("property_name", serde_json::Value::String(property_name.to_string())),
        );
        result_data.insert(
            "property_value".to_string(),
            PropertyValue::new("property_value", property_value),
        );
        result_data.insert(
            "status".to_string(),
            PropertyValue::new("status", serde_json::Value::String("written".to_string())),
        );
        result_data.insert(
            "timestamp".to_string(),
            PropertyValue::new("timestamp", serde_json::Value::String(chrono::Utc::now().to_rfc3339())),
        );

        tracing::info!("test_write_property completed successfully");
        ServiceResult::success(msg_id, service_id, result_data)
    }

    pub fn set_threshold(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("set_threshold called: msg_id={}, service_id={}", msg_id, service_id);
        
        let power_threshold = params.params.get("power_threshold")
            .and_then(|v| v.value.as_f64())
            .unwrap_or(0.0);

        tracing::info!("Setting power threshold to: {}", power_threshold);

        std::thread::sleep(std::time::Duration::from_millis(100));

        let mut result_data = HashMap::new();
        result_data.insert(
            "power_threshold".to_string(),
            PropertyValue::new("power_threshold", serde_json::Value::Number(serde_json::Number::from_f64(power_threshold).unwrap_or_else(|| serde_json::Number::from(0)))),
        );
        result_data.insert(
            "status".to_string(),
            PropertyValue::new("status", serde_json::Value::String("success".to_string())),
        );
        result_data.insert(
            "timestamp".to_string(),
            PropertyValue::new("timestamp", serde_json::Value::String(chrono::Utc::now().to_rfc3339())),
        );

        tracing::info!("set_threshold completed successfully");
        ServiceResult::success(msg_id, service_id, result_data)
    }
}

#[async_trait]
impl Driver for MeterDevice {
    fn metadata(&self) -> DriverMetadata {
        DriverMetadata {
            name: "meter-device".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: "electricity-meter".to_string(),
            description: "Electricity meter device via Modbus RTU/TCP with thing model support".to_string(),
            author: "iThings Team".to_string(),
            tags: vec!["meter".to_string(), "electricity".to_string(), "modbus".to_string(), "thing-model".to_string()],
        }
    }

    fn device_name(&self) -> Option<&str> {
        self.config.as_ref().map(|c| c.device_name.as_str())
    }

    async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        tracing::info!("Initializing electricity meter device (DriverConfig mode)");
        
        let device_config = DeviceConfig {
            device_name: config.driver_name.clone(),
            device_type: config.driver_type.clone(),
            poll_interval_ms: config.poll_interval_ms,
            zmq: common::config::ZmqConfig {
                enabled: config.zmq.subscriber_enabled,
                publisher_address: config.zmq.publisher_address.clone(),
                topic: config.zmq.topic.clone(),
                subscriber_enabled: config.zmq.subscriber_enabled,
                subscriber_address: config.zmq.subscriber_address.clone(),
                write_topic: config.zmq.write_topic.clone(),
                config_update_topic: config.zmq.config_update_topic.clone(),
                high_water_mark: config.zmq.high_water_mark,
                ..Default::default()
            },
            mqtt: common::config::MqttConfig::default(),
            kafka: common::config::KafkaConfig::default(),
            driver: common::config::DriverClientConfig::default(),
            logging: common::config::LoggingConfig {
                level: config.logging.level.clone(),
                format: config.logging.format.clone(),
            },
            custom: config.custom.clone(),
        };
        
        self.initialize_with_device_config(device_config).await
    }

    async fn connect(&mut self) -> Result<()> {
        tracing::info!("Meter device connected (sidecar mode, starting runtime)");
        if let Some(runtime) = &self.runtime {
            runtime.start().await?;
        }
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        tracing::info!("Meter device disconnected, stopping runtime");
        if let Some(runtime) = &self.runtime {
            runtime.stop().await?;
        }
        Ok(())
    }

    async fn read(&self) -> Result<Vec<DataPoint>> {
        self.poll_and_process().await?;

        if let Some(runtime) = &self.runtime {
            let values = runtime.get_all_property_values().await;
            let mut data_points = Vec::new();

            for (_, prop_value) in values {
                let data_value = common::types::DataValue::from_json(&prop_value.value);
                data_points.push(DataPoint {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: prop_value.identifier,
                    value: data_value,
                    quality: common::types::Quality::Good,
                    timestamp: chrono::Utc::now(),
                    metadata: std::collections::HashMap::new(),
                    units: None,
                });
            }

            Ok(data_points)
        } else {
            Ok(vec![])
        }
    }

    async fn write(&self, data_point: &DataPoint) -> Result<()> {
        tracing::info!("Writing data point: {:?}", data_point);

        if let Some(runtime) = &self.runtime {
            let value = serde_json::to_value(&data_point.value)?;
            runtime.set_property_value(&data_point.name, value).await?;
        }

        Ok(())
    }

    async fn status(&self) -> DriverStatus {
        if let Some(runtime) = &self.runtime {
            if runtime.is_running().await {
                DriverStatus::Running
            } else {
                DriverStatus::Stopped
            }
        } else {
            self.base.status().await
        }
    }
}
