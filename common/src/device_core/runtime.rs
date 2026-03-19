use crate::transport::{RemotePublisher, RemoteSubscriber, DriverClient};
use crate::types::{DataPoint, DataValue, Quality};
use super::thing_model::ThingModel;
use super::property::{Property, PropertyValue};
use super::rule::Rule;
use super::event::EventData;
use super::service::{ServiceRequest, ServiceResponse};
use super::state_machine::StateMachineContext;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub type ServiceHandler = Arc<dyn Fn(ServiceRequest) -> ServiceResponse + Send + Sync>;
pub type EventHandler = Arc<dyn Fn(&EventData) + Send + Sync>;

pub struct DeviceRuntime {
    thing_model: ThingModel,
    device_name: String,
    property_values: Arc<RwLock<HashMap<String, PropertyValue>>>,
    rules: Vec<Rule>,
    state_context: Option<StateMachineContext>,
    publisher: Option<Arc<Mutex<Box<dyn RemotePublisher>>>>,
    subscriber: Option<Arc<Mutex<Box<dyn RemoteSubscriber>>>>,
    driver_client: Option<Arc<Mutex<Box<dyn DriverClient>>>>,
    service_handlers: HashMap<String, ServiceHandler>,
    event_handlers: HashMap<String, Vec<EventHandler>>,
    running: Arc<RwLock<bool>>,
}

impl DeviceRuntime {
    pub fn new(thing_model: ThingModel, device_name: &str) -> Self {
        let mut values = HashMap::new();
        
        for prop in &thing_model.properties {
            if let Some(default) = &prop.default_value {
                values.insert(
                    prop.identifier.clone(),
                    PropertyValue::new(&prop.identifier, (*default).clone()),
                );
            }
        }

        Self {
            thing_model,
            device_name: device_name.to_string(),
            property_values: Arc::new(RwLock::new(values)),
            rules: Vec::new(),
            state_context: None,
            publisher: None,
            subscriber: None,
            driver_client: None,
            service_handlers: HashMap::new(),
            event_handlers: HashMap::new(),
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub fn with_publisher(mut self, publisher: Box<dyn RemotePublisher>) -> Self {
        self.publisher = Some(Arc::new(Mutex::new(publisher)));
        self
    }

    pub fn with_subscriber(mut self, subscriber: Box<dyn RemoteSubscriber>) -> Self {
        self.subscriber = Some(Arc::new(Mutex::new(subscriber)));
        self
    }

    pub fn with_driver_client(mut self, client: Box<dyn DriverClient>) -> Self {
        self.driver_client = Some(Arc::new(Mutex::new(client)));
        self
    }

    pub fn with_rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_state_machine(mut self, state_machine: super::state_machine::StateMachine) -> Self {
        let initial_state = state_machine.get_initial_state()
            .map(|s| s.identifier.clone())
            .unwrap_or_default();
        self.state_context = Some(StateMachineContext::new(state_machine, initial_state));
        self
    }

    pub fn register_service_handler(&mut self, service_identifier: &str, handler: ServiceHandler) {
        self.service_handlers.insert(service_identifier.to_string(), handler);
    }

    pub fn register_event_handler(&mut self, event_identifier: &str, handler: EventHandler) {
        self.event_handlers
            .entry(event_identifier.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub async fn get_property_value(&self, identifier: &str) -> Option<PropertyValue> {
        let values = self.property_values.read().await;
        values.get(identifier).cloned()
    }

    pub async fn set_property_value(&self, identifier: &str, value: serde_json::Value) -> Result<()> {
        let prop = self.thing_model.get_property(identifier);
        if let Some(prop_def) = prop {
            prop_def.validate_value(&value).map_err(|e: String| anyhow::anyhow!(e))?;
        }

        let prop_value = PropertyValue::new(identifier, value);
        {
            let mut values = self.property_values.write().await;
            values.insert(identifier.to_string(), prop_value);
        }

        Ok(())
    }

    pub async fn read_properties(&self) -> Result<Vec<PropertyValue>> {
        let mut results = Vec::new();
        
        if let Some(ref client) = self.driver_client {
            let client_guard = client.lock().await;
            let readable_props: Vec<String> = self.thing_model.properties
                .iter()
                .filter(|p: &&Property| p.can_read())
                .map(|p| p.identifier.clone())
                .collect();

            if !readable_props.is_empty() {
                let data_points = client_guard.read_all(&self.device_name, readable_props).await?;
                
                let mut values = self.property_values.write().await;
                for dp in data_points {
                    let value = serde_json::to_value(&dp.value).unwrap_or(serde_json::Value::Null);
                    let prop_value = PropertyValue::new(&dp.name, value);
                    values.insert(dp.name.clone(), prop_value.clone());
                    results.push(prop_value);
                }
            }
        }

        Ok(results)
    }

    pub async fn evaluate_rules(&self) -> Result<()> {
        let values = self.property_values.read().await;
        let prop_values: HashMap<String, serde_json::Value> = values
            .iter()
            .map(|(k, v)| (k.clone(), v.value.clone()))
            .collect();
        drop(values);

        for rule in &self.rules {
            if rule.evaluate(&prop_values) {
                self.execute_rule_actions(&rule.actions).await?;
            }
        }

        Ok(())
    }

    async fn execute_rule_actions(&self, actions: &[super::rule::RuleAction]) -> Result<()> {
        use super::rule::RuleAction;
        
        for action in actions {
            match action {
                RuleAction::SetProperty { identifier, value } => {
                    self.set_property_value(identifier, value.clone()).await?;
                }
                RuleAction::TriggerEvent { event_identifier, data } => {
                    self.trigger_event(event_identifier, data.clone()).await?;
                }
                RuleAction::CallService { service_identifier, params } => {
                    let request = ServiceRequest::new(service_identifier, params.clone());
                    self.handle_service_request(request).await?;
                }
                RuleAction::Log { level, message } => {
                    match level.as_str() {
                        "debug" => tracing::debug!("{}", message),
                        "info" => tracing::info!("{}", message),
                        "warn" => tracing::warn!("{}", message),
                        "error" => tracing::error!("{}", message),
                        _ => tracing::info!("{}", message),
                    }
                }
            }
        }

        Ok(())
    }

    fn convert_json_to_datavalue(value: &serde_json::Value) -> DataValue {
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
                let data: Vec<DataValue> = arr.iter().map(|v| Self::convert_json_to_datavalue(v)).collect();
                DataValue::Array(data)
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::convert_json_to_datavalue(v));
                }
                DataValue::Object(map)
            }
            serde_json::Value::Null => DataValue::Null,
        }
    }

    pub async fn trigger_event(&self, event_identifier: &str, data: HashMap<String, serde_json::Value>) -> Result<()> {
        let event_def = self.thing_model.get_event(event_identifier);
        let level = event_def.map(|e| e.level.clone()).unwrap_or(super::event::EventLevel::Info);
        
        let event_data = EventData::new(
            event_identifier,
            &self.device_name,
            level,
            data.clone(),
        );

        if let Some(handlers) = self.event_handlers.get(event_identifier) {
            for handler in handlers {
                handler(&event_data);
            }
        }

        if let Some(ref publisher) = self.publisher {
            let pub_guard = publisher.lock().await;
            let event_json = serde_json::to_value(&event_data)?;
            let value = Self::convert_json_to_datavalue(&event_json);
            let data_point = DataPoint {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("event_{}", event_identifier),
                value,
                quality: Quality::Good,
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                units: None,
            };
            pub_guard.publish(&self.device_name, &data_point).await?;
            tracing::info!("Event published: {} for device {}", event_identifier, self.device_name);
        }

        Ok(())
    }

    pub async fn handle_service_request(&self, request: ServiceRequest) -> Result<ServiceResponse> {
        let service = self.thing_model.get_service(&request.service_identifier);
        
        if let Some(service_def) = service {
            let result = service_def.validate_input(&request.params);
            if let Err(e) = result {
                return Ok(ServiceResponse::failure(
                    &request.request_id,
                    &request.service_identifier,
                    &e,
                ));
            }
        }

        if let Some(handler) = self.service_handlers.get(&request.service_identifier) {
            let response = handler(request);
            Ok(response)
        } else {
            Ok(ServiceResponse::failure(
                &request.request_id,
                &request.service_identifier,
                &format!("No handler registered for service: {}", request.service_identifier),
            ))
        }
    }

    pub fn get_thing_model(&self) -> &ThingModel {
        &self.thing_model
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn driver_client(&self) -> Option<&Arc<Mutex<Box<dyn DriverClient>>>> {
        self.driver_client.as_ref()
    }

    pub async fn get_all_property_values(&self) -> HashMap<String, PropertyValue> {
        let values = self.property_values.read().await;
        values.clone()
    }

    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = true;
        tracing::info!("Device runtime started: {}", self.device_name);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Device runtime stopped: {}", self.device_name);
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        let running = self.running.read().await;
        *running
    }
}
