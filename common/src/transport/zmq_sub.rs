use crate::config::ZmqConfig;
use crate::transport::subscriber::RemoteSubscriber;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, SUB};

pub struct ZmqSubscriber {
    _context: Arc<Context>,
    socket: Arc<Mutex<Socket>>,
    write_topic: String,
    properties_topic: String,
    enabled: bool,
}

impl ZmqSubscriber {
    pub fn new(config: &ZmqConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::info!("ZeroMQ subscriber is disabled");
            return Ok(None);
        }

        let context = Arc::new(Context::new());
        let socket = context.socket(SUB)?;
        
        socket.set_subscribe(config.write_topic.as_bytes())?;
        socket.set_subscribe(config.properties_topic.as_bytes())?;
        socket.connect(&config.subscriber_address)?;
        
        tracing::info!(
            "ZeroMQ subscriber connected to {} subscribed to topics: '{}', '{}'",
            config.subscriber_address,
            config.write_topic,
            config.properties_topic
        );

        Ok(Some(Self {
            _context: context,
            socket: Arc::new(Mutex::new(socket)),
            write_topic: config.write_topic.clone(),
            properties_topic: config.properties_topic.clone(),
            enabled: true,
        }))
    }

    pub async fn recv_message(&self) -> Result<Option<(String, Vec<u8>)>> {
        if !self.enabled {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            return Ok(None);
        }

        let socket = self.socket.lock().await;
        
        let mut topic_msg = zmq::Message::new();
        let mut payload_msg = zmq::Message::new();
        
        match socket.recv(&mut topic_msg, zmq::DONTWAIT) {
            Ok(_) => {
                let topic_bytes: Vec<u8> = topic_msg.iter().cloned().collect();
                let topic = String::from_utf8_lossy(&topic_bytes).to_string();
                tracing::debug!("ZMQ received topic: {}", topic);
                
                let more = socket.get_rcvmore()?;
                if more {
                    socket.recv(&mut payload_msg, 0)?;
                    let payload: Vec<u8> = payload_msg.iter().cloned().collect();
                    tracing::debug!("ZMQ received payload: {} bytes", payload.len());
                    Ok(Some((topic, payload)))
                } else {
                    tracing::debug!("ZMQ message has no payload part");
                    Ok(None)
                }
            }
            Err(zmq::Error::EAGAIN) => {
                Ok(None)
            }
            Err(e) => {
                Err(anyhow::anyhow!("Failed to receive message: {}", e))
            }
        }
    }

    pub async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        if let Some((topic, payload)) = self.recv_message().await? {
            if topic == self.write_topic {
                let data_point: DataPoint = serde_json::from_slice(&payload)?;
                return Ok(Some(data_point));
            }
        }
        Ok(None)
    }

    pub async fn recv_properties(&self) -> Result<Option<Vec<DataPoint>>> {
        if let Some((topic, payload)) = self.recv_message().await? {
            if topic == self.properties_topic {
                let data_points: Vec<DataPoint> = serde_json::from_slice(&payload)?;
                return Ok(Some(data_points));
            }
        }
        Ok(None)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Clone for ZmqSubscriber {
    fn clone(&self) -> Self {
        Self {
            _context: Arc::clone(&self._context),
            socket: Arc::clone(&self.socket),
            write_topic: self.write_topic.clone(),
            properties_topic: self.properties_topic.clone(),
            enabled: self.enabled,
        }
    }
}

#[async_trait]
impl RemoteSubscriber for ZmqSubscriber {
    async fn subscribe(&mut self) -> Result<()> {
        tracing::info!("ZeroMQ subscriber already subscribed to topics: {}, {}", self.write_topic, self.properties_topic);
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
