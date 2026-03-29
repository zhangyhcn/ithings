use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub enabled: bool,
    pub broker_address: String,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub topic_prefix: String,
    pub qos: u8,
    #[serde(default)]
    pub tenant_id: Option<String>,
    #[serde(default)]
    pub org_id: Option<String>,
    #[serde(default)]
    pub site_id: Option<String>,
    #[serde(default)]
    pub namespace_id: Option<String>,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            broker_address: "tcp://localhost:1883".to_string(),
            client_id: "device-client".to_string(),
            username: None,
            password: None,
            topic_prefix: "devices".to_string(),
            qos: 1,
            tenant_id: None,
            org_id: None,
            site_id: None,
            namespace_id: None,
        }
    }
}
