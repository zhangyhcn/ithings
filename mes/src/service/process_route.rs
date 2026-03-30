use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{ProcessRouteEntity, ProcessRouteColumn, ProcessRouteModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProcessRouteRequest {
    pub product_id: Uuid,
    pub route_name: String,
    pub version: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProcessRouteRequest {
    pub route_name: Option<String>,
    pub version: Option<String>,
    pub status: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessRouteResponse {
    pub id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub route_name: String,
    pub version: String,
    pub status: String,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for ProcessRouteResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            product_id: model.product_id.to_string(),
            route_name: model.route_name,
            version: model.version,
            status: model.status,
            is_default: model.is_default,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct ProcessRouteService {
    db: Arc<DatabaseConnection>,
}

impl ProcessRouteService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateProcessRouteRequest,
    ) -> Result<ProcessRouteResponse, AppError> {
        let now = Utc::now().naive_utc();
        
        // 如果设置为默认，先取消该产品的其他默认工艺路线
        if req.is_default.unwrap_or(false) {
            self.unset_default_routes(tenant_id, req.product_id).await?;
        }
        
        let active_model = crate::entity::process_route::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            product_id: Set(req.product_id),
            route_name: Set(req.route_name),
            version: Set(req.version.unwrap_or_else(|| "1.0".to_string())),
            status: Set("draft".to_string()),
            is_default: Set(req.is_default.unwrap_or(false)),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(self.db.as_ref()).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<ProcessRouteResponse>, AppError> {
        let models = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::TenantId.eq(tenant_id))
            .order_by_desc(ProcessRouteColumn::CreatedAt)
            .all(self.db.as_ref())
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn list_by_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<Vec<ProcessRouteResponse>, AppError> {
        let models = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::TenantId.eq(tenant_id))
            .filter(ProcessRouteColumn::ProductId.eq(product_id))
            .order_by_desc(ProcessRouteColumn::CreatedAt)
            .all(self.db.as_ref())
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<ProcessRouteResponse, AppError> {
        let model = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::not_found("ProcessRoute not found".to_string())),
        }
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateProcessRouteRequest,
    ) -> Result<ProcessRouteResponse, AppError> {
        let model = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("ProcessRoute not found".to_string()));
        };

        // 如果设置为默认，先取消该产品的其他默认工艺路线
        if req.is_default.unwrap_or(false) {
            self.unset_default_routes(model.tenant_id, model.product_id).await?;
        }

        let mut active_model = crate::entity::process_route::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            product_id: Set(model.product_id),
            route_name: Set(model.route_name),
            version: Set(model.version),
            status: Set(model.status),
            is_default: Set(model.is_default),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        if let Some(route_name) = req.route_name {
            active_model.route_name = Set(route_name);
        }
        if let Some(version) = req.version {
            active_model.version = Set(version);
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }
        if let Some(is_default) = req.is_default {
            active_model.is_default = Set(is_default);
        }

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("ProcessRoute not found".to_string()));
        };

        // 软删除：将状态改为deleted
        let mut active_model = crate::entity::process_route::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            product_id: Set(model.product_id),
            route_name: Set(model.route_name),
            version: Set(model.version),
            status: Set("deleted".to_string()),
            is_default: Set(model.is_default),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        active_model.update(self.db.as_ref()).await?;
        Ok(())
    }

    pub async fn set_as_default(&self, id: Uuid) -> Result<ProcessRouteResponse, AppError> {
        let model = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("ProcessRoute not found".to_string()));
        };

        // 先取消该产品的其他默认工艺路线
        self.unset_default_routes(model.tenant_id, model.product_id).await?;

        let mut active_model = crate::entity::process_route::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            product_id: Set(model.product_id),
            route_name: Set(model.route_name),
            version: Set(model.version),
            status: Set(model.status),
            is_default: Set(true),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }

    async fn unset_default_routes(&self, tenant_id: Uuid, product_id: Uuid) -> Result<(), AppError> {
        use crate::entity::process_route::Entity as ProcessRouteEntity;
        
        let routes = ProcessRouteEntity::find()
            .filter(ProcessRouteColumn::TenantId.eq(tenant_id))
            .filter(ProcessRouteColumn::ProductId.eq(product_id))
            .filter(ProcessRouteColumn::IsDefault.eq(true))
            .all(self.db.as_ref())
            .await?;

        for route in routes {
            let mut active_model = crate::entity::process_route::ActiveModel {
                id: Set(route.id),
                tenant_id: Set(route.tenant_id),
                product_id: Set(route.product_id),
                route_name: Set(route.route_name),
                version: Set(route.version),
                status: Set(route.status),
                is_default: Set(false),
                created_at: Set(route.created_at),
                updated_at: Set(Utc::now().naive_utc()),
            };
            active_model.update(self.db.as_ref()).await?;
        }

        Ok(())
    }
}
