use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "contracts")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub contract_no: String,
    #[sea_orm(column_type = "Uuid")]
    pub supplier_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Date")]
    pub start_date: Date,
    #[sea_orm(column_type = "Date")]
    pub end_date: Date,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub total_amount: Decimal,
    #[sea_orm(column_type = "Text", default_value = "CNY")]
    pub currency: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub payment_terms: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub delivery_terms: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub quality_terms: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub remarks: Option<String>,
    #[sea_orm(column_type = "Text", default_value = "draft")]
    pub status: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub signed_by: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub signed_at: Option<DateTime>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub created_by: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
