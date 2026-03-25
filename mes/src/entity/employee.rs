use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_employees")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub employee_no: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub department_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub position: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "Text", default_value = "active")]
    pub status: String,
    #[sea_orm(column_type = "Date", nullable)]
    pub entry_date: Option<Date>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
