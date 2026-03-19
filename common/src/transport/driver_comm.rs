use crate::types::DataPoint;
use crate::config::driver::DriverConfig;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadRequest {
    pub request_id: String,
    pub device_name: String,
    pub resources: Vec<DataPointRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPointRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResponse {
    pub request_id: String,
    pub device_name: String,
    pub data_points: Vec<DataPoint>,
}

#[async_trait]
pub trait DriverServer: Send + Sync {
    async fn recv_request(&self) -> Result<(Vec<u8>, ReadRequest)>;
    fn send_response(&self, identity: &[u8], response: &ReadResponse) -> Result<()>;
    fn connected(&self) -> bool;
}

#[async_trait]
pub trait DriverClient: Send + Sync {
    async fn read(&self, device_name: &str, resource_names: &[&str]) -> Result<ReadResponse>;
    async fn read_all(&self, device_name: &str, resource_names: Vec<String>) -> Result<Vec<DataPoint>>;
    async fn send_config(&self, config: &DriverConfig) -> Result<()>;
    fn connected(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DriverCommType {
    Zmq,
    InMemory,
    TcpSocket,
}

impl std::str::FromStr for DriverCommType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zmq" => Ok(DriverCommType::Zmq),
            "inmemory" => Ok(DriverCommType::InMemory),
            "tcpsocket" => Ok(DriverCommType::TcpSocket),
            _ => Err(anyhow::anyhow!("Unknown driver communication type: {}", s)),
        }
    }
}
