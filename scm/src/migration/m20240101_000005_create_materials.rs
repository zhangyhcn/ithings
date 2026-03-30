pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Material::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Material::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Material::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Material::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Material::CategoryId).uuid())
                    .col(ColumnDef::new(Material::Code).string().not_null())
                    .col(ColumnDef::new(Material::Name).string().not_null())
                    .col(ColumnDef::new(Material::Specification).string())
                    .col(ColumnDef::new(Material::Model).string())
                    .col(ColumnDef::new(Material::Unit).string().not_null())
                    .col(ColumnDef::new(Material::UnitWeight).decimal())
                    .col(ColumnDef::new(Material::UnitVolume).decimal())
                    .col(ColumnDef::new(Material::Barcode).string())
                    .col(ColumnDef::new(Material::Status).string().default("active"))
                    .col(ColumnDef::new(Material::SafetyStock).decimal().default(0))
                    .col(ColumnDef::new(Material::MaxStock).decimal().default(0))
                    .col(ColumnDef::new(Material::MinOrderQty).decimal().default(0))
                    .col(ColumnDef::new(Material::LeadTime).integer().default(0))
                    .col(ColumnDef::new(Material::PurchasePrice).decimal())
                    .col(ColumnDef::new(Material::SalePrice).decimal())
                    .col(ColumnDef::new(Material::CostPrice).decimal())
                    .col(ColumnDef::new(Material::Remark).text())
                    .col(ColumnDef::new(Material::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Material::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .index(
                        Index::create()
                            .name("idx_material_tenant_org")
                            .col(Material::TenantId)
                            .col(Material::OrgId),
                    )
                    .index(Index::create().name("idx_material_code").col(Material::Code))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Material::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Material {
    Table,
    Id,
    TenantId,
    OrgId,
    CategoryId,
    Code,
    Name,
    Specification,
    Model,
    Unit,
    UnitWeight,
    UnitVolume,
    Barcode,
    Status,
    SafetyStock,
    MaxStock,
    MinOrderQty,
    LeadTime,
    PurchasePrice,
    SalePrice,
    CostPrice,
    Remark,
    CreatedAt,
    UpdatedAt,
}
