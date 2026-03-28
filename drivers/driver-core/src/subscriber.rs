use common::config::topics::*;
use crate::config::ZmqConfig;
use crate::types::DataPoint;
use crate::DeviceInstanceConfig;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, SUB};

#[derive(Debug, Clone)]
pub enum IncomingMessage {
    WriteRequest(DataPoint),
    ConfigUpdate(DeviceInstanceConfig),
    ConfigDelete(String),
}

pub struct ZmqSubscriber {
    socket: Arc<Mutex<Socket>>,
    write_topic: String,
    config_update_topic: String,
    enabled: bool,
}

impl std::fmt::Debug for ZmqSubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZmqSubscriber")
            .field("write_topic", &self.write_topic)
            .field("config_update_topic", &self.config_update_topic)
            .field("enabled", &self.enabled)
            .field("socket", &"<zmq::Socket>")
            .finish()
    }
}

impl Clone for ZmqSubscriber {
    fn clone(&self) -> Self {
        Self {
            socket: Arc::clone(&self.socket),
            write_topic: self.write_topic.clone(),
            config_update_topic: self.config_update_topic.clone(),
            enabled: self.enabled,
        }
    }
}

impl ZmqSubscriber {
    pub fn new(config: &ZmqConfig) -> Result<Option<Self>> {
        if !config.subscriber_enabled {
            tracing::info!("ZeroMQ subscriber is disabled");
            return Ok(None);
        }

        let context = Context::new();
        let socket = context.socket(SUB)?;
        
        socket.set_subscribe(config.write_topic.as_bytes())?;
        socket.set_subscribe(config.config_update_topic.as_bytes())?;
        socket.connect(&config.subscriber_address)?;
        
        tracing::info!(
            "ZeroMQ subscriber connected to {} subscribed to topics: write='{}', config_update='{}'",
            config.subscriber_address,
            config.write_topic,
            config.config_update_topic
        );

        Ok(Some(Self {
            socket: Arc::new(Mutex::new(socket)),
            write_topic: config.write_topic.clone(),
            config_update_topic: config.config_update_topic.clone(),
            enabled: true,
        }))
    }

    pub async fn recv_message(&self) -> Result<Option<IncomingMessage>> {
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
                
                let (topic_first_byte, payload_part) = content.split_first().ok_or_else(
                    || anyhow::anyhow!("Invalid message format: expected topic and payload")
                )?;
                let mut topic_bytes = Vec::new();
                topic_bytes.push(*topic_first_byte);
                let (null_pos, _) = payload_part.iter().enumerate()
                    .find(|(_, &b)| b == 0)
                    .unwrap_or((payload_part.len(), &0));
                topic_bytes.extend_from_slice(&payload_part[..null_pos]);
                let topic_str = String::from_utf8_lossy(&topic_bytes).to_string();
                let payload_part = if null_pos < payload_part.len() {
                    &payload_part[(null_pos + 1)..]
                } else {
                    &[]
                };
                
                if topic_str == self.write_topic {
                    let data_point: DataPoint = serde_json::from_slice(payload_part)?;
                    Ok(Some(IncomingMessage::WriteRequest(data_point)))
                } else if topic_str == self.config_update_topic {
                    let config: DeviceInstanceConfig = serde_json::from_slice(payload_part)?;
                    Ok(Some(IncomingMessage::ConfigUpdate(config)))
                } else if topic_str == CONFIG_DELETE_TOPIC {
                    let device_id: String = serde_json::from_slice(payload_part)?;
                    Ok(Some(IncomingMessage::ConfigDelete(device_id)))
                } else {
                    tracing::warn!("Received message on unknown topic: {}", topic_str);
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
        match self.recv_message().await {
            Ok(Some(IncomingMessage::WriteRequest(dp))) => Ok(Some(dp)),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
