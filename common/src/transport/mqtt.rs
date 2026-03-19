use crate::config::MqttConfig;
use crate::transport::publisher::RemotePublisher;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub struct MqttPublisher {
    client: Option<Arc<Mutex<AsyncClient>>>,
    config: MqttConfig,
    connected: bool,
}

impl MqttPublisher {
    pub fn new(config: &MqttConfig) -> Self {
        Self {
            client: None,
            config: config.clone(),
            connected: false,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("MQTT publisher is disabled");
            return Ok(());
        }

        let broker: Vec<&str> = self.config.broker_address.split("://").collect();
        let host_port: Vec<&str> = broker.get(1).unwrap_or(&"localhost:1883").split(':').collect();
        let host = host_port.get(0).unwrap_or(&"localhost").to_string();
        let port = host_port.get(1).unwrap_or(&"1883").parse::<u16>().unwrap_or(1883);

        let mut options = MqttOptions::new(&self.config.client_id, &host, port);
        options.set_keep_alive(Duration::from_secs(30));
        
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            options.set_credentials(username, password);
        }

        let (client, mut eventloop) = AsyncClient::new(options, 10);

        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        match event {
                            Event::Incoming(Packet::ConnAck(_)) => {
                                tracing::info!("MQTT connected");
                            }
                            Event::Incoming(Packet::Publish(_)) => {}
                            Event::Outgoing(_) => {}
                            _ => {}
                        }
                    }
                    Err(e) => {
                        tracing::error!("MQTT eventloop error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        self.client = Some(Arc::new(Mutex::new(client)));
        self.connected = true;
        tracing::info!("Connected to MQTT broker at {}", self.config.broker_address);

        Ok(())
    }

    pub async fn publish(&self, device_name: &str, data_point: &DataPoint) -> Result<()> {
        if !self.config.enabled || !self.connected {
            return Ok(());
        }

        let topic = format!(
            "{}/{}/{}",
            self.config.topic_prefix.trim_end_matches('/'),
            device_name,
            data_point.name
        );

        let payload = serde_json::to_string(data_point)?;
        if let Some(client) = &self.client {
            let client_guard = client.lock().await;
            let qos = match self.config.qos {
                0 => QoS::AtMostOnce,
                1 => QoS::AtLeastOnce,
                2 => QoS::ExactlyOnce,
                _ => QoS::AtLeastOnce,
            };
            client_guard.publish(&topic, qos, false, payload.into_bytes()).await?;
            tracing::trace!("Published to MQTT: {}", topic);
        }

        Ok(())
    }

    pub async fn publish_batch(&self, device_name: &str, data_points: &[DataPoint]) -> Result<()> {
        if !self.config.enabled || !self.connected {
            return Ok(());
        }

        for data_point in data_points {
            self.publish(device_name, data_point).await?;
        }

        Ok(())
    }

    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn connected(&self) -> bool {
        self.connected
    }
}

#[async_trait]
impl RemotePublisher for MqttPublisher {
    async fn connect(&mut self) -> Result<()> {
        self.connect().await
    }

    async fn publish(&self, device_name: &str, data_point: &DataPoint) -> Result<()> {
        self.publish(device_name, data_point).await
    }

    async fn publish_batch(&self, device_name: &str, data_points: &[DataPoint]) -> Result<()> {
        self.publish_batch(device_name, data_points).await
    }

    fn enabled(&self) -> bool {
        self.enabled()
    }

    fn connected(&self) -> bool {
        self.connected()
    }

    fn publisher_type(&self) -> &str {
        "mqtt"
    }
}
