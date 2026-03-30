use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "outbound_order_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub outbound_order_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    pub location_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Option<Decimal>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::outbound_order::Entity",
        from = "Column::OutboundOrderId",
        to = "super::outbound_order::Column::Id",
        on_delete = "Cascade"
    )]
    OutboundOrder,
    #[sea_orm(
        belongs_to = "super::material::Entity",
        from = "Column::MaterialId",
        to = "super::material::Column::Id",
        on_delete = "Cascade"
    )]
    Material,
}

impl Related<super::outbound_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutboundOrder.def()
    }
}

impl Related<super::material::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Material.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
