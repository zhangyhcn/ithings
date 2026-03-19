use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub tenant_id: Option<Uuid>,
    #[sea_orm(unique, column_type = "Text")]
    pub username: String,
    #[sea_orm(unique, column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String,
    #[sea_orm(column_type = "Text")]
    pub phone: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub role: String,
    #[sea_orm(column_type = "Boolean")]
    pub is_superuser: bool,
    #[sea_orm(column_type = "Boolean")]
    pub is_active: bool,
    #[sea_orm(column_type = "Timestamp")]
    pub last_login: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tenant::Entity",
        from = "Column::TenantId",
        to = "super::tenant::Column::Id"
    )]
    Tenant,
}

impl ActiveModelBehavior for ActiveModel {}
