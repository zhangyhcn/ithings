pub mod product;
pub mod work_order;
pub mod material;
pub mod equipment;
pub mod employee;

pub use product::create_product_router;
pub use work_order::create_work_order_router;
pub use material::create_material_router;
pub use equipment::create_equipment_router;
pub use employee::create_employee_router;
