use crate::config::driver::DriverConfig;
use crate::config::DriverClientConfig;
use crate::types::{DataPoint};
use crate::transport::driver_comm::{DriverClient, ReadRequest, ReadResponse, DataPointRequest};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, PUB};
use uuid::Uuid;

pub struct ZmqDriverClient {
    socket: Arc<Mutex<Socket>>,
    server_address: String,
    connected: bool,
}

impl ZmqDriverClient {
    pub fn new(config: &DriverClientConfig) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(PUB)?;
        
        let router_address = config.router_address.as_deref().unwrap_or("tcp://localhost");
        let router_sub_port = config.router_sub_port.unwrap_or(5550);
        let connect_address = format!("{}:{}", router_address, router_sub_port);
        
        socket.connect(&connect_address)?;
        
        tracing::info!(
            "ZMQ driver client (PUB) connected to router {}",
            connect_address
        );

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            server_address: connect_address,
            connected: true,
        })
    }
}

#[async_trait]
impl DriverClient for ZmqDriverClient {
    async fn read(&self, device_name: &str, resource_names: &[&str]) -> Result<ReadResponse> {
        Err(anyhow::anyhow!("Read not supported in PUB mode"))
    }

    async fn read_all(&self, device_name: &str, resource_names: Vec<String>) -> Result<Vec<DataPoint>> {
        Err(anyhow::anyhow!("Read not supported in PUB mode"))
    }

    async fn send_config(&self, config: &DriverConfig) -> Result<()> {
        let topic = "driver/config_update";
        let json = serde_json::to_string(config)?;
        let message = format!("{} {}", topic, json);
        
        let socket = self.socket.lock().await;
        socket.send(message.as_bytes(), 0)?;
        
        tracing::info!("Sent driver config via PUB to topic '{}'", topic);
        Ok(())
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
