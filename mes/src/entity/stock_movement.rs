use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_stock_movements")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub movement_no: String,
    #[sea_orm(column_type = "Text")]
    pub movement_type: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub work_order_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub quantity: Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub batch_no: Option<String>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub operator_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", default_value = "pending")]
    pub status: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
