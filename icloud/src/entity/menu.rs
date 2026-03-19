use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "menus")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub status: String,
    #[sea_orm(column_type = "Json")]
    pub roles: serde_json::Value,
    pub i18n_key: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_delete = "Cascade"
    )]
    Parent,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_menu::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_menu::Relation::Menu.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
