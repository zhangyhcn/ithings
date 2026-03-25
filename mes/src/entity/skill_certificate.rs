use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_skill_certificates")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub employee_id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub skill_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub certificate_no: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub level: Option<String>,
    #[sea_orm(column_type = "Date", nullable)]
    pub issue_date: Option<Date>,
    #[sea_orm(column_type = "Date", nullable)]
    pub expire_date: Option<Date>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
