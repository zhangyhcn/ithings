use common::{
    DeviceConfig,
    PropertyValue, ServiceParams, ServiceResult,
    DeviceBuilder,
};
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;

pub struct MeterDevice {
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
