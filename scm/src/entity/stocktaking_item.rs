use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "stocktaking_items")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub stocktaking_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub location_id: Option<Uuid>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", default_value = "0")]
    pub system_qty: Decimal,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", default_value = "0")]
    pub actual_qty: Decimal,
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
