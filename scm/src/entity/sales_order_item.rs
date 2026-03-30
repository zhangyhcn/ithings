use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sales_order_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub sales_order_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub unit: String,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub delivered_qty: Decimal,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sales_order::Entity",
        from = "Column::SalesOrderId",
        to = "super::sales_order::Column::Id",
        on_delete = "Cascade"
    )]
    SalesOrder,
    #[sea_orm(
        belongs_to = "super::material::Entity",
        from = "Column::MaterialId",
        to = "super::material::Column::Id",
        on_delete = "Cascade"
    )]
    Material,
}

impl ActiveModelBehavior for ActiveModel {}
