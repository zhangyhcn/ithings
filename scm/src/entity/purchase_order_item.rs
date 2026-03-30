use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scm_purchase_order_items")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub order_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub material_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub specification: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub unit: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub quantity: Decimal,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub unit_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub total_price: Decimal,
    #[sea_orm(column_type = "Date", nullable)]
    pub expected_delivery_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub remarks: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
