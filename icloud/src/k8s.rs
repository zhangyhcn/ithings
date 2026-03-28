use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{Api, Client, Config};
use handlebars::Handlebars;
use serde::Serialize;
use std::fs;
use uuid::Uuid;

use crate::utils::AppError;
use common::config::{DeviceGroupPublishConfig, ConfigMapPublishRequest};
use common::config::group::DeviceInGroupConfig;

#[derive(Serialize)]
struct DeviceContainerContext {
    name: String,
    image: String,
}

#[derive(Serialize)]
struct DeploymentTemplateContext {
    deployment_name: String,
    configmap_name: String,
    namespace: String,
    group_id: String,
    device_containers: Vec<DeviceContainerContext>,
    driver_image: String,
    node_selector: Vec<NodeSelectorEntry>,
}

#[derive(Serialize)]
struct NodeSelectorEntry {
    key: String,
    value: String,
}

pub struct K8sClient {
    client: Client,
    namespace: String,
    deployment_template_path: String,
}

impl K8sClient {
    pub async fn new(namespace: Option<String>) -> Result<Self, AppError> {
        let config = Config::infer()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to load kubeconfig: {}", e)))?;
        
        let client = Client::try_from(config)
            .map_err(|e| AppError::InternalServerError(format!("Failed to create k8s client: {}", e)))?;
        
        let namespace = namespace.unwrap_or_else(|| "default".to_string());
        let deployment_template_path = "/root/source/rust/ithings/icloud/templates/device-group.yaml".to_string();
        
        Ok(Self { 
            client, 
            namespace, 
            deployment_template_path,
        })
    }

    fn default_driver_config() -> common::config::driver::DriverConfig {
        common::config::driver::DriverConfig {
            driver_name: "modbus-driver".to_string(),
            driver_type: "modbus".to_string(),
            device_instance_id: "modbus-driver-group".to_string(),
            poll_interval_ms: 1000,
            zmq: Default::default(),
            logging: Default::default(),
            custom: Default::default(),
        }
    }

    pub async fn create_or_update_deployment(
        &self,
        name: &str,
        device_images: &[String],
        driver_image: &str,
        node_labels: &std::collections::HashMap<String, String>,
        instances: &[serde_json::Value],
        group_id: Uuid,
        tenant_id: Uuid,
        org_id: String,
        site_id: String,
        namespace_id: Option<String>,
    ) -> Result<(), AppError> {
        let configmap_name = format!("{}-config", name);
        
        if !instances.is_empty() {
            self.create_or_update_configmap(&configmap_name, group_id, tenant_id, org_id, site_id, namespace_id.clone(), instances).await?;
        }

        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.namespace);
        
        let template_content = fs::read_to_string(&self.deployment_template_path)
            .map_err(|e| AppError::InternalServerError(format!("Failed to read deployment template: {}", e)))?;
        
        let node_selector: Vec<NodeSelectorEntry> = node_labels
            .iter()
            .map(|(k, v)| NodeSelectorEntry {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();
        
        let device_containers: Vec<DeviceContainerContext> = device_images
            .iter()
            .enumerate()
            .map(|(idx, img)| DeviceContainerContext {
                name: format!("device-{}", idx),
                image: img.clone(),
            })
            .collect();
        
        let context = DeploymentTemplateContext {
            deployment_name: name.to_string(),
            configmap_name: configmap_name.clone(),
            namespace: self.namespace.clone(),
            group_id: group_id.to_string(),
            device_containers,
            driver_image: driver_image.to_string(),
            node_selector,
        };
        
        let mut hb = Handlebars::new();
        hb.set_strict_mode(false);
        
        let rendered = hb.render_template(&template_content, &context)
            .map_err(|e| AppError::InternalServerError(format!("Failed to render deployment template: {}", e)))?;
        
        let deployment: Deployment = serde_yaml::from_str(&rendered)
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse deployment YAML: {}", e)))?;
        
        let existing = deployments.get(name).await;
        
        match existing {
            Ok(_) => {
                deployments
                    .replace(name, &Default::default(), &deployment)
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("Failed to update deployment: {}", e)))?;
                tracing::info!("Updated deployment: {}", name);
            }
            Err(_) => {
                deployments
                    .create(&Default::default(), &deployment)
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("Failed to create deployment: {}", e)))?;
                tracing::info!("Created deployment: {}", name);
            }
        }

        Ok(())
    }

    async fn create_or_update_configmap(
        &self,
        configmap_name: &str,
        group_id: Uuid,
        tenant_id: Uuid,
        org_id: String,
        site_id: String,
        namespace_id: Option<String>,
        instances: &[serde_json::Value],
    ) -> Result<(), AppError> {
        use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

        let configmaps: Api<ConfigMap> = Api::namespaced(self.client.clone(), &self.namespace);

        let mut data = std::collections::BTreeMap::new();
        
        let mut remote_transport = serde_json::json!({
            "type": "mqtt",
            "broker": null,
            "brokers": null,
            "username": null,
            "password": null,
            "client_id": null
        });

        let tenant_id_str = tenant_id.to_string();
        if let Some(tenant) = crate::service::cache::GlobalCache::get_tenant(tenant_id).await {
            if let Some(config) = tenant.config {
                if let serde_json::Value::Object(obj) = config {
                    if let Some(rt) = obj.get("remote_transport") {
                        remote_transport = rt.clone();
                    }
                }
            }
        }
        
        let namespace_id_str = namespace_id.unwrap_or_default();
        
        // 解析设备实例为结构化配置
        let devices: Vec<common::config::DeviceInGroupConfig> = instances
            .iter()
            .filter_map(|inst| serde_json::from_value(inst.clone()).ok())
            .collect();
        
        // 构建 DeviceGroupPublishConfig
        let group_config = DeviceGroupPublishConfig {
            tenant_id: tenant_id_str,
            org_id,
            site_id,
            namespace_id: namespace_id_str,
            remote_transport,
            group_id: group_id.to_string(),
            devices,
        };
        
        // 从第一个设备实例提取 driver 配置
        let driver_config = if let Some(first_instance) = instances.first() {
            if let Some(driver) = first_instance.get("driver") {
                // 如果 driver 是字符串，先解析为 JSON 对象
                let base_config = if let serde_json::Value::String(ref s) = driver {
                    serde_json::from_str::<serde_json::Value>(s.trim())
                        .unwrap_or_else(|_| driver.clone())
                } else {
                    driver.clone()
                };
                // 解析为 DeviceInGroupConfig
                serde_json::from_value(base_config)
                    .unwrap_or_else(|_| Self::default_driver_config())
            } else {
                Self::default_driver_config()
            }
        } else {
            Self::default_driver_config()
        };
        
        // 构建发布请求并校验
        let publish_request = ConfigMapPublishRequest {
            config_json: group_config,
            driver_config_json: driver_config,
        };
        
        publish_request.validate()
            .map_err(|e| AppError::BadRequest(format!("Invalid publish config: {}", e)))?;
        
        let json_str = serde_json::to_string_pretty(&publish_request.config_json)
            .map_err(|e| AppError::InternalServerError(format!("Failed to serialize config.json: {}", e)))?;
        data.insert("config.json".to_string(), json_str);

        let configmap = ConfigMap {
            metadata: ObjectMeta {
                name: Some(configmap_name.to_string()),
                namespace: Some(self.namespace.clone()),
                labels: Some({
                    let mut labels = std::collections::BTreeMap::new();
                    labels.insert("device-group".to_string(), group_id.to_string());
                    labels.insert("app.kubernetes.io/managed-by".to_string(), "ithings".to_string());
                    labels
                }),
                ..Default::default()
            },
            data: Some(data),
            ..Default::default()
        };

        let existing = configmaps.get(configmap_name).await;

        match existing {
            Ok(_) => {
                configmaps
                    .replace(configmap_name, &Default::default(), &configmap)
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("Failed to update configmap: {}", e)))?;
                tracing::info!("Updated configmap: {}", configmap_name);
            }
            Err(_) => {
                configmaps
                    .create(&Default::default(), &configmap)
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("Failed to create configmap: {}", e)))?;
                tracing::info!("Created configmap: {}", configmap_name);
            }
        }

        Ok(())
    }

    pub async fn delete_deployment(&self, name: &str) -> Result<(), AppError> {
        let configmap_name = format!("{}-config", name);
        
        let configmaps: Api<ConfigMap> = Api::namespaced(self.client.clone(), &self.namespace);
        match configmaps.delete(&configmap_name, &Default::default()).await {
            Ok(_) => {
                tracing::info!("Deleted configmap: {}", configmap_name);
            }
            Err(e) => {
                if e.to_string().contains("not found") {
                    tracing::info!("ConfigMap {} not found, skipping deletion", configmap_name);
                } else {
                    tracing::warn!("Failed to delete configmap {}: {}", configmap_name, e);
                }
            }
        }

        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.namespace);
        
        match deployments.delete(name, &Default::default()).await {
            Ok(_) => {
                tracing::info!("Deleted deployment: {}", name);
                Ok(())
            }
            Err(e) => {
                if e.to_string().contains("not found") {
                    tracing::info!("Deployment {} not found, skipping deletion", name);
                    Ok(())
                } else {
                    Err(AppError::InternalServerError(format!("Failed to delete deployment: {}", e)))
                }
            }
        }
    }

    pub async fn get_deployment_status(&self, name: &str) -> Result<Option<String>, AppError> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.namespace);
        
        match deployments.get(name).await {
            Ok(deployment) => {
                let status = deployment
                    .status
                    .and_then(|s| s.conditions.and_then(|c| c.into_iter().next().map(|cond| cond.type_)));
                Ok(status)
            }
            Err(_) => Ok(None),
        }
    }
}
