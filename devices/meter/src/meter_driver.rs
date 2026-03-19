use device_common::{
    DeviceConfig, DataPoint, DriverMetadata, DriverStatus,
    PublisherFactory, SubscriberFactory,
    DriverClientFactory,
    ThingModel, DeviceRuntime,
    Rule, StateMachine,
};
use driver_core::driver::{BaseDriver, Driver};
use driver_core::config::DriverConfig;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MeterDevice {
    base: BaseDriver,
    config: Option<DeviceConfig>,
    runtime: Option<Arc<DeviceRuntime>>,
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
        
        let driver_config = DriverConfig {
            driver_name: config.device_name.clone(),
            driver_type: config.device_type.clone(),
            device_instance_id: config.device_name.clone(),
            poll_interval_ms: config.poll_interval_ms,
            zmq: driver_core::config::ZmqConfig {
                enabled: config.zmq.enabled,
                publisher_address: config.zmq.subscriber_address.clone(),
                topic: config.zmq.write_topic.clone(),
                subscriber_enabled: config.zmq.enabled,
                subscriber_address: config.zmq.subscriber_address.clone(),
                write_topic: config.zmq.write_topic.clone(),
                config_update_topic: config.zmq.config_update_topic.clone(),
                high_water_mark: config.zmq.high_water_mark,
            },
            logging: driver_core::config::LoggingConfig {
                level: config.logging.level.clone(),
                format: config.logging.format.clone(),
            },
            custom: config.custom.clone(),
        };
        
        self.base.initialize(driver_config.clone()).await?;

        tracing::debug!("Initializing publisher (MQTT/Kafka)");
        let mut publisher = PublisherFactory::create(&config)?;
        if let Some(ref mut p) = publisher {
            if let Err(e) = p.connect().await {
                tracing::error!("Failed to connect to publisher: {}", e);
            } else {
                tracing::info!("Connected to {} publisher", p.publisher_type());
            }
        }

        tracing::debug!("Initializing subscriber (ZMQ/Kafka) for service calls");
        let mut subscriber = SubscriberFactory::create(&config)?;
        if let Some(ref mut s) = subscriber {
            if let Err(e) = s.subscribe().await {
                tracing::error!("Failed to subscribe: {}", e);
            } else {
                tracing::info!("Subscribed to {} subscriber", s.subscriber_type());
            }
        }

        tracing::debug!("Initializing driver client (sidecar mode)");
        let driver_client = DriverClientFactory::create(&config)?;

        tracing::debug!("Loading thing model from configuration");
        let thing_model = self.load_thing_model(&config)?;

        tracing::debug!("Validating thing model");
        thing_model.validate().map_err(|e| anyhow::anyhow!(e))?;

        tracing::info!("Thing model loaded: {} v{}", thing_model.model_id, thing_model.model_version);

        let mut runtime = DeviceRuntime::new(thing_model, &config.device_name);

        if let Some(publisher) = publisher {
            runtime = runtime.with_publisher(publisher);
        }

        if let Some(subscriber) = subscriber {
            runtime = runtime.with_subscriber(subscriber);
        }

        if let Some(client) = driver_client {
            runtime = runtime.with_driver_client(client);
        }

        if let Some(rules) = self.load_rules(&config)? {
             tracing::info!("Loaded {} rules", rules.len());
             runtime = runtime.with_rules(rules);
         }

        if let Some(state_machine) = self.load_state_machine(&config)? {
             tracing::info!("State machine configured");
             runtime = runtime.with_state_machine(state_machine);
         }

        let runtime_arc = Arc::new(runtime);
        self.runtime = Some(runtime_arc);
        self.config = Some(config);

        tracing::info!("Electricity meter device initialized with thing model");
        Ok(())
    }

    fn load_thing_model(&self, config: &DeviceConfig) -> Result<ThingModel> {
        if let Some(thing_model_path) = config.custom.get("thing_model_path") {
            if let Some(path) = thing_model_path.as_str() {
                tracing::info!("Loading thing model from file: {}", path);
                return Ok(ThingModel::from_file(path)?);
            }
        }

        if let Some(thing_model_json) = config.custom.get("thing_model") {
            tracing::info!("Loading thing model from custom config JSON");
            let thing_model: ThingModel = serde_json::from_value(thing_model_json.clone())?;
            return Ok(thing_model);
        }

        tracing::warn!("No thing model configured, using default empty model");
        Ok(ThingModel::default())
    }

    fn load_rules(&self, config: &DeviceConfig) -> Result<Option<Vec<Rule>>> {
        if let Some(rules_json) = config.custom.get("rules") {
            let rules: Vec<Rule> = serde_json::from_value(rules_json.clone())?;
            Ok(Some(rules))
        } else {
            Ok(None)
        }
    }

    fn load_state_machine(&self, config: &DeviceConfig) -> Result<Option<StateMachine>> {
        if let Some(sm_json) = config.custom.get("state_machine") {
            let sm: StateMachine = serde_json::from_value(sm_json.clone())?;
            Ok(Some(sm))
        } else {
            Ok(None)
        }
    }

    pub async fn poll_and_process(&self) -> Result<()> {
        if let Some(runtime) = &self.runtime {
            let data_points = runtime.read_properties().await?;

            if !data_points.is_empty() {
                tracing::debug!("Read {} properties from driver", data_points.len());

                for prop_value in data_points {
                    runtime.set_property_value(&prop_value.identifier, prop_value.value.clone()).await?;
                }

                runtime.evaluate_rules().await?;
            }
        }
        Ok(())
    }

    pub fn get_runtime(&self) -> Option<&Arc<DeviceRuntime>> {
        self.runtime.as_ref()
    }

    pub fn poll_interval_ms(&self) -> u64 {
        self.config.as_ref().map(|c| c.poll_interval_ms).unwrap_or(1000)
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
            zmq: device_common::config::ZmqConfig {
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
            mqtt: device_common::config::MqttConfig::default(),
            kafka: device_common::config::KafkaConfig::default(),
            driver: device_common::config::DriverClientConfig::default(),
            logging: device_common::config::LoggingConfig {
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
                let data_value = device_common::types::DataValue::from_json(&prop_value.value);
                data_points.push(DataPoint {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: prop_value.identifier,
                    value: data_value,
                    quality: device_common::types::Quality::Good,
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
