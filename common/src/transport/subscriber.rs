use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait RemoteSubscriber: Send + Sync {
    async fn subscribe(&mut self) -> Result<()>;
    async fn recv_write_request(&self) -> Result<Option<DataPoint>>;
    async fn recv_properties(&self) -> Result<Option<Vec<DataPoint>>> {
        Ok(None)
    }
    fn enabled(&self) -> bool;
    fn connected(&self) -> bool;
    fn subscriber_type(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriberType {
    Zmq,
    Kafka,
}

impl std::str::FromStr for SubscriberType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zmq" => Ok(SubscriberType::Zmq),
            "kafka" => Ok(SubscriberType::Kafka),
            _ => Err(anyhow::anyhow!("Unknown subscriber type: {}", s)),
        }
    }
}
