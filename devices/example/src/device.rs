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

pub struct ExampleDevice {
    base: BaseDriver,
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
            base: BaseDriver::new(),
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

#[async_trait]
impl Driver for ExampleDevice {
    fn metadata(&self) -> DriverMetadata {
        DriverMetadata {
            name: "example-device".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: "example".to_string(),
            description: "Example device driver demonstrating DeviceBuilder usage".to_string(),
            author: "iThings Team".to_string(),
            tags: vec!["example".to_string(), "demo".to_string()],
        }
    }

    fn device_name(&self) -> Option<&str> {
        self.config.as_ref().map(|c| c.device_name.as_str())
    }

    async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
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
        if let Some(runtime) = &self.runtime {
            runtime.start().await?;
        }
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(runtime) = &self.runtime {
            runtime.stop().await?;
        }
        Ok(())
    }

    async fn read(&self) -> Result<Vec<DataPoint>> {
        Ok(vec![])
    }

    async fn write(&self, _data_point: &DataPoint) -> Result<()> {
        Ok(())
    }

    async fn status(&self) -> DriverStatus {
        if let Some(runtime) = &self.runtime {
            if runtime.is_running().await {
                return DriverStatus::Running;
            }
        }
        self.base.status().await
    }
}
