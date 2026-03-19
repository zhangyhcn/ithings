use crate::config::driver::DriverConfig;
use crate::types::{DataPoint};
use crate::transport::driver_comm::{DriverClient, ReadRequest, ReadResponse, DataPointRequest};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, REQ};
use uuid::Uuid;

pub struct ZmqDriverClient {
    socket: Arc<Mutex<Socket>>,
    server_address: String,
    connected: bool,
}

impl ZmqDriverClient {
    pub fn new(server_address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(REQ)?;
        
        socket.connect(server_address)?;
        
        tracing::info!(
            "Connected to driver server at {}",
            server_address
        );

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            server_address: server_address.to_string(),
            connected: true,
        })
    }
}

#[async_trait]
impl DriverClient for ZmqDriverClient {
    async fn read(&self, device_name: &str, resource_names: &[&str]) -> Result<ReadResponse> {
        let request = ReadRequest {
            request_id: Uuid::new_v4().to_string(),
            device_name: device_name.to_string(),
            resources: resource_names.iter()
                .map(|name| DataPointRequest { name: name.to_string() })
                .collect(),
        };

        let socket = self.socket.lock().await;
        
        let json = serde_json::to_string(&request)?;
        socket.send(json.as_bytes(), 0)?;
        
        let mut response_msg = zmq::Message::new();
        socket.recv(&mut response_msg, 0)?;
        
        let response: ReadResponse = serde_json::from_slice(&response_msg.iter().cloned().collect::<Vec<u8>>())?;
        Ok(response)
    }

    async fn read_all(&self, device_name: &str, resource_names: Vec<String>) -> Result<Vec<DataPoint>> {
        let names: Vec<&str> = resource_names.iter().map(|s| s.as_str()).collect();
        let response = self.read(device_name, &names).await?;
        Ok(response.data_points)
    }

    async fn send_config(&self, config: &DriverConfig) -> Result<()> {
        let json = serde_json::to_string(config)?;
        let socket = self.socket.lock().await;
        socket.send(json.as_bytes(), 0)?;
        
        let mut response_msg = zmq::Message::new();
        socket.recv(&mut response_msg, 0)?;
        
        Ok(())
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
