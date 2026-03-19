use axum::{
    extract::{Extension, State},
    response::Json,
    routing::post,
    Router,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use chrono::Utc;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    middleware::AuthUser,
    response::Response,
    utils::AppError,
    entity::TokenBlacklistEntity,
    entity::token_blacklist::ActiveModel,
    entity::token_blacklist::Column as TokenBlacklistColumn,
};

pub fn create_logout_router(db: DatabaseConnection, auth_state: crate::middleware::AuthState) -> Router {
    Router::new()
        .route("/auth/logout", post(logout))
        .with_state(db)
        .layer(axum::middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::auth_middleware,
        ))
}

async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Response<()>>, AppError> {
    // 从请求头中获取token
    let token = auth_user.token.ok_or_else(|| AppError::BadRequest("Token not found".to_string()))?;
    
    // 检查token是否已经在黑名单中
    let existing = TokenBlacklistEntity::find()
        .filter(TokenBlacklistColumn::Token.eq(&token))
        .one(&db)
        .await?;
    
    if existing.is_some() {
        return Ok(Json(Response::success(())));
    }
    
    // 将token加入黑名单
    let active_model = ActiveModel {
        id: Set(Uuid::new_v4()),
        token: Set(token),
        expires_at: Set(Utc::now().naive_utc() + chrono::Duration::hours(24)), // token默认有效期24小时
        created_at: Set(Utc::now().naive_utc()),
    };
    
    active_model.insert(&db).await?;
    
    Ok(Json(Response::success(())))
}
