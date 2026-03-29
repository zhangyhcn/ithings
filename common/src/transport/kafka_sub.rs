use crate::config::KafkaConfig;
use crate::transport::subscriber::RemoteSubscriber;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct KafkaSubscriber {
    consumer: Option<Arc<Mutex<StreamConsumer>>>,
    config: KafkaConfig,
    connected: bool,
}

impl KafkaSubscriber {
    pub fn new(config: &KafkaConfig) -> Result<Option<Self>> {
        if !config.enabled {
            return Ok(None);
        }

        Ok(Some(Self {
            consumer: None,
            config: config.clone(),
            connected: false,
        }))
    }
}

#[async_trait]
impl RemoteSubscriber for KafkaSubscriber {
    async fn subscribe(&mut self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("Kafka subscriber is disabled");
            return Ok(());
        }

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.config.brokers);
        client_config.set("group.id", &self.config.consumer_group);
        
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            client_config.set("security.protocol", "SASL_SSL");
            client_config.set("sasl.mechanism", "PLAIN");
            client_config.set("sasl.username", username);
            client_config.set("sasl.password", password);
        }

        let consumer: StreamConsumer = client_config.create()?;
        consumer.subscribe(&[&self.config.write_topic, &self.config.properties_topic])?;
        
        self.consumer = Some(Arc::new(Mutex::new(consumer)));
        self.connected = true;
        tracing::info!("Subscribed to Kafka topics: {}, {}", self.config.write_topic, self.config.properties_topic);

        Ok(())
    }

    async fn recv_write_request(&self) -> Result<Option<DataPoint>> {
        if !self.config.enabled || !self.connected {
            return Ok(None);
        }

        if let Some(consumer) = &self.consumer {
            let consumer_guard = consumer.lock().await;
            match consumer_guard.recv().await {
                Ok(message) => {
                    let topic = message.topic();
                    if topic == self.config.write_topic {
                        match message.payload() {
                            Some(payload) => {
                                let data_point: DataPoint = serde_json::from_slice(payload)?;
                                return Ok(Some(data_point));
                            }
                            None => {}
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Kafka consumer error: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    async fn recv_properties(&self) -> Result<Option<Vec<DataPoint>>> {
        if !self.config.enabled || !self.connected {
            return Ok(None);
        }

        if let Some(consumer) = &self.consumer {
            let consumer_guard = consumer.lock().await;
            match consumer_guard.recv().await {
                Ok(message) => {
                    let topic = message.topic();
                    if topic == self.config.properties_topic {
                        match message.payload() {
                            Some(payload) => {
                                let data_points: Vec<DataPoint> = serde_json::from_slice(payload)?;
                                return Ok(Some(data_points));
                            }
                            None => {}
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Kafka consumer error: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    fn enabled(&self) -> bool {
        self.config.enabled
    }

    fn connected(&self) -> bool {
        self.connected
    }

    fn subscriber_type(&self) -> &str {
        "kafka"
    }
}
