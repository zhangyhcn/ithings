use crate::config::{group::DeviceGroupConfig, DeviceInGroupConfig, device::DeviceConfig, driver::DriverConfig};
use crate::device_core::{ThingModel, DeviceRuntime, Rule};
use crate::transport::DriverClientFactory;
use crate::types::{DataPoint, DeviceProfile};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DeviceManager {
    group_config: Option<DeviceGroupConfig>,
    devices: HashMap<String, Arc<DeviceRuntime>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            group_config: None,
            devices: HashMap::new(),
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
                        .map(|p| crate::device_core::ServiceResult {
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

        let driver_client_config = crate::config::DriverClientConfig {
            enabled: true,
            server_address: format!("tcp://{}:{}", 
                device_config.driver.custom.get("host")
                    .and_then(|v| v.as_str())
                    .unwrap_or("127.0.0.1"),
                device_config.driver.custom.get("port")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(502),
            ),
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
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}
