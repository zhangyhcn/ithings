use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::middleware::jwt::{Claims, JwtConfig};
use crate::utils::AppError;

#[derive(Clone)]
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(
        &self,
        user_id: Uuid,
        tenant_id: Option<Uuid>,
        username: String,
        role: String,
        is_superuser: bool,
    ) -> Result<String, AppError> {
        let claims = Claims::new(
            username,
            user_id,
            tenant_id,
            role,
            is_superuser,
            self.config.expires_in,
        );

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        tenant_id: Option<Uuid>,
        username: String,
        role: String,
        is_superuser: bool,
    ) -> Result<String, AppError> {
        let claims = Claims::new(
            username,
            user_id,
            tenant_id,
            role,
            is_superuser,
            self.config.refresh_expires_in,
        );

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }

    pub fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), AppError> {
        let claims = self.verify_token(refresh_token)?;

        let access_token = self.generate_token(
            claims.user_id,
            claims.tenant_id,
            claims.sub.clone(),
            claims.role.clone(),
            claims.is_superuser,
        )?;

        let new_refresh_token = self.generate_refresh_token(
            claims.user_id,
            claims.tenant_id,
            claims.sub,
            claims.role,
            claims.is_superuser,
        )?;

        Ok((access_token, new_refresh_token))
    }
}
