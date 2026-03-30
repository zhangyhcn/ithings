use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "production_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    pub order_no: String,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    pub bom_id: Option<Uuid>,
    pub quantity: Decimal,
    pub unit: String,
    pub planned_start_date: Option<DateTime>,
    pub planned_end_date: Option<DateTime>,
    pub actual_start_date: Option<DateTime>,
    pub actual_end_date: Option<DateTime>,
    pub status: String,
    pub remark: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::material::Entity",
        from = "Column::MaterialId",
        to = "super::material::Column::Id",
        on_delete = "Cascade"
    )]
    Material,
    #[sea_orm(
        belongs_to = "super::bom::Entity",
        from = "Column::BomId",
        to = "super::bom::Column::Id",
        on_delete = "SetNull"
    )]
    Bom,
}

impl ActiveModelBehavior for ActiveModel {}
