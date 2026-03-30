use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "scm_suppliers")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub supplier_code: String,
    #[sea_orm(column_type = "Text")]
    pub supplier_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub contact_person: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub contact_phone: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub contact_email: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub bank_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub bank_account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub tax_number: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub supplier_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub credit_level: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub remarks: Option<String>,
    #[sea_orm(column_type = "Text", default_value = "active")]
    pub status: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
