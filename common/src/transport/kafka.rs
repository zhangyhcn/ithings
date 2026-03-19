use crate::config::KafkaConfig;
use crate::transport::publisher::RemotePublisher;
use crate::types::DataPoint;
use anyhow::Result;
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub struct KafkaPublisher {
    producer: Option<FutureProducer>,
    config: KafkaConfig,
    connected: bool,
}

impl KafkaPublisher {
    pub fn new(config: &KafkaConfig) -> Self {
        Self {
            producer: None,
            config: config.clone(),
            connected: false,
        }
    }
}

#[async_trait]
impl RemotePublisher for KafkaPublisher {
    async fn connect(&mut self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("Kafka publisher is disabled");
            return Ok(());
        }

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.config.brokers);
        
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            client_config.set("security.protocol", "SASL_SSL");
            client_config.set("sasl.mechanism", "PLAIN");
            client_config.set("sasl.username", username);
            client_config.set("sasl.password", password);
        }

        let producer: FutureProducer = client_config.create()?;
        
        self.producer = Some(producer);
        self.connected = true;
        tracing::info!("Connected to Kafka brokers at {}", self.config.brokers);

        Ok(())
    }

    async fn publish(&self, device_name: &str, data_point: &DataPoint) -> Result<()> {
        if !self.config.enabled || !self.connected {
            return Ok(());
        }

        let topic = format!(
            "{}.{}",
            self.config.topic_prefix.trim_end_matches('.'),
            device_name
        );

        let payload = serde_json::to_string(data_point)?;
        
        if let Some(producer) = &self.producer {
            let record = FutureRecord::to(&topic)
                .payload(&payload)
                .key(&data_point.name);
            
            match producer.send(record, Duration::from_secs(5)).await {
                Ok(_) => {
                    tracing::trace!("Published to Kafka: {} -> {}", topic, data_point.name);
                }
                Err((e, _)) => {
                    tracing::error!("Failed to publish to Kafka: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    async fn publish_batch(&self, device_name: &str, data_points: &[DataPoint]) -> Result<()> {
        if !self.config.enabled || !self.connected {
            return Ok(());
        }

        for data_point in data_points {
            self.publish(device_name, data_point).await?;
        }

        Ok(())
    }

    fn enabled(&self) -> bool {
        self.config.enabled
    }

    fn connected(&self) -> bool {
        self.connected
    }

    fn publisher_type(&self) -> &str {
        "kafka"
    }
}
