use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub role: String,
    pub is_superuser: bool,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(
        sub: String,
        user_id: Uuid,
        tenant_id: Option<Uuid>,
        role: String,
        is_superuser: bool,
        expires_in: i64,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            sub,
            user_id,
            tenant_id,
            role,
            is_superuser,
            exp: now + expires_in,
            iat: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
}

impl JwtConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            expires_in: 3600,
            refresh_expires_in: 604800,
        }
    }

    pub fn with_expires_in(mut self, expires_in: i64) -> Self {
        self.expires_in = expires_in;
        self
    }

    pub fn with_refresh_expires_in(mut self, refresh_expires_in: i64) -> Self {
        self.refresh_expires_in = refresh_expires_in;
        self
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self::new("your-secret-key-change-in-production".to_string())
    }
}
