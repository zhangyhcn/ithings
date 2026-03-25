use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "nodes")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub status: String,
    #[sea_orm(column_type = "Json")]
    pub labels: Json,
    #[sea_orm(column_type = "Json")]
    pub roles: Json,
    #[sea_orm(column_type = "Text")]
    pub internal_ip: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub os: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub kernel_version: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub container_runtime: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
