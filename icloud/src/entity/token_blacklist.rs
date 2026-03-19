use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "token_blacklist")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub token: String,
    #[sea_orm(column_type = "Timestamp")]
    pub expires_at: NaiveDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
