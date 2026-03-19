mod api;
mod config;
mod entity;
mod middleware;
mod migration;
mod response;
mod service;
mod utils;

use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::{Database, DatabaseBackend, Statement, ConnectionTrait};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use api::tenant::create_tenant_router;
use api::user::create_user_router;
use api::site::create_site_router;
use api::product::create_product_router;
use api::driver::create_driver_router;
use api::node::create_node_router;
use api::device_instance::create_device_instance_router;
use api::namespace::create_namespace_router;
use api::organization::create_organization_router;
use api::department::create_department_router;
use api::menu::create_menu_router;
use api::role::create_role_router;
use api::role_menu::create_role_menu_router;
use api::user_role::create_user_role_router;
use api::logout::create_logout_router;
use api::{create_crd_router, create_operator_router, create_controller_router, create_config_map_router, create_secret_router};
use api::namespace::list_namespaces_by_tenant_slug;
use config::Config;
use middleware::{AuthState, JwtConfig, JwtService};

#[tokio::main]
async fn main() {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("icloud=info".parse().unwrap()))
        .init();

    tracing::info!("Starting iCloud server...");

    let db = Database::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected successfully");

    let migrations = vec![
        r#"CREATE TABLE IF NOT EXISTS token_blacklist (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            token TEXT NOT NULL UNIQUE,
            expires_at TIMESTAMP NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS tenants (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            description TEXT,
            config JSON,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID,
            username TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            phone TEXT,
            role TEXT NOT NULL DEFAULT 'user',
            is_superuser BOOLEAN NOT NULL DEFAULT false,
            is_active BOOLEAN NOT NULL DEFAULT true,
            last_login TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE SET NULL
        )"#,
        r#"ALTER TABLE users ALTER COLUMN tenant_id DROP NOT NULL;"#,
        r#"CREATE TABLE IF NOT EXISTS roles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            permissions JSONB NOT NULL DEFAULT '[]'::jsonb,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            UNIQUE(tenant_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS user_roles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            user_id UUID NOT NULL,
            role_id UUID NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
            UNIQUE(user_id, role_id)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS sites (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            description TEXT,
            location TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            config JSON,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
        )"#,
        r#"CREATE TABLE IF NOT EXISTS namespaces (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            site_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            description TEXT,
            namespace_type TEXT NOT NULL DEFAULT 'default',
            config JSON,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
        )"#,
        r#"CREATE INDEX IF NOT EXISTS idx_user_tenant_username ON users(tenant_id, username)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_site_tenant_slug ON sites(tenant_id, slug)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_namespace_tenant_slug ON namespaces(tenant_id, slug)"#,
        r#"CREATE TABLE IF NOT EXISTS organizations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            parent_id UUID,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            FOREIGN KEY (parent_id) REFERENCES organizations(id) ON DELETE SET NULL
        )"#,
        r#"CREATE TABLE IF NOT EXISTS departments (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            organization_id UUID NOT NULL,
            parent_id UUID,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE,
            FOREIGN KEY (parent_id) REFERENCES departments(id) ON DELETE SET NULL
        )"#,
        r#"CREATE INDEX IF NOT EXISTS idx_org_tenant_slug ON organizations(tenant_id, slug)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_dept_org_slug ON departments(organization_id, slug)"#,
        r#"CREATE TABLE IF NOT EXISTS menus (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            parent_id UUID,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            component TEXT NOT NULL,
            icon TEXT,
            sort_order INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'active',
            roles JSONB NOT NULL DEFAULT '[]'::jsonb,
            i18n_key TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (parent_id) REFERENCES menus(id) ON DELETE CASCADE
        )"#,
        r#"CREATE TABLE IF NOT EXISTS role_menus (
            id SERIAL PRIMARY KEY,
            role_id UUID NOT NULL,
            menu_id UUID NOT NULL,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
            FOREIGN KEY (menu_id) REFERENCES menus(id) ON DELETE CASCADE,
            UNIQUE(role_id, menu_id)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS crdes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            namespace_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            "group" TEXT NOT NULL,
            version TEXT NOT NULL,
            kind TEXT NOT NULL,
            description TEXT,
            yaml JSON,
            status TEXT NOT NULL DEFAULT 'draft',
            k8s_name TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (namespace_id) REFERENCES namespaces(id) ON DELETE CASCADE,
            UNIQUE(namespace_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS operators (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            namespace_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT,
            yaml JSON,
            status TEXT NOT NULL DEFAULT 'draft',
            k8s_name TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (namespace_id) REFERENCES namespaces(id) ON DELETE CASCADE,
            UNIQUE(namespace_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS controllers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            namespace_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            kind TEXT NOT NULL,
            version TEXT NOT NULL,
            description TEXT,
            yaml JSON,
            status TEXT NOT NULL DEFAULT 'draft',
            k8s_name TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (namespace_id) REFERENCES namespaces(id) ON DELETE CASCADE,
            UNIQUE(namespace_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS config_maps (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            namespace_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            data JSON,
            status TEXT NOT NULL DEFAULT 'draft',
            k8s_name TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (namespace_id) REFERENCES namespaces(id) ON DELETE CASCADE,
            UNIQUE(namespace_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS secrets (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            namespace_id UUID NOT NULL,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            data JSON,
            status TEXT NOT NULL DEFAULT 'draft',
            k8s_name TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (namespace_id) REFERENCES namespaces(id) ON DELETE CASCADE,
            UNIQUE(namespace_id, slug)
        )"#,
        r#"CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            thing_model JSON NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
        )"#,
        r#"CREATE TABLE IF NOT EXISTS drivers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            protocol_type TEXT NOT NULL,
            image TEXT NOT NULL,
            version TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
        )"#,
        r#"CREATE TABLE IF NOT EXISTS nodes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            name TEXT NOT NULL,
            address TEXT,
            k8s_context TEXT,
            is_shared BOOLEAN NOT NULL DEFAULT false,
            status TEXT NOT NULL DEFAULT 'offline',
            last_sync TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
        )"#,
        r#"CREATE TABLE IF NOT EXISTS device_instances (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            org_id UUID NOT NULL,
            site_id UUID NOT NULL,
            name TEXT NOT NULL,
            brand_model TEXT,
            product_id UUID NOT NULL,
            driver_id UUID NOT NULL,
            poll_interval_ms INTEGER NOT NULL,
            device_type TEXT NOT NULL,
            driver_config JSON NOT NULL,
            thing_model JSON NOT NULL,
            node_id UUID NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
            FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
            FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (driver_id) REFERENCES drivers(id) ON DELETE CASCADE,
            FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
        )"#,
    ];

    for sql in migrations {
        db.execute(Statement::from_string(DatabaseBackend::Postgres, sql))
            .await
            .expect("Migration failed");
    }

    db.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        r#"INSERT INTO tenants (name, slug, description, config, status) 
           VALUES ('默认租户', 'default', '系统默认租户', '{}', 'active')
           ON CONFLICT (slug) DO NOTHING"#
    ))
    .await
    .expect("Failed to create default tenant");

    // 初始化默认菜单
    service::menu::MenuService::init_default_menus(&db).await.expect("Failed to initialize default menus");

    tracing::info!("Database migration completed");

    let jwt_config = JwtConfig::new(config.jwt.secret)
        .with_expires_in(config.jwt.expires_in)
        .with_refresh_expires_in(config.jwt.refresh_expires_in);
    let jwt_service = JwtService::new(jwt_config);

    let app = create_app(db.clone(), jwt_service);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

fn create_app(
    db: sea_orm::DatabaseConnection,
    jwt_service: JwtService,
) -> Router {
    let auth_state = AuthState::new(jwt_service, db.clone());

    // 租户子资源路由需要放在租户路由前面，避免被 /tenants/{id} 匹配
    let api_v1 = Router::new()
        .nest("/tenants/:tenant_id", Router::new()
            .merge(create_organization_router(db.clone()))
            .merge(create_department_router(db.clone()))
            .merge(create_site_router(db.clone()))
            .merge(create_product_router(db.clone()))
            .merge(create_driver_router(db.clone()))
            .merge(create_node_router(db.clone()))
            .merge(create_namespace_router(db.clone()))
            .nest("/sites/:site_id", Router::new()
                .merge(create_device_instance_router(db.clone()))
            )
        )
        // 直接通过租户slug获取命名空间列表
        .merge(Router::new()
            .route("/tenant/:tenant_slug/namespaces", get(list_namespaces_by_tenant_slug))
            .with_state(db.clone())
        )
        // 其他路由
        .merge(create_tenant_router(db.clone()))
        .merge(create_menu_router(db.clone(), auth_state.clone()))
        .merge(create_role_router(db.clone(), auth_state.clone()))
        .merge(create_user_role_router(db.clone(), auth_state.clone()))
        .merge(create_role_menu_router(db.clone(), auth_state.clone()))
        .merge(create_user_router(db.clone(), auth_state.clone()))
        .merge(create_crd_router(db.clone(), auth_state.clone()))
        .merge(create_operator_router(db.clone(), auth_state.clone()))
        .merge(create_controller_router(db.clone(), auth_state.clone()))
        .merge(create_config_map_router(db.clone(), auth_state.clone()))
        .merge(create_secret_router(db.clone(), auth_state.clone()))
        .merge(create_logout_router(db, auth_state));

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_v1)
}

async fn health_check() -> &'static str {
    "OK"
}
