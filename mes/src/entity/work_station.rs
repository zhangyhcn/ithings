use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_work_stations")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub station_no: String,
    #[sea_orm(column_type = "Text")]
    pub station_name: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub workshop_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub production_line_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub equipment_id: Option<Uuid>,
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
