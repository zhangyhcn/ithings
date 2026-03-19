use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tenant::Table)
                    .col(
                        ColumnDef::new(Tenant::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Tenant::Name).text().not_null())
                    .col(ColumnDef::new(Tenant::Slug).text().not_null().unique_key())
                    .col(ColumnDef::new(Tenant::Description).text().null())
                    .col(ColumnDef::new(Tenant::Config).json().null())
                    .col(
                        ColumnDef::new(Tenant::Status)
                            .text()
                            .not_null()
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(Tenant::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Tenant::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(User::TenantId).uuid().not_null())
                    .col(ColumnDef::new(User::Username).text().not_null())
                    .col(ColumnDef::new(User::Email).text().not_null().unique_key())
                    .col(ColumnDef::new(User::PasswordHash).text().not_null())
                    .col(ColumnDef::new(User::Phone).text().null())
                    .col(ColumnDef::new(User::Role).text().not_null().default("user"))
                    .col(
                        ColumnDef::new(User::IsSuperuser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(User::LastLogin).timestamp().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_tenant_username")
                    .table(User::Table)
                    .col(User::TenantId)
                    .col(User::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .col(
                        ColumnDef::new(Role::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Role::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Role::Name).text().not_null())
                    .col(ColumnDef::new(Role::Slug).text().not_null())
                    .col(ColumnDef::new(Role::Description).text().null())
                    .col(ColumnDef::new(Role::Permissions).json().not_null().default("[]"))
                    .col(ColumnDef::new(Role::Status).text().not_null().default("active"))
                    .col(
                        ColumnDef::new(Role::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Role::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Role::Table, Role::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("roles_tenant_id_slug_unique")
                            .table(Role::Table)
                            .col(Role::TenantId)
                            .col(Role::Slug)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .col(
                        ColumnDef::new(UserRole::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserRole::TenantId).uuid().not_null())
                    .col(ColumnDef::new(UserRole::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRole::RoleId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserRole::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_user_role_user_id_role_id")
                            .table(UserRole::Table)
                            .col(UserRole::UserId)
                            .col(UserRole::RoleId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Site::Table)
                    .col(
                        ColumnDef::new(Site::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Site::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Site::Name).text().not_null())
                    .col(ColumnDef::new(Site::Slug).text().not_null().unique_key())
                    .col(ColumnDef::new(Site::Description).text().null())
                    .col(ColumnDef::new(Site::Location).text().null())
                    .col(ColumnDef::new(Site::Status).text().not_null().default("active"))
                    .col(ColumnDef::new(Site::Config).json().null())
                    .col(
                        ColumnDef::new(Site::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Site::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Site::Table, Site::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_site_tenant_slug")
                    .table(Site::Table)
                    .col(Site::TenantId)
                    .col(Site::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Namespace::Table)
                    .col(
                        ColumnDef::new(Namespace::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Namespace::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Namespace::Name).text().not_null())
                    .col(ColumnDef::new(Namespace::Slug).text().not_null().unique_key())
                    .col(ColumnDef::new(Namespace::Description).text().null())
                    .col(ColumnDef::new(Namespace::NamespaceType).text().not_null().default("default"))
                    .col(ColumnDef::new(Namespace::Config).json().null())
                    .col(ColumnDef::new(Namespace::Status).text().not_null().default("active"))
                    .col(
                        ColumnDef::new(Namespace::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Namespace::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Namespace::Table, Namespace::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_namespace_tenant_slug")
                    .table(Namespace::Table)
                    .col(Namespace::TenantId)
                    .col(Namespace::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::drop().table(Namespace::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        let stmt = Table::drop().table(Site::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        let stmt = Table::drop().table(UserRole::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        let stmt = Table::drop().table(Role::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        let stmt = Table::drop().table(User::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        let stmt = Table::drop().table(Tenant::Table).to_owned();
        manager.drop_table(stmt).await?;
        
        Ok(())
    }
}

#[derive(Iden)]
enum Tenant {
    Table,
    Id,
    Name,
    Slug,
    Description,
    Config,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Organization {
    Table,
    Id,
    TenantId,
    ParentId,
    Name,
    Slug,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Department {
    Table,
    Id,
    TenantId,
    OrganizationId,
    ParentId,
    Name,
    Slug,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    TenantId,
    Username,
    Email,
    PasswordHash,
    Phone,
    Role,
    IsSuperuser,
    IsActive,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
    TenantId,
    Name,
    Slug,
    Description,
    Permissions,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserRole {
    Table,
    Id,
    TenantId,
    UserId,
    RoleId,
    CreatedAt,
}

#[derive(Iden)]
enum Site {
    Table,
    Id,
    TenantId,
    Name,
    Slug,
    Description,
    Location,
    Status,
    Config,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Namespace {
    Table,
    Id,
    TenantId,
    Name,
    Slug,
    Description,
    NamespaceType,
    Config,
    Status,
    CreatedAt,
    UpdatedAt,
}
