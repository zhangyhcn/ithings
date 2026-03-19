use crate::config::DriverConfig;
use crate::driver::Driver;
use crate::types::DeviceProfile;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInstanceConfig {
    pub device_instance_id: String,
    pub device_profile: DeviceProfile,
    pub custom: HashMap<String, serde_json::Value>,
    pub poll_interval_ms: Option<u64>,
}

#[derive(Debug)]
pub struct DeviceInstance<D: Driver> {
    pub id: String,
    pub config_hash: String,
    pub driver: D,
    pub config: DeviceInstanceConfig,
}

pub struct DeviceInstanceManager<D: Driver> {
    instances: HashMap<String, DeviceInstance<D>>,
    base_config: DriverConfig,
}

impl<D: Driver + Default> DeviceInstanceManager<D> {
    pub fn new(base_config: DriverConfig) -> Self {
        Self {
            instances: HashMap::new(),
            base_config,
        }
    }

    pub fn calculate_config_hash(config: &DeviceInstanceConfig) -> String {
        let json = serde_json::to_string(config).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub async fn upsert_device(&mut self, config: DeviceInstanceConfig) -> Result<bool> {
        let hash = Self::calculate_config_hash(&config);

        if let Some(existing) = self.instances.get(&config.device_instance_id) {
            if existing.config_hash == hash {
                tracing::debug!(
                    "Device instance {} unchanged, hash matches: {}", 
                    config.device_instance_id, 
                    hash
                );
                return Ok(false);
            }
            tracing::info!(
                "Device instance {} config changed, stopping old instance", 
                config.device_instance_id
            );
            self.remove_device(&config.device_instance_id).await?;
        }

        tracing::info!("Creating new device instance: {}", config.device_instance_id);
        self.create_device(config, hash).await?;
        Ok(true)
    }

    async fn create_device(&mut self, config: DeviceInstanceConfig, hash: String) -> Result<()> {
        let device_instance_id = config.device_instance_id.clone();
        let mut driver = D::default();

        let mut full_config = self.base_config.clone();
        full_config.device_instance_id = device_instance_id.clone();
        full_config.custom = config.custom.clone();
        if let Some(poll_interval) = config.poll_interval_ms {
            full_config.poll_interval_ms = poll_interval;
        }

        driver.initialize(full_config.clone()).await?;

        if let Some(profile_json) = full_config.custom.get("profile") {
            let profile: DeviceProfile = serde_json::from_value::<DeviceProfile>(profile_json.clone())?;
            driver.add_device_profile(profile).await?;
        }

        if let Some(profiles_json) = full_config.custom.get("profiles") {
            if let serde_json::Value::Array(profiles_array) = profiles_json {
                for profile_json in profiles_array {
                    let profile: DeviceProfile = serde_json::from_value::<DeviceProfile>(profile_json.clone())?;
                    driver.add_device_profile(profile).await?;
                }
            }
        }

        driver.connect().await?;

        self.instances.insert(
            device_instance_id.clone(),
            DeviceInstance {
                id: device_instance_id.clone(),
                config_hash: hash,
                driver,
                config,
            }
        );

        tracing::info!("Device instance {} created and connected successfully", device_instance_id);
        Ok(())
    }

    pub async fn remove_device(&mut self, device_instance_id: &str) -> Result<()> {
        if let Some(mut instance) = self.instances.remove(device_instance_id) {
            tracing::info!("Stopping and removing device instance: {}", device_instance_id);
            instance.driver.disconnect().await?;
            Ok(())
        } else {
            tracing::debug!("Device instance {} not found for removal", device_instance_id);
            Ok(())
        }
    }

    pub fn get_device(&self, device_instance_id: &str) -> Option<&DeviceInstance<D>> {
        self.instances.get(device_instance_id)
    }

    pub fn get_device_mut(&mut self, device_instance_id: &str) -> Option<&mut DeviceInstance<D>> {
        self.instances.get_mut(device_instance_id)
    }

    pub fn get_all_devices(&self) -> &HashMap<String, DeviceInstance<D>> {
        &self.instances
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub async fn stop_all(&mut self) -> Result<()> {
        tracing::info!("Stopping all {} device instances", self.instances.len());
        let device_ids: Vec<String> = self.instances.keys().cloned().collect();
        for id in device_ids {
            self.remove_device(&id).await?;
        }
        Ok(())
    }
}