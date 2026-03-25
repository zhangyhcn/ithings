use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{MaterialEntity, MaterialColumn, MaterialModel as Model};
use crate::utils::AppError;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialRequest {
    pub material_no: String,
    pub material_name: String,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub material_type: Option<String>,
    pub safety_stock: Option<f64>,
    pub max_stock: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialResponse {
    pub id: String,
    pub tenant_id: String,
    pub material_no: String,
    pub material_name: String,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub material_type: Option<String>,
    pub safety_stock: Option<String>,
    pub max_stock: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for MaterialResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            material_no: model.material_no,
            material_name: model.material_name,
            specification: model.specification,
            unit: model.unit,
            material_type: model.material_type,
            safety_stock: model.safety_stock.map(|s| s.to_string()),
            max_stock: model.max_stock.map(|s| s.to_string()),
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct MaterialService {
    db: DatabaseConnection,
}

impl MaterialService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateMaterialRequest,
    ) -> Result<MaterialResponse, AppError> {
        let now = Utc::now().naive_utc();
        let active_model = crate::entity::material::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            material_no: Set(req.material_no),
            material_name: Set(req.material_name),
            specification: Set(req.specification),
            unit: Set(req.unit),
            material_type: Set(req.material_type),
            safety_stock: Set(Decimal::from_f64(req.safety_stock.unwrap_or(0.0))),
            max_stock: Set(Decimal::from_f64(req.max_stock.unwrap_or(0.0))),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<MaterialResponse>, AppError> {
        let models = MaterialEntity::find()
            .filter(MaterialColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }
}
