use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sales_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub org_id: Uuid,
    pub order_no: String,
    pub customer_name: String,
    pub customer_contact: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub order_date: DateTime,
    pub delivery_date: Option<DateTime>,
    pub total_amount: Decimal,
    pub status: String,
    pub remark: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
