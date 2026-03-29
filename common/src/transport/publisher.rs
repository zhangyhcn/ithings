use crate::types::{DataPoint, DeviceEvent};
use crate::device_core::ServiceResult;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait RemotePublisher: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn publish(&self, device_instance_id: &str, data_point: &DataPoint) -> Result<()>;
    async fn publish_batch(&self, device_instance_id: &str, data_points: &[DataPoint]) -> Result<()>;
    async fn publish_write(&self, device_instance_id: &str, data_point: &DataPoint) -> Result<()>;
    async fn publish_event(&self, device_instance_id: &str, event: &DeviceEvent) -> Result<()>;
    async fn publish_service_reply(&self, device_instance_id: &str, reply: &ServiceResult) -> Result<()>;
    fn enabled(&self) -> bool;
    fn connected(&self) -> bool;
    fn publisher_type(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PublisherType {
    Mqtt,
    Kafka,
}

impl std::str::FromStr for PublisherType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mqtt" => Ok(PublisherType::Mqtt),
            "kafka" => Ok(PublisherType::Kafka),
            _ => Err(anyhow::anyhow!("Unknown publisher type: {}", s)),
        }
    }
}
