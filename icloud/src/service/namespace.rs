use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{NamespaceEntity, NamespaceColumn, NamespaceModel as Model, TenantEntity, TenantColumn};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNamespaceRequest {
    pub site_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub namespace_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNamespaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub namespace_type: Option<String>,
    pub config: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceResponse {
    pub id: String,
    pub tenant_id: String,
    pub site_id: String,
    pub name: String,
    pub slug: String,
    description: Option<String>,
    namespace_type: String,
    config: JsonValue,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<Model> for NamespaceResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            site_id: model.site_id.to_string(),
            name: model.name,
            slug: model.slug,
            description: model.description,
            namespace_type: model.namespace_type,
            config: model.config,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct NamespaceService {
    db: DatabaseConnection,
}

impl NamespaceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, tenant_id: Uuid, req: CreateNamespaceRequest) -> Result<NamespaceResponse, AppError> {
        let _tenant = TenantEntity::find_by_id(tenant_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?;

        let existing = NamespaceEntity::find()
            .filter(NamespaceColumn::TenantId.eq(tenant_id))
            .filter(NamespaceColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Namespace with this slug already exists".to_string()));
        }

        let site_id = Uuid::parse_str(&req.site_id)
            .map_err(|_| AppError::BadRequest("Invalid site_id".to_string()))?;

        let site = crate::entity::site::Entity::find_by_id(site_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Site not found".to_string()))?;

        if site.tenant_id != tenant_id {
            return Err(AppError::BadRequest("Site does not belong to this tenant".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::namespace::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            site_id: Set(site_id),
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            namespace_type: Set(req.namespace_type.unwrap_or_else(|| "default".to_string())),
            config: Set(JsonValue::Object(serde_json::Map::new())),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<NamespaceResponse, AppError> {
        let model = NamespaceEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Namespace not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_all(
        &self,
    ) -> Result<Vec<NamespaceResponse>, AppError> {
        let models = NamespaceEntity::find()
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn list_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<NamespaceResponse>, AppError> {
        let models = NamespaceEntity::find()
            .filter(NamespaceColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn list_by_tenant_slug(
        &self,
        tenant_slug: &str,
    ) -> Result<Vec<NamespaceResponse>, AppError> {
        let tenant = TenantEntity::find()
            .filter(TenantColumn::Slug.eq(tenant_slug))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Tenant not found".to_string()))?;

        let models = NamespaceEntity::find()
            .filter(NamespaceColumn::TenantId.eq(tenant.id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateNamespaceRequest,
    ) -> Result<NamespaceResponse, AppError> {
        let mut model = NamespaceEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Namespace not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(namespace_type) = req.namespace_type {
            model.namespace_type = Set(namespace_type);
        }
        if let Some(config) = req.config {
            model.config = Set(config);
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = NamespaceEntity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Namespace not found".to_string()));
        }
        Ok(())
    }
}
