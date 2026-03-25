use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_processes")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub route_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub process_no: String,
    #[sea_orm(column_type = "Text")]
    pub process_name: String,
    #[sea_orm(column_type = "Integer")]
    pub sequence: i32,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub work_station_id: Option<Uuid>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub standard_time: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub setup_time: Option<Decimal>,
    #[sea_orm(column_type = "Json", nullable)]
    pub process_params: Option<JsonValue>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub next_process_id: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
