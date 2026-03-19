use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set, prelude::Json};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use kube::{Api, Resource, api::{PostParams, Patch, PatchParams}};
use k8s_openapi::api::apps::v1 as apps;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::entity::{ControllerEntity, ControllerColumn, ControllerModel as Model};
use crate::utils::{AppError, K8sClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateControllerRequest {
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub kind: String,
    pub version: String,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateControllerRequest {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerResponse {
    pub id: String,
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub kind: String,
    pub version: String,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
    pub status: String,
    pub k8s_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for ControllerResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            namespace_id: model.namespace_id.to_string(),
            name: model.name,
            slug: model.slug,
            kind: model.kind,
            version: model.version,
            description: model.description,
            yaml: model.yaml,
            status: model.status,
            k8s_name: model.k8s_name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct ControllerService {
    db: DatabaseConnection,
}

impl ControllerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateControllerRequest) -> Result<ControllerResponse, AppError> {
        let namespace_id = Uuid::parse_str(&req.namespace_id)
            .map_err(|e| AppError::BadRequest(format!("Invalid namespace ID: {}", e.to_string())))?;

        let existing = ControllerEntity::find()
            .filter(ControllerColumn::NamespaceId.eq(namespace_id))
            .filter(ControllerColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Controller with this slug already exists in the namespace".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::controller::ActiveModel {
            id: Set(Uuid::new_v4()),
            namespace_id: Set(namespace_id),
            name: Set(req.name),
            slug: Set(req.slug),
            kind: Set(req.kind),
            version: Set(req.version),
            description: Set(req.description),
            yaml: Set(req.yaml),
            status: Set("draft".to_string()),
            k8s_name: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let controller = active_model.insert(&self.db).await?;
        Ok(controller.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<ControllerResponse, AppError> {
        let model = ControllerEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Controller not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_namespace(
        &self,
        namespace_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<ControllerResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = ControllerEntity::find()
            .filter(ControllerColumn::NamespaceId.eq(namespace_id))
            .paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<ControllerResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = ControllerEntity::find().paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateControllerRequest,
    ) -> Result<ControllerResponse, AppError> {
        let mut model = ControllerEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Controller not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(kind) = req.kind {
            model.kind = Set(kind);
        }
        if let Some(version) = req.version {
            model.version = Set(version);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(yaml) = req.yaml {
            model.yaml = Set(Some(yaml));
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn publish(&self, id: Uuid) -> Result<ControllerResponse, AppError> {
        let model = ControllerEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Controller not found".to_string()))?;

        let yaml_data = model.yaml.clone()
            .ok_or(AppError::BadRequest("Controller yaml configuration is required".to_string()))?;

        let k8s_client = K8sClient::new().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create K8s client: {}", e)))?;

        let k8s_name = self.build_and_apply_controller(&yaml_data, &k8s_client).await?;
        
        let mut active_model = model.into_active_model();
        active_model.status = Set("published".to_string());
        active_model.k8s_name = Set(Some(k8s_name));
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    async fn build_and_apply_controller(
        &self,
        yaml_data: &JsonValue,
        k8s_client: &K8sClient,
    ) -> Result<String, AppError> {
        let deployment: apps::Deployment = if yaml_data.is_string() {
            let yaml_str = yaml_data.as_str().unwrap();
            serde_json::from_str(yaml_str)
                .map_err(|e| AppError::BadRequest(format!("Invalid Deployment format: {}", e)))?
        } else {
            let json_str = serde_json::to_string(yaml_data)
                .map_err(|e| AppError::BadRequest(format!("Invalid yaml format: {}", e)))?;
            serde_json::from_str(&json_str)
                .map_err(|e| AppError::BadRequest(format!("Invalid Deployment format: {}", e)))?
        };

        let name = deployment.metadata.name.clone().unwrap_or_default();
        let namespace = deployment.metadata.namespace.clone().unwrap_or_else(|| "default".to_string());

        k8s_client.apply_deployment(&namespace, deployment).await
            .map_err(|e| AppError::BadRequest(format!("Failed to publish Controller to K8s: {}", e)))?;

        Ok(name)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = ControllerEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Controller not found".to_string()))?;

        if let Some(k8s_name) = &model.k8s_name {
            if let Some(yaml_data) = &model.yaml {
                let namespace = self.extract_namespace_from_yaml(yaml_data);
                match K8sClient::new().await {
                    Ok(k8s_client) => {
                        if let Err(e) = k8s_client.delete_deployment(&namespace, k8s_name).await {
                            tracing::warn!("Failed to delete Controller from K8s: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to create K8s client: {}", e);
                    }
                }
            }
        }

        let result = ControllerEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Controller not found".to_string()));
        }
        Ok(())
    }

    fn extract_namespace_from_yaml(&self, yaml_data: &JsonValue) -> String {
        if yaml_data.is_string() {
            if let Ok(deployment) = serde_json::from_str::<apps::Deployment>(yaml_data.as_str().unwrap()) {
                return deployment.metadata.namespace.unwrap_or_else(|| "default".to_string());
            }
        } else if let Some(namespace) = yaml_data.get("metadata").and_then(|m| m.get("namespace")).and_then(|n| n.as_str()) {
            return namespace.to_string();
        }
        "default".to_string()
    }
}
