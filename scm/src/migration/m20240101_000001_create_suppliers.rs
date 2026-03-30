use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Supplier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplier::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplier::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Supplier::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Supplier::SupplierCode).text().not_null())
                    .col(ColumnDef::new(Supplier::SupplierName).text().not_null())
                    .col(ColumnDef::new(Supplier::ContactPerson).text())
                    .col(ColumnDef::new(Supplier::ContactPhone).text())
                    .col(ColumnDef::new(Supplier::ContactEmail).text())
                    .col(ColumnDef::new(Supplier::Address).text())
                    .col(ColumnDef::new(Supplier::BankName).text())
                    .col(ColumnDef::new(Supplier::BankAccount).text())
                    .col(ColumnDef::new(Supplier::TaxNumber).text())
                    .col(ColumnDef::new(Supplier::SupplierType).text())
                    .col(ColumnDef::new(Supplier::CreditLevel).text())
                    .col(ColumnDef::new(Supplier::Remarks).text())
                    .col(ColumnDef::new(Supplier::Status).text().default("active"))
                    .col(ColumnDef::new(Supplier::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Supplier::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Supplier::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Supplier {
    Table,
    Id,
    TenantId,
    OrgId,
    SupplierCode,
    SupplierName,
    ContactPerson,
    ContactPhone,
    ContactEmail,
    Address,
    BankName,
    BankAccount,
    TaxNumber,
    SupplierType,
    CreditLevel,
    Remarks,
    Status,
    CreatedAt,
    UpdatedAt,
}
