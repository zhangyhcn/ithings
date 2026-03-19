use crate::config::DeviceConfig;
use crate::transport::publisher::{RemotePublisher, PublisherType};
use crate::transport::subscriber::{RemoteSubscriber, SubscriberType};
use crate::transport::mqtt::MqttPublisher;
use crate::transport::kafka::KafkaPublisher;
use crate::transport::kafka_sub::KafkaSubscriber;
use crate::transport::zmq_sub::ZmqSubscriber;
use anyhow::Result;

pub struct PublisherFactory;

impl PublisherFactory {
    pub fn create(config: &DeviceConfig) -> Result<Option<Box<dyn RemotePublisher>>> {
        if let Some(publisher_type) = config.custom.get("publisher_type") {
            let publisher_type: PublisherType = serde_json::from_value(publisher_type.clone())?;
            
            match publisher_type {
                PublisherType::Mqtt => {
                    let publisher = MqttPublisher::new(&config.mqtt);
                    Ok(Some(Box::new(publisher)))
                }
                PublisherType::Kafka => {
                    let publisher = KafkaPublisher::new(&config.kafka);
                    Ok(Some(Box::new(publisher)))
                }
            }
        } else if config.kafka.enabled {
            let publisher = KafkaPublisher::new(&config.kafka);
            Ok(Some(Box::new(publisher)))
        } else if config.mqtt.enabled {
            let publisher = MqttPublisher::new(&config.mqtt);
            Ok(Some(Box::new(publisher)))
        } else {
            Ok(None)
        }
    }
}

pub struct SubscriberFactory;

impl SubscriberFactory {
    pub fn create(config: &DeviceConfig) -> Result<Option<Box<dyn RemoteSubscriber>>> {
        if let Some(subscriber_type) = config.custom.get("subscriber_type") {
            let subscriber_type: SubscriberType = serde_json::from_value(subscriber_type.clone())?;
            
            match subscriber_type {
                SubscriberType::Zmq => {
                    if config.zmq.enabled {
                        if let Some(subscriber) = ZmqSubscriber::new(&config.zmq)? {
                            Ok(Some(Box::new(subscriber)))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                }
                SubscriberType::Kafka => {
                    if config.kafka.enabled {
                        if let Some(subscriber) = KafkaSubscriber::new(&config.kafka)? {
                            Ok(Some(Box::new(subscriber)))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                }
            }
        } else {
            if config.zmq.enabled {
                if let Some(subscriber) = ZmqSubscriber::new(&config.zmq)? {
                    Ok(Some(Box::new(subscriber)))
                } else {
                    Ok(None)
                }
            } else if config.kafka.enabled {
                if let Some(subscriber) = KafkaSubscriber::new(&config.kafka)? {
                    Ok(Some(Box::new(subscriber)))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        }
    }
}
