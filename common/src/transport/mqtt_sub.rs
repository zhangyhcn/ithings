use crate::config::MqttConfig;
use crate::transport::subscriber::RemoteSubscriber;
use crate::types::DataPoint;
use crate::device_core::ServiceCallRequest;
use anyhow::Result;
use async_trait::async_trait;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashSet;

pub struct MqttSubscriber {
    client: Option<Arc<Mutex<AsyncClient>>>,
    write_receiver: Option<Arc<Mutex<mpsc::Receiver<DataPoint>>>>,
    service_receiver: Option<Arc<Mutex<mpsc::Receiver<ServiceCallRequest>>>>,
    config: MqttConfig,
    connected: bool,
    subscribed_topics: Arc<Mutex<HashSet<String>>>,
}

impl MqttSubscriber {
    pub fn new(config: &MqttConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::info!("MQTT subscriber is disabled");
            return Ok(None);
        }

        Ok(Some(Self {
            client: None,
            write_receiver: None,
            service_receiver: None,
            config: config.clone(),
            connected: false,
            subscribed_topics: Arc::new(Mutex::new(HashSet::new())),
        }))
    }

    pub async fn recv_service_call(&self) -> Result<Option<ServiceCallRequest>> {
        if !self.config.enabled || !self.connected {
            return Ok(None);
        }

        if let Some(receiver) = &self.service_receiver {
            let mut rx = receiver.lock().await;
            match rx.try_recv() {
                Ok(request) => {
                    tracing::info!("Got service call from channel: msg_id={}", request.msg_id);
                    Ok(Some(request))
                }
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(e) => Err(anyhow::anyhow!("Service call receive error: {}", e)),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn subscribe_service_topic(&self, device_instance_id: &str) -> Result<()> {
        let topic = if let (Some(tenant_id), Some(org_id), Some(site_id), Some(namespace_id)) = (
            &self.config.tenant_id,
            &self.config.org_id,
            &self.config.site_id,
            &self.config.namespace_id,
        ) {
            format!(
                "/{}/{}/{}/{}/{}/service/call",
                tenant_id, org_id, site_id, namespace_id, device_instance_id
            )
        } else {
            format!("{}/{}/service/call", 
                self.config.topic_prefix.trim_end_matches('/'),
                device_instance_id)
        };

        if let Some(client) = &self.client {
            let client_guard = client.lock().await;
            client_guard.subscribe(&topic, QoS::AtLeastOnce).await?;
            tracing::info!("MQTT subscriber subscribed to service call topic: {}", topic);
        }

        let mut topics = self.subscribed_topics.lock().await;
        topics.insert(topic);
        
        Ok(())
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
        let (write_tx, write_rx) = mpsc::channel::<DataPoint>(100);
        let (service_tx, service_rx) = mpsc::channel::<ServiceCallRequest>(100);

        let client_arc = Arc::new(Mutex::new(client));
        let client_clone = Arc::clone(&client_arc);
        let client_for_struct = Arc::clone(&client_arc);
        let topics_clone = Arc::clone(&self.subscribed_topics);

        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        tracing::trace!("MQTT subscriber event: {:?}", event);
                        match event {
                            Event::Incoming(Packet::Publish(publish)) => {
                                let topic = publish.topic.clone();
                                let payload = &publish.payload;
                                tracing::info!("MQTT subscriber received message on topic: {}", topic);
                                
                                if topic.ends_with("/service/call") {
                                    tracing::info!("Service call payload: {}", String::from_utf8_lossy(payload));
                                    if let Ok(request) = serde_json::from_slice::<ServiceCallRequest>(payload) {
                                        tracing::info!("Successfully parsed service call: msg_id={}, service_id={}", request.msg_id, request.service_id);
                                        if let Err(e) = service_tx.send(request).await {
                                            tracing::error!("Failed to send service call to channel: {}", e);
                                        } else {
                                            tracing::info!("Service call sent to channel successfully");
                                        }
                                    } else {
                                        tracing::warn!("Failed to parse service call request from topic: {}", topic);
                                    }
                                } else {
                                    if let Ok(data_point) = serde_json::from_slice::<DataPoint>(payload) {
                                        let _ = write_tx.send(data_point).await;
                                    }
                                }
                            }
                            Event::Incoming(Packet::ConnAck(_)) => {
                                tracing::info!("MQTT subscriber connected, resubscribing topics...");
                                let topics = topics_clone.lock().await;
                                let client = client_clone.lock().await;
                                for topic in topics.iter() {
                                    if let Err(e) = client.subscribe(topic, QoS::AtLeastOnce).await {
                                        tracing::warn!("Failed to resubscribe topic {}: {}", topic, e);
                                    } else {
                                        tracing::info!("Resubscribed topic: {}", topic);
                                    }
                                }
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

        self.client = Some(client_for_struct);
        self.write_receiver = Some(Arc::new(Mutex::new(write_rx)));
        self.service_receiver = Some(Arc::new(Mutex::new(service_rx)));
        self.connected = true;
        tracing::info!("MQTT subscriber connected to {}", self.config.broker_address);

        Ok(())
    }

    async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        if !self.config.enabled || !self.connected {
            return Ok(None);
        }

        if let Some(receiver) = &self.write_receiver {
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
