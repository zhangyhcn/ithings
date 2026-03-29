use anyhow::Result;
use std::sync::Arc;

use crate::config::DeviceConfig;
use crate::transport::{PublisherFactory, SubscriberFactory, DriverClientFactory};
use crate::transport::zmq_sub::ZmqSubscriber;

use super::thing_model::ThingModel;
use super::rule::Rule;
use super::state_machine::StateMachine;
use super::service::ServiceHandler;
use super::runtime::DeviceRuntime;

pub struct DeviceBuilder {
    config: DeviceConfig,
    thing_model: Option<ThingModel>,
    services: Vec<(String, ServiceHandler)>,
    rules: Option<Vec<Rule>>,
    state_machine: Option<StateMachine>,
    device_id: Option<String>,
}

impl DeviceBuilder {
    pub fn new(config: DeviceConfig) -> Self {
        Self {
            config,
            thing_model: None,
            services: Vec::new(),
            rules: None,
            state_machine: None,
            device_id: None,
        }
    }

    pub fn with_thing_model(mut self, model: ThingModel) -> Self {
        self.thing_model = Some(model);
        self
    }

    pub fn with_thing_model_from_file(mut self, path: &str) -> Result<Self> {
        let model = ThingModel::from_file(path)?;
        self.thing_model = Some(model);
        Ok(self)
    }

    pub fn with_thing_model_from_json(mut self, json: serde_json::Value) -> Result<Self> {
        let model: ThingModel = serde_json::from_value(json)?;
        self.thing_model = Some(model);
        Ok(self)
    }

    pub fn with_service(mut self, name: &str, handler: ServiceHandler) -> Self {
        self.services.push((name.to_string(), handler));
        self
    }

    pub fn with_rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = Some(rules);
        self
    }

    pub fn with_state_machine(mut self, state_machine: StateMachine) -> Self {
        self.state_machine = Some(state_machine);
        self
    }

    pub fn with_device_id(mut self, device_id: &str) -> Self {
        self.device_id = Some(device_id.to_string());
        self
    }

    pub async fn build(self) -> Result<Arc<DeviceRuntime>> {
        let thing_model = self.thing_model.unwrap_or_else(|| {
            tracing::warn!("No thing model provided, using default empty model");
            ThingModel::default()
        });

        thing_model.validate().map_err(|e| anyhow::anyhow!(e))?;
        tracing::info!("Thing model loaded: {} v{}", thing_model.model_id, thing_model.model_version);

        let publisher = Self::create_publisher(&self.config).await?;
        let subscriber = Self::create_subscriber(&self.config).await?;
        let internal_subscriber = Self::create_internal_subscriber(&self.config)?;
        let driver_client = Self::create_driver_client(&self.config)?;
        let rules = Self::load_rules(&self.config)?;
        let state_machine = Self::load_state_machine(&self.config)?;

        let mut runtime = DeviceRuntime::new(thing_model, &self.config.device_name);

        if let Some(device_id) = self.device_id {
            runtime = runtime.with_device_id(&device_id);
        }

        if let Some(publisher) = publisher {
            runtime = runtime.with_publisher(publisher);
        }

        if let Some(subscriber) = subscriber {
            runtime = runtime.with_subscriber(subscriber);
        }

        if let Some(internal_sub) = internal_subscriber {
            runtime = runtime.with_internal_subscriber(internal_sub);
        }

        if let Some(client) = driver_client {
            runtime = runtime.with_driver_client(client);
        }

        if let Some(rules) = rules {
            tracing::info!("Loaded {} rules", rules.len());
            runtime = runtime.with_rules(rules);
        }

        if let Some(sm) = state_machine {
            tracing::info!("State machine configured");
            runtime = runtime.with_state_machine(sm);
        }

        for (name, handler) in self.services {
            runtime.register_service(&name, handler);
            tracing::debug!("Registered service: {}", name);
        }

        Ok(Arc::new(runtime))
    }

    async fn create_publisher(config: &DeviceConfig) -> Result<Option<Box<dyn crate::transport::RemotePublisher>>> {
        tracing::debug!("Initializing publisher (MQTT/Kafka)");
        let mut publisher = PublisherFactory::create(config)?;
        if let Some(ref mut p) = publisher {
            if let Err(e) = p.connect().await {
                tracing::error!("Failed to connect to publisher: {}", e);
            } else {
                tracing::info!("Connected to {} publisher", p.publisher_type());
            }
        }
        Ok(publisher)
    }

    async fn create_subscriber(config: &DeviceConfig) -> Result<Option<Box<dyn crate::transport::RemoteSubscriber>>> {
        tracing::debug!("Initializing subscriber (ZMQ/Kafka) for service calls");
        let mut subscriber = SubscriberFactory::create(config)?;
        if let Some(ref mut s) = subscriber {
            if let Err(e) = s.subscribe().await {
                tracing::error!("Failed to subscribe: {}", e);
            } else {
                tracing::info!("Subscribed to {} subscriber", s.subscriber_type());
            }
        }
        Ok(subscriber)
    }

    fn create_internal_subscriber(config: &DeviceConfig) -> Result<Option<ZmqSubscriber>> {
        tracing::debug!("Initializing internal ZMQ subscriber for driver properties");
        let subscriber = ZmqSubscriber::new(&crate::config::ZmqConfig {
            enabled: config.zmq.enabled,
            subscriber_address: config.zmq.subscriber_address.clone(),
            write_topic: config.zmq.write_topic.clone(),
            properties_topic: config.zmq.properties_topic.clone(),
            ..Default::default()
        })?;
        Ok(subscriber)
    }

    fn create_driver_client(config: &DeviceConfig) -> Result<Option<Box<dyn crate::transport::DriverClient>>> {
        tracing::debug!("Initializing driver client (sidecar mode)");
        let client = DriverClientFactory::create(config)?;
        Ok(client)
    }

    fn load_rules(config: &DeviceConfig) -> Result<Option<Vec<Rule>>> {
        if let Some(rules_json) = config.custom.get("rules") {
            let rules: Vec<Rule> = serde_json::from_value(rules_json.clone())?;
            Ok(Some(rules))
        } else {
            Ok(None)
        }
    }

    fn load_state_machine(config: &DeviceConfig) -> Result<Option<StateMachine>> {
        if let Some(sm_json) = config.custom.get("state_machine") {
            let sm: StateMachine = serde_json::from_value(sm_json.clone())?;
            Ok(Some(sm))
        } else {
            Ok(None)
        }
    }

    pub fn load_thing_model_from_config(config: &DeviceConfig) -> Result<ThingModel> {
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
}

pub struct DeviceContext {
    pub runtime: Arc<DeviceRuntime>,
    pub config: DeviceConfig,
}

impl DeviceContext {
    pub fn new(runtime: Arc<DeviceRuntime>, config: DeviceConfig) -> Self {
        Self { runtime, config }
    }

    pub fn poll_interval_ms(&self) -> u64 {
        self.config.poll_interval_ms
    }
}
