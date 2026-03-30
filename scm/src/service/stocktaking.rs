use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{StocktakingOrderEntity, StocktakingOrderColumn, StocktakingOrderModel as Model};
use crate::entity::stocktaking_order::ActiveModel;
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStocktakingRequest {
    pub stocktaking_no: String,
    pub warehouse_id: String,
    pub stocktaking_date: String,
    pub stocktaking_type: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStocktakingRequest {
    pub stocktaking_date: Option<String>,
    pub stocktaking_type: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StocktakingResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub stocktaking_no: String,
    pub warehouse_id: String,
    pub stocktaking_date: String,
    pub stocktaking_type: String,
    pub status: String,
    pub remarks: Option<String>,
    pub created_by: Option<String>,
    pub confirmed_by: Option<String>,
    pub confirmed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct StocktakingService {
    db: DatabaseConnection,
}

impl StocktakingService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<StocktakingResponse>, AppError> {
        let items = StocktakingOrderEntity::find()
            .filter(StocktakingOrderColumn::TenantId.eq(tenant_id))
            .filter(StocktakingOrderColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(items.into_iter().map(Self::model_to_response).collect())
    }

    pub async fn create(&self, tenant_id: Uuid, org_id: Uuid, req: CreateStocktakingRequest) -> Result<StocktakingResponse, AppError> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();
        
        let active_model = ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            stocktaking_no: Set(req.stocktaking_no),
            warehouse_id: Set(Uuid::parse_str(&req.warehouse_id).map_err(|_| AppError::BadRequest("Invalid warehouse_id".to_string()))?),
            stocktaking_date: Set(req.stocktaking_date.parse().map_err(|_| AppError::BadRequest("Invalid stocktaking_date".to_string()))?),
            stocktaking_type: Set(req.stocktaking_type.unwrap_or_else(|| "full".to_string())),
            status: Set("draft".to_string()),
            remarks: Set(req.remarks),
            created_by: Set(None),
            confirmed_by: Set(None),
            confirmed_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn get(&self, id: Uuid) -> Result<StocktakingResponse, AppError> {
        let model = StocktakingOrderEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Stocktaking not found".to_string()))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn update(&self, id: Uuid, req: UpdateStocktakingRequest) -> Result<StocktakingResponse, AppError> {
        let model = StocktakingOrderEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Stocktaking not found".to_string()))?;

        let mut active_model: ActiveModel = model.into();
        
        if let Some(stocktaking_date) = req.stocktaking_date {
            active_model.stocktaking_date = Set(stocktaking_date.parse().map_err(|_| AppError::BadRequest("Invalid stocktaking_date".to_string()))?);
        }
        if let Some(stocktaking_type) = req.stocktaking_type {
            active_model.stocktaking_type = Set(stocktaking_type);
        }
        if let Some(remarks) = req.remarks {
            active_model.remarks = Set(Some(remarks));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        active_model.updated_at = Set(Utc::now().naive_utc());

        let model = active_model
            .update(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        StocktakingOrderEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(())
    }

    fn model_to_response(model: Model) -> StocktakingResponse {
        StocktakingResponse {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            stocktaking_no: model.stocktaking_no,
            warehouse_id: model.warehouse_id.to_string(),
            stocktaking_date: model.stocktaking_date.to_string(),
            stocktaking_type: model.stocktaking_type,
            status: model.status,
            remarks: model.remarks,
            created_by: model.created_by.map(|v| v.to_string()),
            confirmed_by: model.confirmed_by.map(|v| v.to_string()),
            confirmed_at: model.confirmed_at.map(|v| v.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
