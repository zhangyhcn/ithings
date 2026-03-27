use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{Api, Client, Config};
use handlebars::Handlebars;
use serde::Serialize;
use std::fs;
use uuid::Uuid;

use crate::utils::AppError;

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

    pub async fn create_or_update_deployment(
        &self,
        name: &str,
        device_images: &[String],
        driver_image: &str,
        node_labels: &std::collections::HashMap<String, String>,
        instances: &[serde_json::Value],
        group_id: Uuid,
    ) -> Result<(), AppError> {
        let configmap_name = format!("{}-config", name);
        
        if !instances.is_empty() {
            self.create_or_update_configmap(&configmap_name, group_id, instances).await?;
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
        instances: &[serde_json::Value],
    ) -> Result<(), AppError> {
        use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

        let configmaps: Api<ConfigMap> = Api::namespaced(self.client.clone(), &self.namespace);

        let mut data = std::collections::BTreeMap::new();
        
        let group_config = serde_json::json!({
            "tenant_id": "",
            "org_id": "",
            "site_id": "",
            "namespace_id": "",
            "remote_transport": {
                "type": "mqtt",
                "broker": null,
                "brokers": null,
                "username": null,
                "password": null,
                "client_id": null
            },
            "devices": instances
        });
        
        let driver_config = serde_json::json!({
             "driver_name": "modbus-driver",
             "driver_type": "modbus",
             "device_instance_id": "modbus-driver-group",
             "poll_interval_ms": 1000,
             "zmq": {
                 "enabled": true,
                 "publisher_address": "tcp://*:5556",
                 "topic": "modbus/data"
             },
             "logging": {
                 "level": "info",
                 "format": "json"
             },
             "custom": {}
         });
        
        let json_str = serde_json::to_string_pretty(&group_config).unwrap_or_default();
        data.insert("config.json".to_string(), json_str);
        
        let driver_json_str = serde_json::to_string_pretty(&driver_config).unwrap_or_default();
        data.insert("driver-config.json".to_string(), driver_json_str);

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
