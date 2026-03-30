use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{EmployeeEntity, EmployeeColumn, EmployeeModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    pub employee_no: String,
    pub name: String,
    pub department_id: Option<Uuid>,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub entry_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeResponse {
    pub id: String,
    pub tenant_id: String,
    pub employee_no: String,
    pub name: String,
    pub department_id: Option<String>,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub entry_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for EmployeeResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            employee_no: model.employee_no,
            name: model.name,
            department_id: model.department_id.map(|id| id.to_string()),
            position: model.position,
            phone: model.phone,
            status: model.status,
            entry_date: model.entry_date.map(|d| d.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct EmployeeService {
    db: DatabaseConnection,
}

impl EmployeeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateEmployeeRequest,
    ) -> Result<EmployeeResponse, AppError> {
        let now = Utc::now().naive_utc();
        let active_model = crate::entity::employee::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            employee_no: Set(req.employee_no),
            name: Set(req.name),
            department_id: Set(req.department_id),
            position: Set(req.position),
            phone: Set(req.phone),
            status: Set("active".to_string()),
            entry_date: Set(req.entry_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<EmployeeResponse>, AppError> {
        let models = EmployeeEntity::find()
            .filter(EmployeeColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<EmployeeResponse>, AppError> {
        let model = EmployeeEntity::find_by_id(id)
            .filter(EmployeeColumn::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?;

        Ok(model.map(Into::into))
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        name: Option<String>,
        department_id: Option<Uuid>,
        position: Option<String>,
        phone: Option<String>,
        entry_date: Option<String>,
        status: Option<String>,
    ) -> Result<EmployeeResponse, AppError> {
        let employee = EmployeeEntity::find_by_id(id)
            .filter(EmployeeColumn::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("Employee not found".to_string()))?;

        let mut active_model: crate::entity::employee::ActiveModel = employee.into();
        if let Some(n) = name {
            active_model.name = Set(n);
        }
        if let Some(did) = department_id {
            active_model.department_id = Set(Some(did));
        }
        if let Some(pos) = position {
            active_model.position = Set(Some(pos));
        }
        if let Some(ph) = phone {
            active_model.phone = Set(Some(ph));
        }
        if let Some(date) = entry_date {
            active_model.entry_date = Set(chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok());
        }
        if let Some(s) = status {
            active_model.status = Set(s);
        }
        active_model.updated_at = Set(Utc::now().naive_utc());

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        EmployeeEntity::delete_by_id(id)
            .filter(EmployeeColumn::TenantId.eq(tenant_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
