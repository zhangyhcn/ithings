use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub is_leaf: bool,
    pub debit_credit: String,
    pub currency: String,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "liability")]
    Liability,
    #[serde(rename = "equity")]
    Equity,
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "expense")]
    Expense,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
}
