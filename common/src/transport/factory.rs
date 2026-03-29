use crate::config::{DeviceConfig, group::{RemoteTransportConfig, DeviceGroupConfig}};
use crate::transport::publisher::{RemotePublisher, PublisherType};
use crate::transport::subscriber::{RemoteSubscriber, SubscriberType};
use crate::transport::mqtt::MqttPublisher;
use crate::transport::kafka::KafkaPublisher;
use crate::transport::kafka_sub::KafkaSubscriber;
use crate::transport::zmq_sub::ZmqSubscriber;
use anyhow::Result;

pub struct PublisherFactory;

impl PublisherFactory {
    pub fn create_from_remote_transport(rt: &RemoteTransportConfig) -> Result<Box<dyn RemotePublisher>> {
        Self::create_from_remote_transport_with_ids(rt, None, None, None, None)
    }

    pub fn create_from_remote_transport_with_ids(
        rt: &RemoteTransportConfig,
        tenant_id: Option<String>,
        org_id: Option<String>,
        site_id: Option<String>,
        namespace_id: Option<String>,
    ) -> Result<Box<dyn RemotePublisher>> {
        match rt.r#type.as_str() {
            "mqtt" => {
                let mqtt_config = crate::config::MqttConfig {
                    enabled: true,
                    broker_address: rt.broker.clone().unwrap_or_default(),
                    client_id: rt.client_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                    username: rt.username.clone(),
                    password: rt.password.clone(),
                    tenant_id,
                    org_id,
                    site_id,
                    namespace_id,
                    ..Default::default()
                };
                Ok(Box::new(MqttPublisher::new(&mqtt_config)))
            }
            "kafka" => {
                let kafka_config = crate::config::KafkaConfig {
                    enabled: true,
                    brokers: rt.brokers.clone().unwrap_or_default(),
                    username: rt.username.clone(),
                    password: rt.password.clone(),
                    ..Default::default()
                };
                Ok(Box::new(KafkaPublisher::new(&kafka_config)))
            }
            _ => {
                anyhow::bail!("Unsupported remote transport type: {}", rt.r#type);
            }
        }
    }

    pub fn create_from_group_config(group_config: &DeviceGroupConfig) -> Result<Box<dyn RemotePublisher>> {
        Self::create_from_remote_transport_with_ids(
            &group_config.remote_transport,
            Some(group_config.tenant_id.clone()),
            Some(group_config.org_id.clone()),
            Some(group_config.site_id.clone()),
            Some(group_config.namespace_id.clone()),
        )
    }

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
