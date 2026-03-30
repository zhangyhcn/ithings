use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "materials")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    pub category_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub specification: Option<String>,
    pub model: Option<String>,
    pub unit: String,
    pub unit_weight: Option<Decimal>,
    pub unit_volume: Option<Decimal>,
    pub barcode: Option<String>,
    pub status: String,
    pub safety_stock: Decimal,
    pub max_stock: Decimal,
    pub min_order_qty: Decimal,
    pub lead_time: i32,
    pub purchase_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::material_category::Entity",
        from = "Column::CategoryId",
        to = "super::material_category::Column::Id",
        on_delete = "SetNull"
    )]
    Category,
}

impl Related<super::material_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
