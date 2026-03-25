use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_work_orders")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub order_no: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub erp_order_no: Option<String>,
    #[sea_orm(column_type = "Uuid")]
    pub product_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub product_name: String,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub quantity: Decimal,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", default_value = 0)]
    pub completed_qty: Decimal,
    #[sea_orm(column_type = "Text", default_value = "pending")]
    pub status: String,
    #[sea_orm(column_type = "Integer", default_value = 0)]
    pub priority: i32,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub plan_start_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub plan_end_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub actual_start_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub actual_end_time: Option<DateTime>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub workshop_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub production_line_id: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
