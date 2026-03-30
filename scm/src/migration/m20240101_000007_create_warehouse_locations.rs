pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WarehouseLocation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WarehouseLocation::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WarehouseLocation::TenantId).uuid().not_null())
                    .col(ColumnDef::new(WarehouseLocation::OrgId).uuid().not_null())
                    .col(ColumnDef::new(WarehouseLocation::WarehouseId).uuid().not_null())
                    .col(ColumnDef::new(WarehouseLocation::Zone).string())
                    .col(ColumnDef::new(WarehouseLocation::RowNum).integer())
                    .col(ColumnDef::new(WarehouseLocation::Shelf).string())
                    .col(ColumnDef::new(WarehouseLocation::Level).integer())
                    .col(ColumnDef::new(WarehouseLocation::LocationCode).string().not_null())
                    .col(ColumnDef::new(WarehouseLocation::LocationName).string())
                    .col(ColumnDef::new(WarehouseLocation::LocationType).string().default("normal"))
                    .col(ColumnDef::new(WarehouseLocation::Capacity).decimal().default(0))
                    .col(ColumnDef::new(WarehouseLocation::Status).string().default("active"))
                    .col(ColumnDef::new(WarehouseLocation::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(WarehouseLocation::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_warehouse_location_warehouse")
                            .from(WarehouseLocation::Table, WarehouseLocation::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_warehouse_location_tenant_org")
                            .col(WarehouseLocation::TenantId)
                            .col(WarehouseLocation::OrgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WarehouseLocation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WarehouseLocation {
    Table,
    Id,
    TenantId,
    OrgId,
    WarehouseId,
    Zone,
    RowNum,
    Shelf,
    Level,
    LocationCode,
    LocationName,
    LocationType,
    Capacity,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Warehouse {
    Table,
    Id,
}
