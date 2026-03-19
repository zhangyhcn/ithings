use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "device_instances")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub site_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub brand_model: Option<String>,
    #[sea_orm(column_type = "Uuid")]
    pub product_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub driver_id: Uuid,
    #[sea_orm(column_type = "Integer")]
    pub poll_interval_ms: u64,
    #[sea_orm(column_type = "Text")]
    pub device_type: String,
    #[sea_orm(column_type = "Json")]
    pub driver_config: Json,
    #[sea_orm(column_type = "Json")]
    pub thing_model: Json,
    #[sea_orm(column_type = "Uuid")]
    pub node_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub status: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
