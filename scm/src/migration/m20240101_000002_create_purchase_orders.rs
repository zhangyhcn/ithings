use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrder::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PurchaseOrder::TenantId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrder::OrgId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrder::OrderNo).text().not_null())
                    .col(ColumnDef::new(PurchaseOrder::SupplierId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrder::OrderDate).date().not_null())
                    .col(ColumnDef::new(PurchaseOrder::ExpectedDeliveryDate).date())
                    .col(ColumnDef::new(PurchaseOrder::PaymentTerms).text())
                    .col(ColumnDef::new(PurchaseOrder::DeliveryAddress).text())
                    .col(ColumnDef::new(PurchaseOrder::ContactPerson).text())
                    .col(ColumnDef::new(PurchaseOrder::ContactPhone).text())
                    .col(ColumnDef::new(PurchaseOrder::TotalAmount).decimal().not_null())
                    .col(ColumnDef::new(PurchaseOrder::Currency).text())
                    .col(ColumnDef::new(PurchaseOrder::Remarks).text())
                    .col(ColumnDef::new(PurchaseOrder::Status).text().default("draft"))
                    .col(ColumnDef::new(PurchaseOrder::CreatedBy).uuid())
                    .col(ColumnDef::new(PurchaseOrder::ApprovedBy).uuid())
                    .col(ColumnDef::new(PurchaseOrder::ApprovedAt).timestamp())
                    .col(ColumnDef::new(PurchaseOrder::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PurchaseOrder::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseOrder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PurchaseOrder {
    Table,
    Id,
    TenantId,
    OrgId,
    OrderNo,
    SupplierId,
    OrderDate,
    ExpectedDeliveryDate,
    PaymentTerms,
    DeliveryAddress,
    ContactPerson,
    ContactPhone,
    TotalAmount,
    Currency,
    Remarks,
    Status,
    CreatedBy,
    ApprovedBy,
    ApprovedAt,
    CreatedAt,
    UpdatedAt,
}
