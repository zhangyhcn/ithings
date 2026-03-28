use serde::{Deserialize, Serialize};

use super::group::DeviceInGroupConfig;
use super::driver::DriverConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroupPublishConfig {
    pub tenant_id: String,
    pub org_id: String,
    pub site_id: String,
    pub namespace_id: String,
    pub remote_transport: serde_json::Value,
    pub group_id: String,
    pub devices: Vec<DeviceInGroupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapPublishRequest {
    pub config_json: DeviceGroupPublishConfig,
    pub driver_config_json: DriverConfig,
}

impl ConfigMapPublishRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.config_json.tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }
        if self.config_json.group_id.is_empty() {
            return Err("group_id is required".to_string());
        }
        if self.config_json.devices.is_empty() {
            return Err("devices cannot be empty".to_string());
        }
        
        if self.driver_config_json.driver_name.is_empty() {
            return Err("driver_name is required".to_string());
        }
        if self.driver_config_json.driver_type.is_empty() {
            return Err("driver_type is required".to_string());
        }
        
        Ok(())
    }
}
