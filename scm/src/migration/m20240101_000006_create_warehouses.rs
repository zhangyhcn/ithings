pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Warehouse::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Warehouse::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Warehouse::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Warehouse::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Warehouse::Code).string().not_null())
                    .col(ColumnDef::new(Warehouse::Name).string().not_null())
                    .col(ColumnDef::new(Warehouse::WarehouseType).string().default("normal"))
                    .col(ColumnDef::new(Warehouse::Address).text())
                    .col(ColumnDef::new(Warehouse::Manager).string())
                    .col(ColumnDef::new(Warehouse::Phone).string())
                    .col(ColumnDef::new(Warehouse::Status).string().default("active"))
                    .col(ColumnDef::new(Warehouse::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Warehouse::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .index(
                        Index::create()
                            .name("idx_warehouse_tenant_org")
                            .col(Warehouse::TenantId)
                            .col(Warehouse::OrgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Warehouse::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Warehouse {
    Table,
    Id,
    TenantId,
    OrgId,
    Code,
    Name,
    WarehouseType,
    Address,
    Manager,
    Phone,
    Status,
    CreatedAt,
    UpdatedAt,
}
