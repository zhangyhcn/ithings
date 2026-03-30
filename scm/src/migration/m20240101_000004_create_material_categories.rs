pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MaterialCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MaterialCategory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MaterialCategory::TenantId).uuid().not_null())
                    .col(ColumnDef::new(MaterialCategory::OrgId).uuid().not_null())
                    .col(ColumnDef::new(MaterialCategory::ParentId).uuid())
                    .col(ColumnDef::new(MaterialCategory::Code).string().not_null())
                    .col(ColumnDef::new(MaterialCategory::Name).string().not_null())
                    .col(ColumnDef::new(MaterialCategory::Description).text())
                    .col(ColumnDef::new(MaterialCategory::SortOrder).integer().default(0))
                    .col(ColumnDef::new(MaterialCategory::Status).string().default("active"))
                    .col(ColumnDef::new(MaterialCategory::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(MaterialCategory::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .index(
                        Index::create()
                            .name("idx_material_category_tenant_org")
                            .col(MaterialCategory::TenantId)
                            .col(MaterialCategory::OrgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MaterialCategory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MaterialCategory {
    Table,
    Id,
    TenantId,
    OrgId,
    ParentId,
    Code,
    Name,
    Description,
    SortOrder,
    Status,
    CreatedAt,
    UpdatedAt,
}
