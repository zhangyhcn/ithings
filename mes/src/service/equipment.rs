use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{EquipmentEntity, EquipmentColumn, EquipmentModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEquipmentRequest {
    pub equipment_no: String,
    pub equipment_name: String,
    pub equipment_type: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub purchase_date: Option<String>,
    pub workshop_id: Option<Uuid>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentResponse {
    pub id: String,
    pub tenant_id: String,
    pub equipment_no: String,
    pub equipment_name: String,
    pub equipment_type: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub purchase_date: Option<String>,
    pub workshop_id: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for EquipmentResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            equipment_no: model.equipment_no,
            equipment_name: model.equipment_name,
            equipment_type: model.equipment_type,
            model: model.model,
            manufacturer: model.manufacturer,
            purchase_date: model.purchase_date.map(|d| d.to_string()),
            workshop_id: model.workshop_id.map(|id| id.to_string()),
            status: model.status,
            ip_address: model.ip_address,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct EquipmentService {
    db: DatabaseConnection,
}

impl EquipmentService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateEquipmentRequest,
    ) -> Result<EquipmentResponse, AppError> {
        let now = Utc::now().naive_utc();
        let active_model = crate::entity::equipment::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            equipment_no: Set(req.equipment_no),
            equipment_name: Set(req.equipment_name),
            equipment_type: Set(req.equipment_type),
            model: Set(req.model),
            manufacturer: Set(req.manufacturer),
            purchase_date: Set(req.purchase_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())),
            workshop_id: Set(req.workshop_id),
            status: Set("idle".to_string()),
            ip_address: Set(req.ip_address),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<EquipmentResponse>, AppError> {
        let models = EquipmentEntity::find()
            .filter(EquipmentColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }
}
