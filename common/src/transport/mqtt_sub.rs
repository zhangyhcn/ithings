use crate::config::MqttConfig;
use crate::transport::subscriber::RemoteSubscriber;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::time::Duration;

pub struct MqttSubscriber {
    client: Option<Arc<Mutex<AsyncClient>>>,
    receiver: Option<Arc<Mutex<mpsc::Receiver<DataPoint>>>>,
    config: MqttConfig,
    connected: bool,
}

impl MqttSubscriber {
    pub fn new(config: &MqttConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::info!("MQTT subscriber is disabled");
            return Ok(None);
        }

        Ok(Some(Self {
            client: None,
            receiver: None,
            config: config.clone(),
            connected: false,
        }))
    }
}

#[async_trait]
impl RemoteSubscriber for MqttSubscriber {
    async fn subscribe(&mut self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("MQTT subscriber is disabled");
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
        let (tx, rx) = mpsc::channel::<DataPoint>(100);

        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        match event {
                            Event::Incoming(Packet::Publish(publish)) => {
                                if let Ok(data_point) = serde_json::from_slice::<DataPoint>(&publish.payload) {
                                    let _ = tx.send(data_point).await;
                                }
                            }
                            Event::Incoming(Packet::ConnAck(_)) => {
                                tracing::info!("MQTT subscriber connected");
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        tracing::error!("MQTT subscriber eventloop error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        self.client = Some(Arc::new(Mutex::new(client)));
        self.receiver = Some(Arc::new(Mutex::new(rx)));
        self.connected = true;
        tracing::info!("MQTT subscriber connected to {}", self.config.broker_address);

        Ok(())
    }

    async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        if !self.config.enabled || !self.connected {
            return Ok(None);
        }

        if let Some(receiver) = &self.receiver {
            let mut rx = receiver.lock().await;
            match rx.try_recv() {
                Ok(data_point) => Ok(Some(data_point)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(e) => Err(anyhow::anyhow!("MQTT receive error: {}", e)),
            }
        } else {
            Ok(None)
        }
    }

    fn enabled(&self) -> bool {
        self.config.enabled
    }

    fn connected(&self) -> bool {
        self.connected
    }

    fn subscriber_type(&self) -> &str {
        "mqtt"
    }
}
