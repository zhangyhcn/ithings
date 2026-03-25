use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_schedule_plans")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub plan_no: String,
    #[sea_orm(column_type = "Uuid")]
    pub work_order_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub process_id: Uuid,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub equipment_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub operator_id: Option<Uuid>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
    pub plan_quantity: Decimal,
    #[sea_orm(column_type = "Text", default_value = "pending")]
    pub status: String,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub start_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub end_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
