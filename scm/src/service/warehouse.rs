use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{WarehouseEntity, WarehouseColumn, WarehouseModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWarehouseRequest {
    pub code: String,
    pub name: String,
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub manager: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWarehouseRequest {
    pub name: Option<String>,
    pub warehouse_type: Option<String>,
    pub address: Option<String>,
    pub manager: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub warehouse_type: String,
    pub address: Option<String>,
    pub manager: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for WarehouseResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            code: model.code,
            name: model.name,
            warehouse_type: model.warehouse_type,
            address: model.address,
            manager: model.manager,
            phone: model.phone,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct WarehouseService {
    db: DatabaseConnection,
}

impl WarehouseService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        req: CreateWarehouseRequest,
    ) -> Result<WarehouseResponse, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        
        let warehouse = crate::entity::warehouse::ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            code: Set(req.code),
            name: Set(req.name),
            warehouse_type: Set(req.warehouse_type.unwrap_or_else(|| "normal".to_string())),
            address: Set(req.address),
            manager: Set(req.manager),
            phone: Set(req.phone),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let warehouse = warehouse.insert(&self.db).await?;
        Ok(warehouse.into())
    }

    pub async fn list_all(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<WarehouseResponse>, AppError> {
        let warehouses = WarehouseEntity::find()
            .filter(WarehouseColumn::TenantId.eq(tenant_id))
            .filter(WarehouseColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(warehouses.into_iter().map(|w| w.into()).collect())
    }

    pub async fn get_by_id(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<WarehouseResponse>, AppError> {
        let warehouse = WarehouseEntity::find()
            .filter(WarehouseColumn::TenantId.eq(tenant_id))
            .filter(WarehouseColumn::OrgId.eq(org_id))
            .filter(WarehouseColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        Ok(warehouse.map(|w| w.into()))
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
        req: UpdateWarehouseRequest,
    ) -> Result<WarehouseResponse, AppError> {
        let warehouse = WarehouseEntity::find()
            .filter(WarehouseColumn::TenantId.eq(tenant_id))
            .filter(WarehouseColumn::OrgId.eq(org_id))
            .filter(WarehouseColumn::Id.eq(id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("Warehouse not found".to_string()))?;

        let now = Utc::now().naive_utc();
        let mut warehouse: crate::entity::warehouse::ActiveModel = warehouse.into();
        
        if let Some(name) = req.name {
            warehouse.name = Set(name);
        }
        if let Some(warehouse_type) = req.warehouse_type {
            warehouse.warehouse_type = Set(warehouse_type);
        }
        if let Some(address) = req.address {
            warehouse.address = Set(Some(address));
        }
        if let Some(manager) = req.manager {
            warehouse.manager = Set(Some(manager));
        }
        if let Some(phone) = req.phone {
            warehouse.phone = Set(Some(phone));
        }
        if let Some(status) = req.status {
            warehouse.status = Set(status);
        }
        
        warehouse.updated_at = Set(now);
        let warehouse = warehouse.update(&self.db).await?;
        Ok(warehouse.into())
    }

    pub async fn delete(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        WarehouseEntity::delete_by_id(id)
            .filter(WarehouseColumn::TenantId.eq(tenant_id))
            .filter(WarehouseColumn::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
