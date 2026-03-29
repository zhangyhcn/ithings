use crate::transport::{RemotePublisher, RemoteSubscriber, DriverClient};
use crate::types::{DataPoint, DataValue, Quality, DeviceEvent};
use super::thing_model::ThingModel;
use super::property::{Property, PropertyValue};
use super::rule::Rule;
use super::event::EventData;
use super::service::{ServiceParams, ServiceResult, ServiceHandler, ServiceCallRequest};
use super::state_machine::StateMachineContext;
use super::device_trait::DeviceTrait;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub type EventHandler = Arc<dyn Fn(&EventData) + Send + Sync>;

pub struct DeviceRuntime {
    thing_model: ThingModel,
    device_id: String,
    device_name: String,
    property_values: Arc<RwLock<HashMap<String, PropertyValue>>>,
    rules: Vec<Rule>,
    state_context: Option<StateMachineContext>,
    publisher: Option<Arc<Mutex<Box<dyn RemotePublisher>>>>,
    subscriber: Option<Arc<Mutex<Box<dyn RemoteSubscriber>>>>,
    driver_client: Option<Arc<Mutex<Box<dyn DriverClient>>>>,
    service_registry: HashMap<String, ServiceHandler>,
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
            device_id: uuid::Uuid::new_v4().to_string(),
            device_name: device_name.to_string(),
            property_values: Arc::new(RwLock::new(values)),
            rules: Vec::new(),
            state_context: None,
            publisher: None,
            subscriber: None,
            driver_client: None,
            service_registry: HashMap::new(),
            event_handlers: HashMap::new(),
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub fn with_device_id(mut self, device_id: &str) -> Self {
        self.device_id = device_id.to_string();
        self
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

    pub fn register_service(&mut self, service_id: &str, handler: ServiceHandler) {
        self.service_registry.insert(service_id.to_string(), handler);
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
        
        if let Some(ref subscriber) = self.subscriber {
            let mut sub_guard = subscriber.lock().await;
            
            if let Ok(Some(data_points)) = sub_guard.recv_properties().await {
                let mut values = self.property_values.write().await;
                for dp in data_points {
                    let value = serde_json::to_value(&dp.value).unwrap_or(serde_json::Value::Null);
                    let prop_value = PropertyValue::new(&dp.name, value);
                    values.insert(dp.name.clone(), prop_value.clone());
                    results.push(prop_value);
                }
                tracing::debug!("Updated {} properties in cache", results.len());
            }
        }

        Ok(results)
    }

    pub async fn report_properties(&self) -> Result<()> {
        let values = self.property_values.read().await;
        let properties: Vec<(String, PropertyValue)> = values
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        drop(values);

        if properties.is_empty() {
            return Ok(());
        }

        if let Some(ref publisher) = self.publisher {
            let pub_guard = publisher.lock().await;
            
            for (name, prop_value) in &properties {
                let data_point = DataPoint {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: name.clone(),
                    value: DataValue::from_json(&prop_value.value),
                    quality: Quality::Good,
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::new(),
                    units: None,
                };
                pub_guard.publish(&self.device_name, &data_point).await?;
            }
            tracing::debug!("Reported {} properties to remote", properties.len());
        }

        Ok(())
    }

    pub async fn report_event(&self, event: &DeviceEvent) -> Result<()> {
        if let Some(ref publisher) = self.publisher {
            let pub_guard = publisher.lock().await;
            pub_guard.publish_event(&self.device_name, event).await?;
            tracing::info!("Event reported: {} for device {}", event.name, self.device_name);
        }
        Ok(())
    }

    pub async fn start_processing_loop(self: &Arc<Self>, interval_ms: u64) {
        let running = self.running.clone();
        let runtime = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));
            
            loop {
                let is_running = *running.read().await;
                if !is_running {
                    break;
                }
                
                interval.tick().await;
                
                if let Err(e) = runtime.process_and_report().await {
                    tracing::error!("Processing loop error: {}", e);
                }
            }
        });
    }

    pub async fn process_and_report(&self) -> Result<()> {
        self.evaluate_rules().await?;
        self.report_properties().await?;
        Ok(())
    }

    pub async fn try_recv_properties(&self) -> Result<Vec<PropertyValue>> {
        self.read_properties().await
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
                    let service_params = ServiceParams::from_json(params.clone());
                    self.call_service(&uuid::Uuid::new_v4().to_string(), service_identifier, service_params);
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

    fn convert_property_value_to_datavalue(value: &PropertyValue) -> DataValue {
        Self::convert_json_to_datavalue(&value.value)
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
            
            let params: HashMap<String, DataValue> = data
                .iter()
                .map(|(k, v)| (k.clone(), Self::convert_json_to_datavalue(v)))
                .collect();
            
            let event = DeviceEvent::new(event_identifier, event_identifier, params);
            pub_guard.publish_event(&self.device_name, &event).await?;
            tracing::info!("Event published: {} for device {}", event_identifier, self.device_name);
        }

        Ok(())
    }

    pub fn call_service(&self, msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        let service = self.thing_model.get_service(service_id);
        
        if let Some(service_def) = service {
            let result = service_def.validate_input(&params);
            if let Err(e) = result {
                return ServiceResult::bad_request(msg_id, service_id, &e);
            }
        }

        match self.service_registry.get(service_id) {
            Some(handler) => handler(msg_id, service_id, params),
            None => ServiceResult::not_found(msg_id, service_id),
        }
    }

    pub async fn handle_service_call(&self, request: ServiceCallRequest) -> Result<ServiceResult> {
        let result = self.call_service(&request.msg_id, &request.service_id, ServiceParams { params: request.params });
        
        if let Some(ref publisher) = self.publisher {
            let pub_guard = publisher.lock().await;
            pub_guard.publish_service_reply(&self.device_name, &result).await?;
        }

        Ok(result)
    }

    pub fn get_thing_model(&self) -> &ThingModel {
        &self.thing_model
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }

    pub fn driver_client(&self) -> Option<&Arc<Mutex<Box<dyn DriverClient>>>> {
        self.driver_client.as_ref()
    }

    pub async fn get_all_property_values(&self) -> HashMap<String, PropertyValue> {
        let values = self.property_values.read().await;
        values.clone()
    }

    pub fn supported_services(&self) -> Vec<String> {
        self.service_registry.keys().cloned().collect()
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

impl DeviceTrait for DeviceRuntime {
    fn device_id(&self) -> &str {
        &self.device_id
    }

    fn device_name(&self) -> &str {
        &self.device_name
    }

    fn get_properties(&self) -> HashMap<String, PropertyValue> {
        tokio::task::block_in_place(|| {
            futures::executor::block_on(async {
                self.property_values.read().await.clone()
            })
        })
    }

    fn set_properties(&mut self, props: HashMap<String, PropertyValue>) -> Result<()> {
        tokio::task::block_in_place(|| {
            futures::executor::block_on(async {
                let mut values = self.property_values.write().await;
                values.extend(props);
                Ok(())
            })
        })
    }

    fn service_registry(&mut self) -> &mut HashMap<String, ServiceHandler> {
        &mut self.service_registry
    }

    fn supported_services(&self) -> Vec<String> {
        self.service_registry.keys().cloned().collect()
    }
}
