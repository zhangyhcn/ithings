use std::sync::OnceLock;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::Value as JsonValue;

use crate::entity::{TenantModel, UserModel, RoleModel, MenuModel};

pub struct GlobalCache;

impl GlobalCache {
    fn get_cache() -> &'static RwLock<HashMap<String, JsonValue>> {
        static GLOBAL_CACHE: OnceLock<RwLock<HashMap<String, JsonValue>>> = OnceLock::new();
        GLOBAL_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
    }

    pub async fn get(key: &str) -> Option<JsonValue> {
        let cache = Self::get_cache().read().await;
        cache.get(key).cloned()
    }

    pub async fn set(key: String, value: JsonValue) {
        let mut cache = Self::get_cache().write().await;
        cache.insert(key, value);
    }

    pub async fn remove(key: &str) {
        let mut cache = Self::get_cache().write().await;
        cache.remove(key);
    }

    pub async fn clear() {
        let mut cache = Self::get_cache().write().await;
        cache.clear();
    }

    pub async fn get_tenant(tenant_id: Uuid) -> Option<TenantModel> {
        let key = format!("tenant:{}", tenant_id);
        let value = Self::get(&key).await?;
        serde_json::from_value(value).ok()
    }

    pub async fn set_tenant(tenant: TenantModel) {
        let key = format!("tenant:{}", tenant.id);
        let value = serde_json::to_value(&tenant).ok();
        if let Some(v) = value {
            Self::set(key, v).await;
        }
    }

    pub async fn remove_tenant(tenant_id: Uuid) {
        let key = format!("tenant:{}", tenant_id);
        Self::remove(&key).await;
    }

    pub async fn get_user(user_id: Uuid) -> Option<UserModel> {
        let key = format!("user:{}", user_id);
        let value = Self::get(&key).await?;
        serde_json::from_value(value).ok()
    }

    pub async fn set_user(user: UserModel) {
        let key = format!("user:{}", user.id);
        let value = serde_json::to_value(&user).ok();
        if let Some(v) = value {
            Self::set(key, v).await;
        }
    }

    pub async fn remove_user(user_id: Uuid) {
        let key = format!("user:{}", user_id);
        Self::remove(&key).await;
    }

    pub async fn get_role(role_id: Uuid) -> Option<RoleModel> {
        let key = format!("role:{}", role_id);
        let value = Self::get(&key).await?;
        serde_json::from_value(value).ok()
    }

    pub async fn set_role(role: RoleModel) {
        let key = format!("role:{}", role.id);
        let value = serde_json::to_value(&role).ok();
        if let Some(v) = value {
            Self::set(key, v).await;
        }
    }

    pub async fn remove_role(role_id: Uuid) {
        let key = format!("role:{}", role_id);
        Self::remove(&key).await;
    }

    pub async fn get_menu(menu_id: Uuid) -> Option<MenuModel> {
        let key = format!("menu:{}", menu_id);
        let value = Self::get(&key).await?;
        serde_json::from_value(value).ok()
    }

    pub async fn set_menu(menu: MenuModel) {
        let key = format!("menu:{}", menu.id);
        let value = serde_json::to_value(&menu).ok();
        if let Some(v) = value {
            Self::set(key, v).await;
        }
    }

    pub async fn remove_menu(menu_id: Uuid) {
        let key = format!("menu:{}", menu_id);
        Self::remove(&key).await;
    }

    pub async fn get_custom(key: &str) -> Option<JsonValue> {
        Self::get(key).await
    }

    pub async fn set_custom(key: String, value: JsonValue) {
        Self::set(key, value).await;
    }
}