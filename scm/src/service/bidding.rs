use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{BiddingEntity, BiddingColumn, BiddingModel as Model};
use crate::utils::AppError;
use crate::entity::bidding::ActiveModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBiddingRequest {
    pub bidding_no: String,
    pub title: String,
    pub bidding_type: Option<String>,
    pub publish_date: String,
    pub deadline: String,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBiddingRequest {
    pub title: Option<String>,
    pub bidding_type: Option<String>,
    pub publish_date: Option<String>,
    pub deadline: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiddingResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub bidding_no: String,
    pub title: String,
    pub bidding_type: String,
    pub publish_date: String,
    pub deadline: String,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<String>,
    pub status: String,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct BiddingService {
    db: DatabaseConnection,
}

impl BiddingService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<BiddingResponse>, AppError> {
        let items = BiddingEntity::find()
            .filter(BiddingColumn::TenantId.eq(tenant_id))
            .filter(BiddingColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(items.into_iter().map(Self::model_to_response).collect())
    }

    pub async fn create(&self, tenant_id: Uuid, org_id: Uuid, req: CreateBiddingRequest) -> Result<BiddingResponse, AppError> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();
        
        let active_model = ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            bidding_no: Set(req.bidding_no),
            title: Set(req.title),
            bidding_type: Set(req.bidding_type.unwrap_or_else(|| "public".to_string())),
            publish_date: Set(req.publish_date.parse().map_err(|_| AppError::BadRequest("Invalid publish_date".to_string()))?),
            deadline: Set(req.deadline.parse().map_err(|_| AppError::BadRequest("Invalid deadline".to_string()))?),
            contact_person: Set(req.contact_person),
            contact_phone: Set(req.contact_phone),
            description: Set(req.description),
            requirements: Set(req.requirements),
            status: Set("draft".to_string()),
            created_by: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn get(&self, id: Uuid) -> Result<BiddingResponse, AppError> {
        let model = BiddingEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Bidding not found".to_string()))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn update(&self, id: Uuid, req: UpdateBiddingRequest) -> Result<BiddingResponse, AppError> {
        let model = BiddingEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Bidding not found".to_string()))?;

        let mut active_model: ActiveModel = model.into();
        
        if let Some(title) = req.title {
            active_model.title = Set(title);
        }
        if let Some(bidding_type) = req.bidding_type {
            active_model.bidding_type = Set(bidding_type);
        }
        if let Some(publish_date) = req.publish_date {
            active_model.publish_date = Set(publish_date.parse().map_err(|_| AppError::BadRequest("Invalid publish_date".to_string()))?);
        }
        if let Some(deadline) = req.deadline {
            active_model.deadline = Set(deadline.parse().map_err(|_| AppError::BadRequest("Invalid deadline".to_string()))?);
        }
        if let Some(contact_person) = req.contact_person {
            active_model.contact_person = Set(Some(contact_person));
        }
        if let Some(contact_phone) = req.contact_phone {
            active_model.contact_phone = Set(Some(contact_phone));
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(requirements) = req.requirements {
            active_model.requirements = Set(Some(requirements));
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
        BiddingEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(())
    }

    fn model_to_response(model: Model) -> BiddingResponse {
        BiddingResponse {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            bidding_no: model.bidding_no,
            title: model.title,
            bidding_type: model.bidding_type,
            publish_date: model.publish_date.to_string(),
            deadline: model.deadline.to_string(),
            contact_person: model.contact_person,
            contact_phone: model.contact_phone,
            description: model.description,
            requirements: model.requirements,
            status: model.status,
            created_by: model.created_by.map(|v| v.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
