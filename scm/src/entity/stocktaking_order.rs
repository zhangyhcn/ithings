use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "stocktaking_orders")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub stocktaking_no: String,
    #[sea_orm(column_type = "Uuid")]
    pub warehouse_id: Uuid,
    #[sea_orm(column_type = "Date")]
    pub stocktaking_date: Date,
    #[sea_orm(column_type = "Text", default_value = "full")]
    pub stocktaking_type: String,
    #[sea_orm(column_type = "Text", default_value = "draft")]
    pub status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub remarks: Option<String>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub created_by: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub confirmed_by: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub confirmed_at: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
