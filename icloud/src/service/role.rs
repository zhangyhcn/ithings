use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, prelude::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{RoleEntity, RoleColumn, RoleModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub permissions: Option<Json>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Json>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<super::role::Model> for RoleResponse {
    fn from(model: super::role::Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            name: model.name,
            slug: model.slug,
            description: model.description,
            permissions: model.permissions,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct RoleService {
    db: DatabaseConnection,
}

impl RoleService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateRoleRequest) -> Result<RoleResponse, AppError> {
        // 检查同一租户下slug是否已存在
        let existing = RoleEntity::find()
            .filter(RoleColumn::TenantId.eq(req.tenant_id))
            .filter(RoleColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Role with this slug already exists".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::role::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(req.tenant_id),
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            permissions: Set(req.permissions.unwrap_or_else(|| Json::Array(vec![]))),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(RoleResponse::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<RoleResponse, AppError> {
        let model = RoleEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Role not found".to_string()))?;

        Ok(RoleResponse::from(model))
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<RoleResponse>, AppError> {
        let models = RoleEntity::find()
            .filter(RoleColumn::TenantId.eq(tenant_id))
            .filter(RoleColumn::Status.eq("active"))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(RoleResponse::from).collect())
    }

    pub async fn update(&self, id: Uuid, req: UpdateRoleRequest) -> Result<RoleResponse, AppError> {
        let model = RoleEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Role not found".to_string()))?;

        let mut active_model: crate::entity::role::ActiveModel = model.into();

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(slug) = req.slug {
            // 检查同一租户下slug是否已被其他角色使用
            let role = RoleEntity::find_by_id(id)
                .one(&self.db)
                .await?
                .ok_or(AppError::NotFound("Role not found".to_string()))?;

            let existing = RoleEntity::find()
                .filter(RoleColumn::TenantId.eq(role.tenant_id))
                .filter(RoleColumn::Slug.eq(&slug))
                .filter(RoleColumn::Id.ne(id))
                .one(&self.db)
                .await?;

            if existing.is_some() {
                return Err(AppError::BadRequest("Role with this slug already exists".to_string()));
            }
            active_model.slug = Set(slug);
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(permissions) = req.permissions {
            active_model.permissions = Set(permissions);
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        let model = active_model.update(&self.db).await?;

        Ok(RoleResponse::from(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = RoleEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Role not found".to_string()));
        }
        Ok(())
    }
}
