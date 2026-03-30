use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "delivery_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    pub delivery_no: String,
    pub sales_order_id: Option<Uuid>,
    #[sea_orm(column_type = "Uuid")]
    pub warehouse_id: Uuid,
    pub carrier: Option<String>,
    pub tracking_no: Option<String>,
    pub status: String,
    pub ship_date: Option<DateTime>,
    pub delivery_date: Option<DateTime>,
    pub remark: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sales_order::Entity",
        from = "Column::SalesOrderId",
        to = "super::sales_order::Column::Id",
        on_delete = "SetNull"
    )]
    SalesOrder,
    #[sea_orm(
        belongs_to = "super::warehouse::Entity",
        from = "Column::WarehouseId",
        to = "super::warehouse::Column::Id",
        on_delete = "Cascade"
    )]
    Warehouse,
}

impl ActiveModelBehavior for ActiveModel {}
