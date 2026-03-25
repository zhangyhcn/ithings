use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_equipments")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub equipment_no: String,
    #[sea_orm(column_type = "Text")]
    pub equipment_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub equipment_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub model: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub manufacturer: Option<String>,
    #[sea_orm(column_type = "Date", nullable)]
    pub purchase_date: Option<Date>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub workshop_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", default_value = "idle")]
    pub status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub ip_address: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
