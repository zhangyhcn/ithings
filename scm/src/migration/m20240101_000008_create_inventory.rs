pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Inventory::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Inventory::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::WarehouseId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::LocationId).uuid())
                    .col(ColumnDef::new(Inventory::MaterialId).uuid().not_null())
                    .col(ColumnDef::new(Inventory::BatchNo).string())
                    .col(ColumnDef::new(Inventory::Quantity).decimal().default(0))
                    .col(ColumnDef::new(Inventory::FrozenQty).decimal().default(0))
                    .col(ColumnDef::new(Inventory::AvailableQty).decimal().default(0))
                    .col(ColumnDef::new(Inventory::CostPrice).decimal())
                    .col(ColumnDef::new(Inventory::ProductionDate).timestamp())
                    .col(ColumnDef::new(Inventory::ExpiryDate).timestamp())
                    .col(ColumnDef::new(Inventory::Status).string().default("normal"))
                    .col(ColumnDef::new(Inventory::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Inventory::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_warehouse")
                            .from(Inventory::Table, Inventory::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_location")
                            .from(Inventory::Table, Inventory::LocationId)
                            .to(WarehouseLocation::Table, WarehouseLocation::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_material")
                            .from(Inventory::Table, Inventory::MaterialId)
                            .to(Material::Table, Material::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_inventory_tenant_org")
                            .col(Inventory::TenantId)
                            .col(Inventory::OrgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Inventory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Inventory {
    Table,
    Id,
    TenantId,
    OrgId,
    WarehouseId,
    LocationId,
    MaterialId,
    BatchNo,
    Quantity,
    FrozenQty,
    AvailableQty,
    CostPrice,
    ProductionDate,
    ExpiryDate,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Warehouse {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum WarehouseLocation {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Material {
    Table,
    Id,
}
