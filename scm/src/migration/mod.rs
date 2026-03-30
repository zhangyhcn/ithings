pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_suppliers;
mod m20240101_000002_create_purchase_orders;
mod m20240101_000003_create_purchase_order_items;
mod m20240101_000004_create_material_categories;
mod m20240101_000005_create_materials;
mod m20240101_000006_create_warehouses;
mod m20240101_000007_create_warehouse_locations;
mod m20240101_000008_create_inventory;

pub use m20240101_000001_create_suppliers::Migration as CreateSuppliers;
pub use m20240101_000002_create_purchase_orders::Migration as CreatePurchaseOrders;
pub use m20240101_000003_create_purchase_order_items::Migration as CreatePurchaseOrderItems;
pub use m20240101_000004_create_material_categories::Migration as CreateMaterialCategories;
pub use m20240101_000005_create_materials::Migration as CreateMaterials;
pub use m20240101_000006_create_warehouses::Migration as CreateWarehouses;
pub use m20240101_000007_create_warehouse_locations::Migration as CreateWarehouseLocations;
pub use m20240101_000008_create_inventory::Migration as CreateInventory;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateSuppliers),
            Box::new(CreatePurchaseOrders),
            Box::new(CreatePurchaseOrderItems),
            Box::new(CreateMaterialCategories),
            Box::new(CreateMaterials),
            Box::new(CreateWarehouses),
            Box::new(CreateWarehouseLocations),
            Box::new(CreateInventory),
        ]
    }
}

pub async fn run_migrations(db: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::up(db, None).await
}
