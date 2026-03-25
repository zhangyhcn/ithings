use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use kube::{Api, Client, Config, config::KubeConfigOptions, api::PostParams};
use k8s_openapi::api::core::v1::Node;

use crate::entity::{NodeEntity, NodeColumn, NodeModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLabelsRequest {
    pub labels: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub labels: JsonValue,
    pub roles: JsonValue,
    pub internal_ip: Option<String>,
    pub os: Option<String>,
    pub kernel_version: Option<String>,
    pub container_runtime: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for NodeResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            status: model.status,
            labels: model.labels,
            roles: model.roles,
            internal_ip: model.internal_ip,
            os: model.os,
            kernel_version: model.kernel_version,
            container_runtime: model.container_runtime,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct NodeService {
    db: DatabaseConnection,
}

impl NodeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn sync_from_k8s(&self) -> Result<Vec<NodeResponse>, AppError> {
        let client = Self::get_k8s_client().await?;
        let nodes: Api<Node> = Api::all(client);
        
        let node_list = nodes.list(&Default::default()).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to list K8s nodes: {}", e)))?;
        
        let mut responses = Vec::new();
        
        for k8s_node in node_list.items {
            let node_name = k8s_node.metadata.name.clone().unwrap_or_default();
            
            let status = k8s_node.status.clone().unwrap_or_default();
            let conditions = status.conditions.unwrap_or_default();
            let node_status = conditions
                .iter()
                .find(|c| c.type_ == "Ready")
                .map(|c| if c.status == "True" { "Ready" } else { "NotReady" })
                .unwrap_or("Unknown")
                .to_string();
            
            let mut labels: serde_json::Map<String, JsonValue> = serde_json::Map::new();
            if let Some(node_labels) = k8s_node.metadata.labels {
                for (k, v) in node_labels {
                    labels.insert(k, JsonValue::String(v));
                }
            }
            
            let mut roles: Vec<String> = Vec::new();
            for (key, _) in labels.iter() {
                if key.starts_with("node-role.kubernetes.io/") {
                    if let Some(role) = key.strip_prefix("node-role.kubernetes.io/") {
                        roles.push(role.to_string());
                    }
                }
            }
            if roles.is_empty() {
                for (key, _) in labels.iter() {
                    if key == "kubernetes.io/role" || key.starts_with("node.kubernetes.io/role") {
                        if let Some(value) = labels.get(key).and_then(|v| v.as_str()) {
                            roles.push(value.to_string());
                        }
                    }
                }
            }
            
            let internal_ip = status.addresses.unwrap_or_default()
                .iter()
                .find(|addr| addr.type_ == "InternalIP")
                .map(|addr| addr.address.clone());
            
            let node_info = status.node_info.unwrap_or_default();
            let os: Option<String> = if node_info.operating_system.is_empty() { None } else { Some(node_info.operating_system.clone()) };
            let kernel_version: Option<String> = if node_info.kernel_version.is_empty() { None } else { Some(node_info.kernel_version.clone()) };
            let container_runtime = node_info.container_runtime_version.clone();
            
            let existing = NodeEntity::find()
                .filter(NodeColumn::Name.eq(&node_name))
                .one(&self.db)
                .await?;
            
            let response = if let Some(model) = existing {
                let mut active_model = crate::entity::node::ActiveModel {
                    id: sea_orm::ActiveValue::Set(model.id),
                    name: sea_orm::ActiveValue::Set(node_name.clone()),
                    status: sea_orm::ActiveValue::Set(node_status),
                    labels: sea_orm::ActiveValue::Set(JsonValue::Object(labels.clone())),
                    roles: sea_orm::ActiveValue::Set(serde_json::to_value(&roles).unwrap_or(JsonValue::Array(vec![]))),
                    internal_ip: sea_orm::ActiveValue::Set(internal_ip.clone()),
                    os: sea_orm::ActiveValue::Set(os.clone()),
                    kernel_version: sea_orm::ActiveValue::Set(kernel_version.clone()),
                    container_runtime: sea_orm::ActiveValue::Set(Some(container_runtime.clone())),
                    created_at: sea_orm::ActiveValue::Set(model.created_at),
                    updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
                };
                
                let updated = active_model.update(&self.db).await?;
                updated.into()
            } else {
                let active_model = crate::entity::node::ActiveModel {
                    id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
                    name: sea_orm::ActiveValue::Set(node_name.clone()),
                    status: sea_orm::ActiveValue::Set(node_status),
                    labels: sea_orm::ActiveValue::Set(JsonValue::Object(labels)),
                    roles: sea_orm::ActiveValue::Set(serde_json::to_value(&roles).unwrap_or(JsonValue::Array(vec![]))),
                    internal_ip: sea_orm::ActiveValue::Set(internal_ip),
                    os: sea_orm::ActiveValue::Set(os),
                    kernel_version: sea_orm::ActiveValue::Set(kernel_version),
                    container_runtime: sea_orm::ActiveValue::Set(Some(container_runtime)),
                    created_at: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
                };
                
                let model = active_model.insert(&self.db).await?;
                model.into()
            };
            
            responses.push(response);
        }
        
        Ok(responses)
    }

    async fn get_k8s_client() -> Result<Client, AppError> {
        let config = if let Ok(_kubeconfig_path) = std::env::var("KUBECONFIG") {
            Config::infer()
                .await
                .map_err(|e| AppError::InternalServerError(format!("Failed to infer K8s config: {}", e)))?
        } else {
            Config::infer()
                .await
                .map_err(|e| AppError::InternalServerError(format!("Failed to infer K8s config: {}", e)))?
        };
        
        Client::try_from(config)
            .map_err(|e| AppError::InternalServerError(format!("Failed to create K8s client: {}", e)))
    }

    pub async fn list_all(&self) -> Result<Vec<NodeResponse>, AppError> {
        let models = NodeEntity::find()
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<NodeResponse, AppError> {
        let model = NodeEntity::find()
            .filter(NodeColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Node not found".to_string())),
        }
    }

    pub async fn update_labels(&self, id: Uuid, req: UpdateLabelsRequest) -> Result<NodeResponse, AppError> {
        let model = NodeEntity::find()
            .filter(NodeColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::NotFound("Node not found".to_string()));
        };

        let node_name = model.name.clone();
        
        let client = Self::get_k8s_client().await?;
        let nodes: Api<Node> = Api::all(client);
        
        let mut k8s_node = nodes.get(&node_name).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get K8s node: {}", e)))?;
        
        let mut labels = k8s_node.metadata.labels.clone().unwrap_or_default();
        
        if let Some(obj) = req.labels.as_object() {
            for (key, value) in obj {
                if let Some(v) = value.as_str() {
                    labels.insert(key.clone(), v.to_string());
                }
            }
        }
        
        k8s_node.metadata.labels = Some(labels);
        // resourceVersion已经从get请求中获取到了，保持最新值即可满足K8s更新要求
        
        nodes.replace(&node_name, &PostParams::default(), &k8s_node).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to update K8s node labels: {}", e)))?;
        
        let mut active_model = crate::entity::node::ActiveModel {
            id: sea_orm::ActiveValue::Set(model.id),
            name: sea_orm::ActiveValue::Set(model.name),
            status: sea_orm::ActiveValue::Set(model.status),
            labels: sea_orm::ActiveValue::Set(req.labels),
            roles: sea_orm::ActiveValue::Set(model.roles),
            internal_ip: sea_orm::ActiveValue::Set(model.internal_ip),
            os: sea_orm::ActiveValue::Set(model.os),
            kernel_version: sea_orm::ActiveValue::Set(model.kernel_version),
            container_runtime: sea_orm::ActiveValue::Set(model.container_runtime),
            created_at: sea_orm::ActiveValue::Set(model.created_at),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };
        
        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }
}
