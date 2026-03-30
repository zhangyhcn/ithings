use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "supplier_quotations")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub supplier_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub price: Decimal,
    #[sea_orm(column_type = "Text", default_value = "CNY")]
    pub currency: String,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", default_value = "1")]
    pub min_qty: Decimal,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", nullable)]
    pub max_qty: Option<Decimal>,
    #[sea_orm(column_type = "Date")]
    pub valid_from: Date,
    #[sea_orm(column_type = "Date")]
    pub valid_to: Date,
    #[sea_orm(column_type = "Integer", default_value = "0")]
    pub lead_time: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub payment_terms: Option<String>,
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
