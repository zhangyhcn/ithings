use crate::config::DriverConfig;
use crate::publisher::ZmqPublisher;
use crate::subscriber::ZmqSubscriber;
use crate::types::{DataPoint, DeviceProfile, DriverMetadata, DriverStatus};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

#[async_trait]
pub trait Driver: Send + Sync {
    fn metadata(&self) -> DriverMetadata;
    
    fn device_name(&self) -> Option<&str> {
        None
    }
    
    async fn initialize(&mut self, config: DriverConfig) -> Result<()>;
    
    async fn connect(&mut self) -> Result<()>;
    
    async fn disconnect(&mut self) -> Result<()>;
    
    async fn add_device_profile(&mut self, profile: DeviceProfile) -> Result<()> {
        Ok(())
    }
    
    async fn read(&self) -> Result<Vec<DataPoint>>;
    
    async fn write(&self, data_point: &DataPoint) -> Result<()>;
    
    async fn status(&self) -> DriverStatus;
}

#[derive(Debug)]
pub struct BaseDriver {
    config: Option<DriverConfig>,
    publisher: Option<ZmqPublisher>,
    subscriber: Option<ZmqSubscriber>,
    status: Arc<RwLock<DriverStatus>>,
}

impl BaseDriver {
    pub fn new() -> Self {
        Self {
            config: None,
            publisher: None,
            subscriber: None,
            status: Arc::new(RwLock::new(DriverStatus::Stopped)),
        }
    }

    pub async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        self.publisher = Some(ZmqPublisher::new(&config.zmq)?);
        self.subscriber = ZmqSubscriber::new(&config.zmq)?;
        self.config = Some(config);
        *self.status.write().await = DriverStatus::Starting;
        Ok(())
    }

    pub fn publisher(&self) -> Option<&ZmqPublisher> {
        self.publisher.as_ref()
    }

    pub fn subscriber(&self) -> Option<&ZmqSubscriber> {
        self.subscriber.as_ref()
    }

    pub fn config(&self) -> Option<&DriverConfig> {
        self.config.as_ref()
    }

    pub async fn status(&self) -> DriverStatus {
        *self.status.read().await
    }

    pub async fn set_status(&self, status: DriverStatus) {
        *self.status.write().await = status;
    }

    pub async fn run_polling_loop<D: Driver + ?Sized>(&self, driver: &mut D) -> Result<()> {
        let poll_interval = self.config
            .as_ref()
            .map(|c| c.poll_interval_ms)
            .unwrap_or(1000);
        
        let mut ticker = interval(Duration::from_millis(poll_interval));
        
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    match driver.read().await {
                        Ok(data_points) => {
                            if let Some(publisher) = &self.publisher {
                                if let Some(device_name) = driver.device_name() {
                                    if let Err(e) = publisher.publish_batch(device_name, &data_points).await {
                                        tracing::error!("Failed to publish data: {}", e);
                                    }
                                } else {
                                    if let Err(e) = publisher.publish_batch("", &data_points).await {
                                        tracing::error!("Failed to publish data: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to read data: {}", e);
                        }
                    }
                }
               write_req = async {
                    if let Some(subscriber) = &self.subscriber {
                        subscriber.recv_write_request().await.transpose().map(|res| res.ok())
                    } else {
                        None
                    }
                } => {
                    if let Some(Some(data_point)) = write_req {
                        if let Err(e) = driver.write(&data_point).await {
                            tracing::error!("Failed to write data point '{}': {}", data_point.name, e);
                        } else {
                            tracing::info!("Successfully wrote data point '{}' = {:?}", data_point.name, data_point.value);
                        }
                    }
                }
            }
        }
    }
}

impl Default for BaseDriver {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MultiDeviceDriver<D: Driver + Default> {
    base: BaseDriver,
    device_manager: crate::device_manager::DeviceInstanceManager<D>,
}

impl<D: Driver + Default> MultiDeviceDriver<D> {
    pub fn new(base_config: DriverConfig) -> Self {
        Self {
            base: BaseDriver::new(),
            device_manager: crate::device_manager::DeviceInstanceManager::new(base_config),
        }
    }

    pub async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        self.base.initialize(config.clone()).await?;
        
        // 如果配置中有profile，自动创建设备实例
        if config.custom.contains_key("profile") || config.custom.contains_key("profiles") {
            let device_instance_id = config.device_instance_id.clone();
            let device_config = crate::device_manager::DeviceInstanceConfig {
                device_instance_id: device_instance_id.clone(),
                device_profile: None,
                custom: config.custom.clone(),
                poll_interval_ms: Some(config.poll_interval_ms),
            };
            
            tracing::info!("Creating initial device instance from config: {}", device_instance_id);
            self.device_manager.upsert_device(device_config).await?;
        }
        
        Ok(())
    }

    pub fn device_manager(&self) -> &crate::device_manager::DeviceInstanceManager<D> {
        &self.device_manager
    }

    pub fn device_manager_mut(&mut self) -> &mut crate::device_manager::DeviceInstanceManager<D> {
        &mut self.device_manager
    }

    pub async fn handle_config_update(&mut self, config: crate::device_manager::DeviceInstanceConfig) -> Result<bool> {
        let changed = self.device_manager.upsert_device(config).await?;
        Ok(changed)
    }

    pub async fn handle_config_delete(&mut self, device_instance_id: &str) -> Result<()> {
        self.device_manager.remove_device(device_instance_id).await?;
        Ok(())
    }

    pub async fn run_polling_loop(&mut self) -> Result<()> {
        let base_poll_interval = self.base.config()
            .map(|c| c.poll_interval_ms)
            .unwrap_or(1000);
        
        tracing::info!("Starting polling loop with interval {}ms, {} devices, publisher={:?}", 
            base_poll_interval, 
            self.device_manager.get_all_devices().len(),
            self.base.publisher().map(|p| p.is_enabled()));
        
        let mut ticker = interval(Duration::from_millis(base_poll_interval));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        
        loop {
            tokio::select! {
                biased;
                
                _ = ticker.tick() => {
                    tracing::trace!("Ticker ticked, polling {} devices", self.device_manager.get_all_devices().len());
                    for (_, instance) in self.device_manager.get_all_devices() {
                        match instance.driver.read().await {
                            Ok(data_points) => {
                                tracing::info!("Read {} data points from device {}", data_points.len(), instance.id);
                                if let Some(publisher) = self.base.publisher() {
                                    tracing::info!("Publisher enabled: {}", publisher.is_enabled());
                                    if !data_points.is_empty() {
                                        tracing::info!("Publishing {} data points for device {}", data_points.len(), instance.id);
                                        if let Err(e) = publisher.publish_batch(&instance.id, &data_points).await {
                                            tracing::error!("Failed to publish data for device {}: {}", instance.id, e);
                                        }
                                    }
                                } else {
                                    tracing::warn!("No publisher available for device {}", instance.id);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to read data for device {}: {}", instance.id, e);
                            }
                        }
                    }
                }
                message = async {
                    if let Some(subscriber) = &self.base.subscriber() {
                        subscriber.recv_message().await.transpose().map(|res| res.ok())
                    } else {
                        std::future::pending().await
                    }
                } => {
                    if let Some(Some(msg)) = message {
                        match msg {
                            crate::subscriber::IncomingMessage::WriteRequest(data_point) => {
                                let device_id = data_point.metadata.get("device_instance_id");
                                if let Some(device_id) = device_id {
                                    if let Some(instance) = self.device_manager.get_device(device_id) {
                                        if let Err(e) = instance.driver.write(&data_point).await {
                                            tracing::error!("Failed to write data point '{}' for device {}: {}", 
                                                data_point.name, device_id, e);
                                        } else {
                                            tracing::info!("Successfully wrote data point '{}' = {:?} for device {}", 
                                                data_point.name, data_point.value, device_id);
                                        }
                                    } else {
                                        tracing::warn!("Device instance {} not found for write request", device_id);
                                    }
                                } else {
                                    tracing::warn!("Write request missing device_instance_id in metadata");
                                }
                            }
                            crate::subscriber::IncomingMessage::ConfigUpdate(config) => {
                                tracing::info!("Received config update for device instance: {}", config.device_instance_id);
                                match self.handle_config_update(config).await {
                                    Ok(true) => {
                                        tracing::info!("Device instance config updated successfully");
                                    }
                                    Ok(false) => {
                                        tracing::debug!("Device instance config unchanged, skipping");
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to update device instance config: {}", e);
                                    }
                                }
                            }
                            crate::subscriber::IncomingMessage::ConfigDelete(device_id) => {
                                tracing::info!("Received config delete for device instance: {}", device_id);
                                if let Err(e) = self.handle_config_delete(&device_id).await {
                                    tracing::error!("Failed to delete device instance: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
