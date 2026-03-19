use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::property::Property;
use super::service::Service;
use super::event::Event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingModel {
    pub model_id: String,
    pub model_version: String,
    pub device_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub services: Vec<Service>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ThingModel {
    pub fn new(model_id: &str, device_type: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            model_version: "1.0".to_string(),
            device_type: device_type.to_string(),
            manufacturer: None,
            description: None,
            properties: Vec::new(),
            services: Vec::new(),
            events: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.model_version = version.to_string();
        self
    }

    pub fn with_manufacturer(mut self, manufacturer: &str) -> Self {
        self.manufacturer = Some(manufacturer.to_string());
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn add_property(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }

    pub fn add_service(mut self, service: Service) -> Self {
        self.services.push(service);
        self
    }

    pub fn add_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    pub fn get_property(&self, identifier: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.identifier == identifier)
    }

    pub fn get_service(&self, identifier: &str) -> Option<&Service> {
        self.services.iter().find(|s| s.identifier == identifier)
    }

    pub fn get_event(&self, identifier: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.identifier == identifier)
    }

    pub fn get_readable_properties(&self) -> Vec<&Property> {
        self.properties.iter().filter(|p| p.can_read()).collect()
    }

    pub fn get_writable_properties(&self) -> Vec<&Property> {
        self.properties.iter().filter(|p| p.can_write()).collect()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let model: ThingModel = serde_json::from_str(&content)?;
        Ok(model)
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut identifiers = std::collections::HashSet::new();
        
        for prop in &self.properties {
            if !identifiers.insert(&prop.identifier) {
                return Err(format!("Duplicate property identifier: {}", prop.identifier));
            }
        }
        
        identifiers.clear();
        for service in &self.services {
            if !identifiers.insert(&service.identifier) {
                return Err(format!("Duplicate service identifier: {}", service.identifier));
            }
        }
        
        identifiers.clear();
        for event in &self.events {
            if !identifiers.insert(&event.identifier) {
                return Err(format!("Duplicate event identifier: {}", event.identifier));
            }
        }
        
        Ok(())
    }
}

impl Default for ThingModel {
    fn default() -> Self {
        Self::new("default.model", "unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::property::PropertyType;
    use super::super::property::PropertyAccess;

    #[test]
    fn test_thing_model_creation() {
        let model = ThingModel::new("sensor.temperature.1.0", "temperature-sensor")
            .with_version("1.0")
            .with_manufacturer("IoT Vendor")
            .add_property(
                Property::new("temperature", "温度", PropertyType::Float, PropertyAccess::R)
                    .with_unit("℃")
                    .with_description("环境温度")
            )
            .add_property(
                Property::new("humidity", "湿度", PropertyType::Float, PropertyAccess::R)
                    .with_unit("%RH")
            );

        assert_eq!(model.properties.len(), 2);
        assert!(model.get_property("temperature").is_some());
    }
}
