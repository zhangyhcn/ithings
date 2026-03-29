use std::collections::HashMap;
use anyhow::Result;
use super::property::PropertyValue;
use super::service::{ServiceParams, ServiceResult, ServiceHandler};

pub trait DeviceTrait {
    fn device_id(&self) -> &str;
    fn device_name(&self) -> &str;

    fn get_properties(&self) -> HashMap<String, PropertyValue>;
    fn set_properties(&mut self, props: HashMap<String, PropertyValue>) -> Result<()>;

    fn service_registry(&mut self) -> &mut HashMap<String, ServiceHandler>;

    fn register_service(&mut self, service_id: &str, handler: ServiceHandler) {
        self.service_registry().insert(service_id.to_string(), handler);
    }

    fn call_service(&mut self, msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        match self.service_registry().get(service_id) {
            Some(handler) => handler(msg_id, service_id, params),
            None => ServiceResult::not_found(msg_id, service_id),
        }
    }

    fn supported_services(&self) -> Vec<String>;
}

#[derive(Default)]
pub struct BaseDevice {
    pub device_id: String,
    pub device_name: String,
    pub properties: HashMap<String, PropertyValue>,
    pub service_registry: HashMap<String, ServiceHandler>,
}

impl BaseDevice {
    pub fn new(device_id: &str, device_name: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            device_name: device_name.to_string(),
            properties: HashMap::new(),
            service_registry: HashMap::new(),
        }
    }
}

impl DeviceTrait for BaseDevice {
    fn device_id(&self) -> &str {
        &self.device_id
    }

    fn device_name(&self) -> &str {
        &self.device_name
    }

    fn get_properties(&self) -> HashMap<String, PropertyValue> {
        self.properties.clone()
    }

    fn set_properties(&mut self, props: HashMap<String, PropertyValue>) -> Result<()> {
        self.properties.extend(props);
        Ok(())
    }

    fn service_registry(&mut self) -> &mut HashMap<String, ServiceHandler> {
        &mut self.service_registry
    }

    fn supported_services(&self) -> Vec<String> {
        self.service_registry.keys().cloned().collect()
    }
}
