use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_maintenance_plans")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub equipment_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub plan_type: String,
    #[sea_orm(column_type = "Date", nullable)]
    pub plan_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    #[sea_orm(column_type = "Text", default_value = "pending")]
    pub status: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub executor_id: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub execute_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
