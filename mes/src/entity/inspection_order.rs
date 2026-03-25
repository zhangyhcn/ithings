use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "mes_inspection_orders")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub inspection_no: String,
    #[sea_orm(column_type = "Text")]
    pub inspection_type: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub work_order_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub material_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub batch_no: Option<String>,
    #[sea_orm(column_type = "Integer", nullable)]
    pub sample_qty: Option<i32>,
    #[sea_orm(column_type = "Integer", nullable)]
    pub pass_qty: Option<i32>,
    #[sea_orm(column_type = "Integer", nullable)]
    pub defect_qty: Option<i32>,
    #[sea_orm(column_type = "Text", default_value = "pending")]
    pub result: String,
    #[sea_orm(column_type = "Uuid", nullable)]
    pub inspector_id: Option<Uuid>,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub inspect_time: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
