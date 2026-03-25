use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_materials")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub material_no: String,
    #[sea_orm(column_type = "Text")]
    pub material_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub specification: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub unit: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub material_type: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", nullable)]
    pub safety_stock: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", nullable)]
    pub max_stock: Option<Decimal>,
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
