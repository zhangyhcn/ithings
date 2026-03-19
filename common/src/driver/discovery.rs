use crate::config::{device::DeviceConfig, driver::DriverConfig};
use anyhow::Result;
use std::env;

pub struct ConfigDiscovery;

impl ConfigDiscovery {
    pub fn from_env_or_path() -> Result<DeviceConfig> {
        if let Ok(config_path) = env::var("DEVICE_CONFIG_PATH") {
            tracing::info!("Loading device configuration from DEVICE_CONFIG_PATH: {}", config_path);
            DeviceConfig::from_file(&config_path)
        } else if let Some(config_path) = get_config_path_from_args() {
            tracing::info!("Loading device configuration from command line: {}", config_path);
            DeviceConfig::from_file(&config_path)
        } else {
            tracing::warn!("No configuration path found, trying environment variables only");
            DeviceConfig::from_env()
        }
    }

    pub fn driver_config_from_env(device_config: &DeviceConfig) -> Result<DriverConfig> {
        if let Some(modbus_config) = device_config.custom.get("modbus") {
            let driver_config: DriverConfig = 
                serde_json::from_value(modbus_config.clone())?;
            Ok(driver_config)
        } else {
            Err(anyhow::anyhow!("No driver configuration found in device config.custom"))
        }
    }
}

fn get_config_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == "-c" || args[i] == "--config" {
            if i + 1 < args.len() {
                return Some(args[i + 1].clone());
            }
        }
    }
    None
}
