use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_process_routes")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub product_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub route_name: String,
    #[sea_orm(column_type = "Text", default_value = "1.0")]
    pub version: String,
    #[sea_orm(column_type = "Text", default_value = "draft")]
    pub status: String,
    #[sea_orm(column_type = "Boolean", default_value = false)]
    pub is_default: bool,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
