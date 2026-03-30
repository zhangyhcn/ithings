use common::{
    DeviceConfig,
    PropertyValue, ServiceParams, ServiceResult,
    DeviceBuilder,
};
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;

pub struct ExampleDevice {
    config: Option<DeviceConfig>,
    runtime: Option<Arc<common::DeviceRuntime>>,
}

impl Default for ExampleDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl ExampleDevice {
    pub fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }

    pub async fn initialize_with_device_config(&mut self, config: DeviceConfig) -> Result<()> {
        tracing::info!("Initializing example device: {}", config.device_name);
        
        let thing_model = DeviceBuilder::load_thing_model_from_config(&config)?;
        
        let runtime = DeviceBuilder::new(config.clone())
            .with_thing_model(thing_model)
            .with_service("echo", Self::echo_service)
            .with_service("add", Self::add_service)
            .with_service("get_status", Self::get_status_service)
            .build()
            .await?;

        self.runtime = Some(runtime);
        self.config = Some(config);
        
        tracing::info!("Example device initialized successfully");
        Ok(())
    }

    pub fn get_runtime(&self) -> Option<&Arc<common::DeviceRuntime>> {
        self.runtime.as_ref()
    }

    pub fn poll_interval_ms(&self) -> u64 {
        self.config.as_ref().map(|c| c.poll_interval_ms).unwrap_or(1000)
    }

    pub fn echo_service(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("echo service called: msg_id={}", msg_id);
        
        let message = params.params.get("message")
            .and_then(|v| v.value.as_str())
            .unwrap_or("");

        let mut result_data = HashMap::new();
        result_data.insert(
            "echo".to_string(),
            PropertyValue::new("echo", serde_json::Value::String(message.to_string())),
        );
        result_data.insert(
            "timestamp".to_string(),
            PropertyValue::new("timestamp", serde_json::Value::String(chrono::Utc::now().to_rfc3339())),
        );

        ServiceResult::success(msg_id, service_id, result_data)
    }

    pub fn add_service(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("add service called: msg_id={}", msg_id);
        
        let a = params.params.get("a")
            .and_then(|v| v.value.as_f64())
            .unwrap_or(0.0);
        
        let b = params.params.get("b")
            .and_then(|v| v.value.as_f64())
            .unwrap_or(0.0);

        let sum = a + b;

        let mut result_data = HashMap::new();
        result_data.insert(
            "result".to_string(),
            PropertyValue::new("result", serde_json::json!(sum)),
        );
        result_data.insert(
            "a".to_string(),
            PropertyValue::new("a", serde_json::json!(a)),
        );
        result_data.insert(
            "b".to_string(),
            PropertyValue::new("b", serde_json::json!(b)),
        );

        ServiceResult::success(msg_id, service_id, result_data)
    }

    pub fn get_status_service(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("get_status service called: msg_id={}", msg_id);
        
        let _ = params;

        let mut result_data = HashMap::new();
        result_data.insert(
            "status".to_string(),
            PropertyValue::new("status", serde_json::Value::String("online".to_string())),
        );
        result_data.insert(
            "uptime_seconds".to_string(),
            PropertyValue::new("uptime_seconds", serde_json::json!(chrono::Utc::now().timestamp())),
        );
        result_data.insert(
            "version".to_string(),
            PropertyValue::new("version", serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string())),
        );

        ServiceResult::success(msg_id, service_id, result_data)
    }
}
