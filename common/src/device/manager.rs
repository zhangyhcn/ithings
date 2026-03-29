use crate::config::{group::DeviceGroupConfig, DeviceInGroupConfig, device::DeviceConfig, driver::DriverConfig};
use crate::device_core::{ThingModel, DeviceRuntime, Rule, ServiceCallRequest, ServiceHandler, ServiceParams, ServiceResult};
use crate::transport::{DriverClientFactory, PublisherFactory, RemotePublisher, RemoteSubscriber};
use crate::transport::mqtt_sub::MqttSubscriber;
use crate::types::{DataPoint, DataValue, Quality};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{interval, Duration};
use zmq::{Context, SUB};

pub type ServiceRegistry = HashMap<String, ServiceHandler>;

pub struct DeviceManager {
    group_config: Option<DeviceGroupConfig>,
    devices: HashMap<String, Arc<DeviceRuntime>>,
    remote_publisher: Option<Arc<Mutex<Box<dyn RemotePublisher>>>>,
    zmq_subscriber: Option<Arc<std::sync::Mutex<zmq::Socket>>>,
    zmq_topic: String,
    zmq_receiver: Option<Arc<Mutex<mpsc::Receiver<(String, Vec<DataPoint>)>>>>,
    mqtt_subscriber: Option<Arc<Mutex<MqttSubscriber>>>,
    service_registry: ServiceRegistry,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            group_config: None,
            devices: HashMap::new(),
            remote_publisher: None,
            zmq_subscriber: None,
            zmq_topic: String::new(),
            zmq_receiver: None,
            mqtt_subscriber: None,
            service_registry: HashMap::new(),
        }
    }

    pub fn register_service(&mut self, service_id: &str, handler: ServiceHandler) {
        self.service_registry.insert(service_id.to_string(), handler);
    }

    pub async fn load_from_file(&mut self, path: &str) -> Result<()> {
        let config = DeviceGroupConfig::from_file(path)?;
        tracing::info!("Loaded device group config: {} devices in tenant {}", config.devices.len(), config.tenant_id);
        self.group_config = Some(config);
        Ok(())
    }

    pub async fn initialize_all(&mut self) -> Result<()> {
        let Some(group_config) = &self.group_config else {
            anyhow::bail!("No device group config loaded");
        };

        // Initialize remote publisher from group config (tenant remote_transport)
        if !group_config.remote_transport.r#type.is_empty() {
            let mut publisher = PublisherFactory::create_from_group_config(group_config)?;
            if publisher.enabled() {
                publisher.connect().await?;
                self.remote_publisher = Some(Arc::new(Mutex::new(publisher)));
                tracing::info!("Remote publisher initialized: {}", group_config.remote_transport.r#type);
            }
        }

        // Initialize ZMQ subscriber to receive data from driver via router
        if let Some(first_device) = group_config.devices.first() {
            if first_device.driver.zmq.enabled {
                let router_address = first_device.driver.zmq.router_address
                    .as_deref()
                    .unwrap_or("tcp://localhost");
                let router_pub_port = first_device.driver.zmq.router_pub_port.unwrap_or(5551);
                let connect_address = format!("{}:{}", router_address, router_pub_port);
                let topic = first_device.driver.zmq.topic.clone();
                
                let context = Context::new();
                let socket = context.socket(SUB)?;
                socket.set_subscribe(b"")?;
                socket.connect(&connect_address)?;
                socket.set_rcvtimeo(100).ok();
                
                let (tx, rx) = mpsc::channel::<(String, Vec<DataPoint>)>(100);
                let socket_arc = Arc::new(std::sync::Mutex::new(socket));
                let topic_clone = topic.clone();
                
                let socket_for_task = Arc::clone(&socket_arc);
                tokio::task::spawn_blocking(move || {
                    loop {
                        let sock = socket_for_task.lock().unwrap();
                        let mut msg = zmq::Message::new();
                        match sock.recv(&mut msg, 0) {
                            Ok(_) => {
                                let content = match msg.as_str() {
                                    Some(s) => s,
                                    None => continue,
                                };
                                let parts: Vec<&str> = content.splitn(2, ' ').collect();
                                if parts.len() != 2 {
                                    continue;
                                }
                                let msg_topic = parts[0];
                                let payload = parts[1];
                                
                                if !msg_topic.starts_with(&topic_clone) {
                                    continue;
                                }
                                
                                let topic_parts: Vec<&str> = msg_topic.split('/').collect();
                                if topic_parts.len() < 3 {
                                    continue;
                                }
                                let device_id = topic_parts[2].to_string();
                                
                                if let Ok(data_point) = serde_json::from_str::<DataPoint>(payload) {
                                    let _ = tx.blocking_send((device_id, vec![data_point]));
                                }
                            }
                            Err(zmq::Error::EAGAIN) => {}
                            Err(_) => break,
                        }
                    }
                });
                
                self.zmq_subscriber = Some(socket_arc);
                self.zmq_topic = topic;
                self.zmq_receiver = Some(Arc::new(Mutex::new(rx)));
                tracing::info!("ZMQ subscriber connected to router {} for topic '{}'", connect_address, self.zmq_topic);
            }
        }

        // Initialize MQTT subscriber for service calls
        if !group_config.remote_transport.r#type.is_empty() 
            && group_config.remote_transport.r#type == "mqtt" {
            let broker_address = group_config.remote_transport.broker.clone()
                .or_else(|| group_config.remote_transport.brokers.clone())
                .unwrap_or_else(|| "tcp://localhost:1883".to_string());
            
            let mqtt_config = crate::config::MqttConfig {
                enabled: true,
                broker_address,
                client_id: format!("{}-subscriber", group_config.tenant_id),
                topic_prefix: String::new(),
                qos: 1,
                username: group_config.remote_transport.username.clone(),
                password: group_config.remote_transport.password.clone(),
                tenant_id: Some(group_config.tenant_id.clone()),
                org_id: Some(group_config.org_id.clone()),
                site_id: Some(group_config.site_id.clone()),
                namespace_id: Some(group_config.namespace_id.clone()),
            };
            
            if let Ok(Some(mut subscriber)) = MqttSubscriber::new(&mqtt_config) {
                subscriber.subscribe().await?;
                self.mqtt_subscriber = Some(Arc::new(Mutex::new(subscriber)));
                tracing::info!("MQTT subscriber initialized for service calls");
            }
        }

        for device_config in &group_config.devices {
            tracing::info!("Initializing device: {} ({})", device_config.device_name, device_config.device_id);
            let device_runtime = self.initialize_device(device_config).await?;
            self.devices.insert(device_config.device_id.clone(), device_runtime);
        }

        // Subscribe to service call topics for all devices
        if let Some(mqtt_sub_arc) = &self.mqtt_subscriber {
            let mqtt_sub = mqtt_sub_arc.lock().await;
            for (device_id, _runtime) in &self.devices {
                if let Err(e) = mqtt_sub.subscribe_service_topic(device_id).await {
                    tracing::warn!("Failed to subscribe service topic for device {}: {}", device_id, e);
                }
            }
        }

        tracing::info!("Initialized {} devices total", self.devices.len());
        Ok(())
    }

    async fn initialize_device(&self, device_config: &DeviceInGroupConfig) -> Result<Arc<DeviceRuntime>> {
        let thing_model = crate::device_core::ThingModel {
            model_id: device_config.thing_model.model_id.clone(),
            model_version: device_config.thing_model.model_version.clone(),
            device_type: device_config.thing_model.device_type.clone(),
            manufacturer: Some(device_config.thing_model.manufacturer.clone()),
            description: Some(device_config.thing_model.description.clone()),
            properties: device_config.thing_model.properties.iter()
                .map(|p| crate::device_core::Property {
                    identifier: p.identifier.clone(),
                    name: p.name.clone(),
                    data_type: match p.type_.as_str() {
                        "int" => crate::device_core::PropertyType::Int,
                        "float" => crate::device_core::PropertyType::Float,
                        "bool" => crate::device_core::PropertyType::Bool,
                        "string" => crate::device_core::PropertyType::String,
                        "enum" => crate::device_core::PropertyType::Enum,
                        "array" => crate::device_core::PropertyType::Array,
                        "object" => crate::device_core::PropertyType::Object,
                        _ => crate::device_core::PropertyType::Float,
                    },
                    unit: p.unit.clone(),
                    access: match p.access.as_str() {
                        "R" => crate::device_core::PropertyAccess::R,
                        "RW" => crate::device_core::PropertyAccess::RW,
                        "W" => crate::device_core::PropertyAccess::W,
                        _ => crate::device_core::PropertyAccess::R,
                    },
                    range: p.range.as_ref().map(|r| crate::device_core::PropertyRange {
                        min: r.first().map(|v| serde_json::Value::from(*v)),
                        max: r.last().map(|v| serde_json::Value::from(*v)),
                    }),
                    default_value: p.default_value.clone(),
                    description: p.description.clone(),
                    enum_values: None,
                    attributes: Default::default(),
                })
                .collect(),
            services: device_config.thing_model.services.iter()
                .map(|c| crate::device_core::Service {
                    identifier: c.identifier.clone(),
                    name: c.name.clone(),
                    description: c.description.clone(),
                    input_params: c.input_params.iter()
                        .map(|p| crate::device_core::ServiceParam {
                            identifier: p.identifier.clone(),
                            name: p.name.clone(),
                            data_type: p.type_.clone(),
                            required: Some(p.required.unwrap_or(false)),
                            description: p.description.clone(),
                            default_value: p.default_value.clone(),
                        })
                        .collect(),
                    output_params: c.output_params.iter()
                        .map(|p| crate::device_core::ServiceOutput {
                            identifier: p.identifier.clone(),
                            name: p.name.clone(),
                            data_type: p.type_.clone(),
                            description: p.description.clone(),
                        })
                        .collect(),
                    call_type: Default::default(),
                    attributes: Default::default(),
                })
                .collect(),
            events: device_config.thing_model.events.iter()
                .map(|e| crate::device_core::Event {
                    identifier: e.identifier.clone(),
                    name: e.name.clone(),
                    level: match e.level.to_lowercase().as_str() {
                        "info" => crate::device_core::EventLevel::Info,
                        "warn" => crate::device_core::EventLevel::Warn,
                        "error" => crate::device_core::EventLevel::Error,
                        "fatal" => crate::device_core::EventLevel::Fatal,
                        _ => crate::device_core::EventLevel::Info,
                    },
                    output_params: e.output_params.iter()
                        .map(|op| crate::device_core::EventParam {
                            identifier: op.identifier.clone(),
                            name: op.name.clone(),
                            data_type: op.type_.clone(),
                            description: None,
                        })
                        .collect(),
                    description: e.description.clone(),
                    attributes: Default::default(),
                })
                .collect(),
            metadata: Default::default(),
        };

        let group_config = self.group_config.as_ref().expect("Group config should be loaded");
        
        let driver_client_config = crate::config::DriverClientConfig {
            enabled: true,
            server_address: String::new(),
            router_address: group_config.devices.first()
                .and_then(|d| d.driver.zmq.router_address.clone()),
            router_sub_port: group_config.devices.first()
                .and_then(|d| d.driver.zmq.router_sub_port),
        };

        let device_config_struct = DeviceConfig {
            device_name: device_config.device_name.clone(),
            device_type: device_config.device_type.clone(),
            poll_interval_ms: device_config.poll_interval_ms,
            zmq: Default::default(),
            mqtt: Default::default(),
            kafka: Default::default(),
            driver: driver_client_config.clone(),
            logging: crate::config::LoggingConfig {
                level: device_config.driver.logging.level.clone(),
                format: device_config.driver.logging.format.clone(),
            },
            custom: HashMap::new(),
        };

        let mut runtime = DeviceRuntime::new(
            thing_model,
            &device_config.device_name,
        );
        
        runtime = runtime.with_device_id(&device_config.device_id);

        if let Some(client) = DriverClientFactory::create(&device_config_struct)? {
            runtime = runtime.with_driver_client(client);
        }

        let rules: Vec<crate::device_core::Rule> = device_config.rules.iter()
            .map(|r| crate::device_core::Rule {
                identifier: r.identifier.clone(),
                name: r.name.clone(),
                conditions: r.conditions.iter()
                    .map(|c| crate::device_core::RuleCondition {
                        property_identifier: c.property_identifier.clone(),
                        operator: match c.operator.as_str() {
                            "==" => crate::device_core::ConditionOperator::Equal,
                            "!=" => crate::device_core::ConditionOperator::NotEqual,
                            ">" => crate::device_core::ConditionOperator::GreaterThan,
                            ">=" => crate::device_core::ConditionOperator::GreaterThanOrEqual,
                            "<" => crate::device_core::ConditionOperator::LessThan,
                            "<=" => crate::device_core::ConditionOperator::LessThanOrEqual,
                            "contains" => crate::device_core::ConditionOperator::Contains,
                            "matches" => crate::device_core::ConditionOperator::Matches,
                            _ => crate::device_core::ConditionOperator::Equal,
                        },
                        value: c.value.clone(),
                    })
                    .collect(),
                logic: match r.condition_logic.as_str() {
                    "and" => crate::device_core::ConditionLogic::And,
                    "or" => crate::device_core::ConditionLogic::Or,
                    _ => crate::device_core::ConditionLogic::And,
                },
                actions: r.actions.iter()
                    .map(|a| {
                        use crate::device_core::RuleAction;
                        if let Some(trigger) = &a.trigger_event {
                            RuleAction::TriggerEvent {
                                event_identifier: trigger.event_identifier.clone(),
                                data: trigger.data.iter()
                                    .map(|(k, v): (&String, &String)| (k.clone(), serde_json::Value::String(v.clone())))
                                    .collect(),
                            }
                        } else {
                            RuleAction::Log {
                                level: "info".to_string(),
                                message: "".to_string(),
                            }
                        }
                    })
                    .collect(),
                enabled: r.enabled,
            })
            .collect();

        runtime = runtime.with_rules(rules);

        for (service_id, handler) in &self.service_registry {
            runtime.register_service(service_id, *handler);
        }

        if let Some(ref publisher) = self.remote_publisher {
            runtime = runtime.with_publisher_arc(Arc::clone(publisher));
        }

        let runtime = Arc::new(runtime);

        Ok(runtime)
    }

    pub async fn send_driver_config(&self) -> Result<()> {
        let Some(group_config) = &self.group_config else {
            anyhow::bail!("No device group config loaded");
        };

        for device_config in &group_config.devices {
            self.send_single_driver_config(device_config).await?;
        }

        Ok(())
    }

    async fn send_single_driver_config(&self, device_config: &DeviceInGroupConfig) -> Result<()> {
        let driver_config = crate::config::driver::DriverConfig {
            driver_name: device_config.driver.driver_name.clone(),
            driver_type: device_config.driver.driver_type.clone(),
            device_instance_id: device_config.device_id.clone(),
            poll_interval_ms: device_config.driver.poll_interval_ms,
            zmq: crate::config::driver::ZmqConfig {
                enabled: device_config.driver.zmq.enabled,
                publisher_address: device_config.driver.zmq.publisher_address.clone(),
                topic: device_config.driver.zmq.topic.clone(),
                ..Default::default()
            },
            logging: crate::config::driver::LoggingConfig {
                level: device_config.driver.logging.level.clone(),
                format: device_config.driver.logging.format.clone(),
            },
            custom: device_config.driver.custom.clone(),
        };

        for (device_id, runtime) in &self.devices {
            if device_id == &device_config.device_id {
                if let Some(driver_client) = runtime.driver_client() {
                    tracing::info!("Sending driver config to {} for device {}", 
                        driver_config.driver_name, device_config.device_id);
                    let mut client = driver_client.lock().await;
                    client.send_config(&driver_config).await?;
                }
                break;
            }
        }

        Ok(())
    }

    pub fn get_devices(&self) -> &HashMap<String, Arc<DeviceRuntime>> {
        &self.devices
    }

    pub fn get_device(&self, device_id: &str) -> Option<&Arc<DeviceRuntime>> {
        self.devices.get(device_id)
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    pub async fn start_reporting_loop(&self, report_interval_ms: u64) {
        let Some(remote_publisher_arc) = &self.remote_publisher else {
            tracing::info!("No remote publisher configured, reporting loop will not start");
            return;
        };

        let mut ticker = interval(Duration::from_millis(report_interval_ms));
        tracing::info!("Starting remote reporting loop with interval: {}ms", report_interval_ms);

        let mqtt_enabled = self.mqtt_subscriber.is_some();

        loop {
            tokio::select! {
                biased;
                
                _ = ticker.tick() => {
                    let remote_publisher = remote_publisher_arc.lock().await;

                    for (device_id, runtime) in &self.devices {
                        match self.collect_device_data(runtime, &remote_publisher).await {
                            Ok(()) => {},
                            Err(e) => {
                                tracing::error!("Failed to report data for device {}: {}", device_id, e);
                            }
                        }
                    }
                }
                
                data_result = async {
                    if let Some(rx_arc) = &self.zmq_receiver {
                        let mut rx = rx_arc.lock().await;
                        rx.recv().await
                    } else {
                        std::future::pending().await
                    }
                } => {
                    if let Some((device_id, data_points)) = data_result {
                        if let Some(runtime) = self.devices.get(&device_id) {
                            for dp in &data_points {
                                let json_value = self.datavalue_to_json(&dp.value);
                                if let Err(e) = runtime.set_property_value(&dp.name, json_value).await {
                                    tracing::warn!("Failed to set property {}: {}", dp.name, e);
                                }
                            }
                            tracing::info!("Updated {} properties for device {} via ZMQ", data_points.len(), device_id);
                        }
                    }
                }
                
                service_result = async {
                    if !mqtt_enabled {
                        std::future::pending().await
                    } else {
                        self.receive_service_call().await
                    }
                } => {
                    if let Ok(Some(request)) = service_result {
                        tracing::info!("Received service call: {} for device {}", request.service_id, request.msg_id);
                        for (_device_id, runtime) in &self.devices {
                            if let Ok(result) = runtime.handle_service_call(request.clone()).await {
                                tracing::info!("Service call result: code={}, msg_id={}", result.code, result.msg_id);
                            }
                        }
                    }
                }
                
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Shutdown signal received in reporting loop");
                    return;
                }
            }
        }
    }

    async fn receive_service_call(&self) -> Result<Option<ServiceCallRequest>> {
        let Some(mqtt_sub_arc) = &self.mqtt_subscriber else {
            return Ok(None);
        };

        let mqtt_sub = mqtt_sub_arc.lock().await;
        mqtt_sub.recv_service_call().await
    }

    async fn collect_device_data(&self, runtime: &Arc<DeviceRuntime>, publisher: &tokio::sync::MutexGuard<'_, Box<dyn RemotePublisher>>) -> Result<()> {
        let values = runtime.get_all_property_values().await;
        let mut data_points: Vec<DataPoint> = Vec::new();

        for (prop_id, prop_value) in values {
            let data_value = match prop_value.value {
                serde_json::Value::Bool(b) => DataValue::Bool(b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        DataValue::Int64(i)
                    } else if let Some(f) = n.as_f64() {
                        DataValue::Float64(f)
                    } else {
                        DataValue::Null
                    }
                }
                serde_json::Value::String(s) => DataValue::String(s),
                serde_json::Value::Array(arr) => {
                    let data: Vec<DataValue> = arr.iter().map(|v| self.json_to_datavalue(v)).collect();
                    DataValue::Array(data)
                }
                serde_json::Value::Object(obj) => {
                    let mut map = HashMap::new();
                    for (k, v) in obj {
                        map.insert(k.clone(), self.json_to_datavalue(&v));
                    }
                    DataValue::Object(map)
                 }
                serde_json::Value::Null => DataValue::Null,
            };

            data_points.push(DataPoint {
                id: uuid::Uuid::new_v4().to_string(),
                name: prop_id.clone(),
                value: data_value,
                quality: Quality::Good,
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                units: None,
            });
        }

        if !data_points.is_empty() {
            publisher.as_ref().publish_batch(runtime.get_device_id(), &data_points).await?;
            tracing::debug!("Reported {} data points for device {}",
                data_points.len(), runtime.get_device_id());
        }

        Ok(())
    }

    fn json_to_datavalue(&self, value: &serde_json::Value) -> DataValue {
        match value {
            serde_json::Value::Bool(b) => DataValue::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    DataValue::Int64(i)
                } else if let Some(f) = n.as_f64() {
                    DataValue::Float64(f)
                } else {
                    DataValue::Null
                }
            }
            serde_json::Value::String(s) => DataValue::String(s.clone()),
            serde_json::Value::Array(arr) => {
                let data: Vec<DataValue> = arr.iter().map(|v| self.json_to_datavalue(v)).collect();
                DataValue::Array(data)
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), self.json_to_datavalue(v));
                }
                DataValue::Object(map)
            }
            serde_json::Value::Null => DataValue::Null,
        }
    }

    fn datavalue_to_json(&self, value: &DataValue) -> serde_json::Value {
        match value {
            DataValue::Bool(b) => serde_json::Value::Bool(*b),
            DataValue::Int8(i) => serde_json::Value::Number((*i).into()),
            DataValue::Int16(i) => serde_json::Value::Number((*i).into()),
            DataValue::Int32(i) => serde_json::Value::Number((*i).into()),
            DataValue::Int64(i) => serde_json::Value::Number((*i).into()),
            DataValue::UInt8(i) => serde_json::Value::Number((*i).into()),
            DataValue::UInt16(i) => serde_json::Value::Number((*i).into()),
            DataValue::UInt32(i) => serde_json::Value::Number((*i).into()),
            DataValue::UInt64(i) => serde_json::Value::Number((*i).into()),
            DataValue::Float32(f) => serde_json::Number::from_f64(*f as f64)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            DataValue::Float64(f) => serde_json::Number::from_f64(*f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            DataValue::String(s) => serde_json::Value::String(s.clone()),
            DataValue::Array(arr) => serde_json::Value::Array(arr.iter().map(|v| self.datavalue_to_json(v)).collect()),
            DataValue::Object(map) => serde_json::Value::Object(map.iter().map(|(k, v)| (k.clone(), self.datavalue_to_json(v))).collect()),
            DataValue::Bytes(b) => serde_json::Value::String(b.iter().map(|byte| format!("{:02x}", byte)).collect()),
            DataValue::Null => serde_json::Value::Null,
        }
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}
