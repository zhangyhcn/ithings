use sea_orm::entity::prelude::*;
use sea_orm::prelude::Json;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Json")]
    pub permissions: Json,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRole,
    #[sea_orm(has_many = "super::role_menu::Entity")]
    RoleMenu,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::Role.def().rev())
    }
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_menu::Relation::Menu.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_menu::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
