use crate::config::ZmqConfig;
use crate::transport::subscriber::RemoteSubscriber;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, SUB};

pub struct ZmqSubscriber {
    socket: Arc<Mutex<Socket>>,
    topic: String,
    enabled: bool,
}

impl ZmqSubscriber {
    pub fn new(config: &ZmqConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::info!("ZeroMQ subscriber is disabled");
            return Ok(None);
        }

        let context = Context::new();
        let socket = context.socket(SUB)?;
        
        socket.set_subscribe(config.write_topic.as_bytes())?;
        socket.connect(&config.subscriber_address)?;
        
        tracing::info!(
            "ZeroMQ subscriber connected to {} subscribed to topic '{}'",
            config.subscriber_address,
            config.write_topic
        );

        Ok(Some(Self {
            socket: Arc::new(Mutex::new(socket)),
            topic: config.write_topic.clone(),
            enabled: true,
        }))
    }

    pub async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        if !self.enabled {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            return Ok(None);
        }

        let socket = self.socket.lock().await;
        
        let mut msg = zmq::Message::new();
        match socket.recv(&mut msg, 0) {
            Ok(_) => {
                let content = msg;
                if content.len() < 2 {
                    return Ok(None);
                }
                
                let (topic_part, payload_part) = content.split_first().ok_or_else(
                    || anyhow::anyhow!("Invalid message format: expected topic and payload")
                )?;
                
                let _ = topic_part;
                let data_point: DataPoint = serde_json::from_slice(payload_part)?;
                Ok(Some(data_point))
            }
            Err(zmq::Error::EAGAIN) => {
                Ok(None)
            }
            Err(e) => {
                Err(anyhow::anyhow!("Failed to receive message: {}", e))
            }
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Clone for ZmqSubscriber {
    fn clone(&self) -> Self {
        Self {
            socket: Arc::clone(&self.socket),
            topic: self.topic.clone(),
            enabled: self.enabled,
        }
    }
}

#[async_trait]
impl RemoteSubscriber for ZmqSubscriber {
    async fn subscribe(&mut self) -> Result<()> {
        tracing::info!("ZeroMQ subscriber already subscribed to topic: {}", self.topic);
        Ok(())
    }

    async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        self.recv_write_request().await
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn connected(&self) -> bool {
        self.enabled
    }

    fn subscriber_type(&self) -> &str {
        "zmq"
    }
}
