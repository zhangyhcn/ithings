use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "vouchers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub voucher_no: String,
    pub voucher_date: NaiveDate,
    pub voucher_type: String,
    pub description: Option<String>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTimeUtc>,
    pub posted_by: Option<Uuid>,
    pub posted_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoucherType {
    #[serde(rename = "receipt")]
    Receipt,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "transfer")]
    Transfer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "posted")]
    Posted,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::voucher_item::Entity")]
    Items,
}

impl Related<super::voucher_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn is_balanced(&self) -> bool {
        self.total_debit == self.total_credit
    }
}
