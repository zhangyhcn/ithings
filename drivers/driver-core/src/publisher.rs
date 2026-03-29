use crate::config::ZmqConfig;
use crate::types::DataPoint;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, PUB};

pub struct ZmqPublisher {
    socket: Arc<Mutex<Socket>>,
    topic: String,
    enabled: bool,
}

impl std::fmt::Debug for ZmqPublisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZmqPublisher")
            .field("topic", &self.topic)
            .field("enabled", &self.enabled)
            .field("socket", &"<zmq::Socket>")
            .finish()
    }
}

impl ZmqPublisher {
    pub fn new(config: &ZmqConfig) -> Result<Self> {
        let context = Context::new();

        if !config.enabled {
            tracing::info!("ZeroMQ publisher is disabled");
            let socket = context.socket(PUB)?;
            return Ok(Self {
                socket: Arc::new(Mutex::new(socket)),
                topic: config.topic.clone(),
                enabled: false,
            });
        }

        let socket = context.socket(PUB)?;
        
        if let Some(hwm) = config.high_water_mark {
            socket.set_sndhwm(hwm as i32)?;
        }
        
        let router_address = config.router_address.as_deref().unwrap_or("tcp://localhost");
        let router_sub_port = config.router_sub_port.unwrap_or(5550);
        let connect_address = format!("{}:{}", router_address, router_sub_port);
        
        socket.connect(&connect_address)?;
        
        tracing::info!(
            "ZeroMQ publisher connected to router {} with topic '{}'",
            connect_address,
            config.topic
        );

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            topic: config.topic.clone(),
            enabled: true,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub async fn publish(&self, device_name: &str, data_point: &DataPoint) -> Result<()> {
        if !self.enabled {
            tracing::debug!("Publisher disabled, skipping publish for {}", data_point.name);
            return Ok(());
        }

        let topic = format!("{}{}{}/{}", 
            self.topic,
            if self.topic.ends_with('/') { "" } else { "/" },
            device_name,
            data_point.name
        );
        
        let payload = serde_json::to_string(data_point)?;
        let message = format!("{} {}", topic, payload);
        
        tracing::debug!("Publishing message: {}", &message[..message.len().min(200)]);
        
        let socket = self.socket.lock().await;
        socket.send(message.as_bytes(), 0)?;
        
        tracing::trace!(
            "Published data point: {}/{} = {:?}",
            device_name,
            data_point.name,
            data_point.value
        );

        Ok(())
    }

    pub async fn publish_batch(&self, device_name: &str, data_points: &[DataPoint]) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        for data_point in data_points {
            self.publish(device_name, data_point).await?;
        }

        Ok(())
    }

    pub async fn publish_json(&self, json: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let message = format!("{} {}", self.topic, json);
        let socket = self.socket.lock().await;
        socket.send(message.as_bytes(), 0)?;

        Ok(())
    }
}

impl Clone for ZmqPublisher {
    fn clone(&self) -> Self {
        Self {
            socket: Arc::clone(&self.socket),
            topic: self.topic.clone(),
            enabled: self.enabled,
        }
    }
}
