use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub warehouse_id: Uuid,
    pub location_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    pub batch_no: Option<String>,
    pub quantity: Decimal,
    pub frozen_qty: Decimal,
    pub available_qty: Decimal,
    pub cost_price: Option<Decimal>,
    pub production_date: Option<DateTime>,
    pub expiry_date: Option<DateTime>,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::warehouse::Entity",
        from = "Column::WarehouseId",
        to = "super::warehouse::Column::Id",
        on_delete = "Cascade"
    )]
    Warehouse,
    #[sea_orm(
        belongs_to = "super::warehouse_location::Entity",
        from = "Column::LocationId",
        to = "super::warehouse_location::Column::Id",
        on_delete = "SetNull"
    )]
    Location,
    #[sea_orm(
        belongs_to = "super::material::Entity",
        from = "Column::MaterialId",
        to = "super::material::Column::Id",
        on_delete = "Cascade"
    )]
    Material,
}

impl Related<super::warehouse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warehouse.def()
    }
}

impl Related<super::warehouse_location::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Location.def()
    }
}

impl Related<super::material::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Material.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
