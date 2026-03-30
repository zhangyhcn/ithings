use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrderItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PurchaseOrderItem::TenantId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::OrgId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::OrderId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::MaterialId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::MaterialName).text().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::Specification).text())
                    .col(ColumnDef::new(PurchaseOrderItem::Unit).text())
                    .col(ColumnDef::new(PurchaseOrderItem::Quantity).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::UnitPrice).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::ExpectedDeliveryDate).date())
                    .col(ColumnDef::new(PurchaseOrderItem::Remarks).text())
                    .col(ColumnDef::new(PurchaseOrderItem::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PurchaseOrderItem::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseOrderItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PurchaseOrderItem {
    Table,
    Id,
    TenantId,
    OrgId,
    OrderId,
    MaterialId,
    MaterialName,
    Specification,
    Unit,
    Quantity,
    UnitPrice,
    TotalPrice,
    ExpectedDeliveryDate,
    Remarks,
    CreatedAt,
    UpdatedAt,
}
