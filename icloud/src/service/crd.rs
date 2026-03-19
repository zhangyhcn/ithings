use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set, prelude::Json};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::entity::{CrdEntity, CrdColumn, CrdModel as Model};
use crate::utils::{AppError, K8sClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCrdRequest {
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub group: String,
    pub version: String,
    pub kind: String,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCrdRequest {
    pub name: Option<String>,
    pub group: Option<String>,
    pub version: Option<String>,
    pub kind: Option<String>,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrdResponse {
    pub id: String,
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub group: String,
    pub version: String,
    pub kind: String,
    pub description: Option<String>,
    pub yaml: Option<JsonValue>,
    pub status: String,
    pub k8s_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for CrdResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            namespace_id: model.namespace_id.to_string(),
            name: model.name,
            slug: model.slug,
            group: model.group,
            version: model.version,
            kind: model.kind,
            description: model.description,
            yaml: model.yaml,
            status: model.status,
            k8s_name: model.k8s_name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct CrdService {
    db: DatabaseConnection,
}

impl CrdService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateCrdRequest) -> Result<CrdResponse, AppError> {
        let namespace_id = Uuid::parse_str(&req.namespace_id)
            .map_err(|e| AppError::BadRequest(format!("Invalid namespace ID: {}", e.to_string())))?;

        let existing = CrdEntity::find()
            .filter(CrdColumn::NamespaceId.eq(namespace_id))
            .filter(CrdColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("CRD with this slug already exists in the namespace".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::crd::ActiveModel {
            id: Set(Uuid::new_v4()),
            namespace_id: Set(namespace_id),
            name: Set(req.name),
            slug: Set(req.slug),
            group: Set(req.group),
            version: Set(req.version),
            kind: Set(req.kind),
            description: Set(req.description),
            yaml: Set(req.yaml),
            status: Set("draft".to_string()),
            k8s_name: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let crd = active_model.insert(&self.db).await?;
        Ok(crd.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<CrdResponse, AppError> {
        let model = CrdEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("CRD not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_namespace(
        &self,
        namespace_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<CrdResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = CrdEntity::find()
            .filter(CrdColumn::NamespaceId.eq(namespace_id))
            .paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<CrdResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = CrdEntity::find().paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateCrdRequest,
    ) -> Result<CrdResponse, AppError> {
        let mut model = CrdEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("CRD not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(group) = req.group {
            model.group = Set(group);
        }
        if let Some(version) = req.version {
            model.version = Set(version);
        }
        if let Some(kind) = req.kind {
            model.kind = Set(kind);
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

    pub async fn publish(&self, id: Uuid) -> Result<CrdResponse, AppError> {
        let model = CrdEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("CRD not found".to_string()))?;

        let yaml_data = model.yaml.clone()
            .ok_or(AppError::BadRequest("CRD yaml configuration is required".to_string()))?;

        let k8s_crd = self.build_crd_from_yaml(&yaml_data)?;

        let k8s_client = K8sClient::new().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create K8s client: {}", e)))?;

        let crd_name = k8s_crd.metadata.name.clone().unwrap_or_default();
        
        match k8s_client.apply_crd(k8s_crd).await {
            Ok(_) => {
                let mut active_model = model.into_active_model();
                active_model.status = Set("published".to_string());
                active_model.k8s_name = Set(Some(crd_name));
                active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                let updated = active_model.update(&self.db).await?;
                Ok(updated.into())
            }
            Err(e) => {
                Err(AppError::BadRequest(format!("Failed to publish CRD to K8s: {}", e)))
            }
        }
    }

    fn build_crd_from_yaml(
        &self,
        yaml_data: &JsonValue,
    ) -> Result<apiextensions::CustomResourceDefinition, AppError> {
        let crd: apiextensions::CustomResourceDefinition = if yaml_data.is_string() {
            let yaml_str = yaml_data.as_str().unwrap();
            serde_json::from_str(yaml_str)
                .map_err(|e| AppError::BadRequest(format!("Invalid CRD format: {}", e)))?
        } else {
            let crd_json = serde_json::to_string(yaml_data)
                .map_err(|e| AppError::BadRequest(format!("Invalid yaml format: {}", e)))?;
            serde_json::from_str(&crd_json)
                .map_err(|e| AppError::BadRequest(format!("Invalid CRD format: {}", e)))?
        };

        Ok(crd)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = CrdEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("CRD not found".to_string()))?;

        if let Some(k8s_name) = &model.k8s_name {
            match K8sClient::new().await {
                Ok(k8s_client) => {
                    if let Err(e) = k8s_client.delete_crd(k8s_name).await {
                        tracing::warn!("Failed to delete CRD from K8s: {}", e);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to create K8s client: {}", e);
                }
            }
        }

        let result = CrdEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("CRD not found".to_string()));
        }
        Ok(())
    }
}
