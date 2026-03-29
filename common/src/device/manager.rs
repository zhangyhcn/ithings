use crate::config::{group::DeviceGroupConfig, DeviceInGroupConfig, device::DeviceConfig, driver::DriverConfig};
use crate::device_core::{ThingModel, DeviceRuntime, Rule};
use crate::transport::{DriverClientFactory, PublisherFactory, RemotePublisher};
use crate::types::{DataPoint, DataValue, Quality};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};
use zmq::{Context, SUB};

pub struct DeviceManager {
    group_config: Option<DeviceGroupConfig>,
    devices: HashMap<String, Arc<DeviceRuntime>>,
    remote_publisher: Option<Arc<Mutex<Box<dyn RemotePublisher>>>>,
    zmq_subscriber: Option<Arc<Mutex<zmq::Socket>>>,
    zmq_topic: String,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            group_config: None,
            devices: HashMap::new(),
            remote_publisher: None,
            zmq_subscriber: None,
            zmq_topic: String::new(),
        }
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
                
                self.zmq_subscriber = Some(Arc::new(Mutex::new(socket)));
                self.zmq_topic = topic;
                tracing::info!("ZMQ subscriber connected to router {} for topic '{}'", connect_address, self.zmq_topic);
            }
        }

        for device_config in &group_config.devices {
            tracing::info!("Initializing device: {} ({})", device_config.device_name, device_config.device_id);
            let device_runtime = self.initialize_device(device_config).await?;
            self.devices.insert(device_config.device_id.clone(), device_runtime);
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

    pub async fn start_reporting_loop(&self, report_interval_ms: u64) -> ! {
        let Some(remote_publisher_arc) = &self.remote_publisher else {
            tracing::info!("No remote publisher configured, reporting loop will not start");
            std::future::pending().await
        };

        let mut ticker = interval(Duration::from_millis(report_interval_ms));
        tracing::info!("Starting remote reporting loop with interval: {}ms", report_interval_ms);

        loop {
            tokio::select! {
                biased;
                
                _ = ticker.tick() => {
                    let remote_publisher = remote_publisher_arc.lock().unwrap();

                    for (device_id, runtime) in &self.devices {
                        match self.collect_device_data(runtime, &**remote_publisher).await {
                            Ok(()) => {},
                            Err(e) => {
                                tracing::error!("Failed to report data for device {}: {}", device_id, e);
                            }
                        }
                    }
                }
                
                data_result = self.receive_zmq_data() => {
                    if let Ok(Some((device_id, data_points))) = data_result {
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
            }
        }
    }

    async fn receive_zmq_data(&self) -> Result<Option<(String, Vec<DataPoint>)>> {
        let Some(subscriber_arc) = &self.zmq_subscriber else {
            tokio::time::sleep(Duration::from_millis(100)).await;
            return Ok(None);
        };

        let zmq_topic = self.zmq_topic.clone();
        let subscriber = Arc::clone(subscriber_arc);
        
        let result = tokio::task::spawn_blocking(move || {
            let socket = subscriber.lock().unwrap();
            socket.set_rcvtimeo(10).ok()?;
            
            let mut msg = zmq::Message::new();
            match socket.recv(&mut msg, 0) {
                Ok(_) => {
                    let content = msg.as_str().unwrap_or("");
                    tracing::debug!("ZMQ received raw: {}", &content[..content.len().min(200)]);
                    let parts: Vec<&str> = content.splitn(2, ' ').collect();
                    if parts.len() != 2 {
                        tracing::warn!("ZMQ message format invalid, expected 'topic payload': {}", content.len());
                        return Some(Err(anyhow::anyhow!("Invalid message format")));
                    }
                    
                    let topic = parts[0];
                    let payload = parts[1];
                    
                    if !topic.starts_with(&zmq_topic) {
                        tracing::trace!("ZMQ topic mismatch: {} vs {}", topic, zmq_topic);
                        return None;
                    }
                    
                    let topic_parts: Vec<&str> = topic.split('/').collect();
                    if topic_parts.len() < 3 {
                        tracing::warn!("ZMQ topic format invalid: {}", topic);
                        return None;
                    }
                    let device_id = topic_parts[2].to_string();
                    
                    match serde_json::from_str::<DataPoint>(payload) {
                        Ok(data_point) => {
                            tracing::info!("Received ZMQ data: device={}, {} = {:?}", device_id, data_point.name, data_point.value);
                            Some(Ok((device_id, vec![data_point])))
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse ZMQ payload: {}", e);
                            None
                        }
                    }
                }
                Err(zmq::Error::EAGAIN) => None,
                Err(e) => Some(Err(anyhow::anyhow!("ZMQ recv error: {}", e))),
            }
        }).await?;
        
        result.transpose()
    }

    async fn collect_device_data(&self, runtime: &Arc<DeviceRuntime>, publisher: &dyn RemotePublisher) -> Result<()> {
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
            publisher.publish_batch(runtime.get_device_name(), &data_points).await?;
            tracing::debug!("Reported {} data points for device {}",
                data_points.len(), runtime.get_device_name());
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
