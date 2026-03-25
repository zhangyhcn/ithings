use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct TenantCache {
    data: Arc<Mutex<HashMap<Uuid, TenantInfo>>>,
}

#[derive(Clone, Debug)]
pub struct TenantInfo {
    pub registry_url: Option<String>,
    pub virtual_cluster_name: Option<String>,
    pub config: Option<serde_json::Value>,
}

impl TenantCache {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, tenant_id: &Uuid) -> Option<TenantInfo> {
        let cache = self.data.lock().unwrap();
        cache.get(tenant_id).cloned()
    }

    pub fn set(&self, tenant_id: Uuid, info: TenantInfo) {
        let mut cache = self.data.lock().unwrap();
        cache.insert(tenant_id, info);
    }

    pub fn get_registry_url(&self, tenant_id: &Uuid) -> Option<String> {
        self.get(tenant_id).and_then(|info| info.registry_url.clone())
    }

    pub fn set_registry_url(&self, tenant_id: Uuid, registry_url: String) {
        let mut cache = self.data.lock().unwrap();
        if let Some(mut info) = cache.get_mut(&tenant_id) {
            info.registry_url = Some(registry_url);
        } else {
            cache.insert(tenant_id, TenantInfo {
                registry_url: Some(registry_url),
                virtual_cluster_name: None,
                config: None,
            });
        }
    }

    pub fn get_virtual_cluster_name(&self, tenant_id: &Uuid) -> Option<String> {
        self.get(tenant_id).and_then(|info| info.virtual_cluster_name.clone())
    }

    pub fn set_virtual_cluster_name(&self, tenant_id: Uuid, cluster_name: String) {
        let mut cache = self.data.lock().unwrap();
        if let Some(mut info) = cache.get_mut(&tenant_id) {
            info.virtual_cluster_name = Some(cluster_name);
        } else {
            cache.insert(tenant_id, TenantInfo {
                registry_url: None,
                virtual_cluster_name: Some(cluster_name),
                config: None,
            });
        }
    }

    pub fn get_config(&self, tenant_id: &Uuid) -> Option<serde_json::Value> {
        self.get(tenant_id).and_then(|info| info.config.clone())
    }

    pub fn set_config(&self, tenant_id: Uuid, config: serde_json::Value) {
        let mut cache = self.data.lock().unwrap();
        if let Some(mut info) = cache.get_mut(&tenant_id) {
            info.config = Some(config);
        } else {
            cache.insert(tenant_id, TenantInfo {
                registry_url: None,
                virtual_cluster_name: None,
                config: Some(config),
            });
        }
    }

    pub fn remove(&self, tenant_id: &Uuid) {
        let mut cache = self.data.lock().unwrap();
        cache.remove(tenant_id);
    }

    pub fn clear(&self) {
        let mut cache = self.data.lock().unwrap();
        cache.clear();
    }
}

lazy_static! {
    static ref TENANT_CACHE: TenantCache = TenantCache::new();
}

pub fn get_tenant_cache() -> TenantCache {
    TENANT_CACHE.clone()
}