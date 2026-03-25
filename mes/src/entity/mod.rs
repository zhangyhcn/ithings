pub mod product;
pub mod work_order;
pub mod process_route;
pub mod process;
pub mod schedule_plan;
pub mod material;
pub mod inventory;
pub mod stock_movement;
pub mod work_station;
pub mod production_record;
pub mod equipment;
pub mod maintenance_plan;
pub mod employee;
pub mod skill_certificate;
pub mod inspection_order;
pub mod defect_record;
pub mod warehouse;
pub mod location;
pub mod workshop;
pub mod production_line;

pub use product::Entity as ProductEntity;
pub use product::Model as ProductModel;
pub use product::Column as ProductColumn;

pub use work_order::Entity as WorkOrderEntity;
pub use work_order::Model as WorkOrderModel;
pub use work_order::Column as WorkOrderColumn;

pub use process_route::Entity as ProcessRouteEntity;
pub use process_route::Model as ProcessRouteModel;
pub use process_route::Column as ProcessRouteColumn;

pub use process::Entity as ProcessEntity;
pub use process::Model as ProcessModel;
pub use process::Column as ProcessColumn;

pub use schedule_plan::Entity as SchedulePlanEntity;
pub use schedule_plan::Model as SchedulePlanModel;
pub use schedule_plan::Column as SchedulePlanColumn;

pub use material::Entity as MaterialEntity;
pub use material::Model as MaterialModel;
pub use material::Column as MaterialColumn;

pub use inventory::Entity as InventoryEntity;
pub use inventory::Model as InventoryModel;
pub use inventory::Column as InventoryColumn;

pub use stock_movement::Entity as StockMovementEntity;
pub use stock_movement::Model as StockMovementModel;
pub use stock_movement::Column as StockMovementColumn;

pub use work_station::Entity as WorkStationEntity;
pub use work_station::Model as WorkStationModel;
pub use work_station::Column as WorkStationColumn;

pub use production_record::Entity as ProductionRecordEntity;
pub use production_record::Model as ProductionRecordModel;
pub use production_record::Column as ProductionRecordColumn;

pub use equipment::Entity as EquipmentEntity;
pub use equipment::Model as EquipmentModel;
pub use equipment::Column as EquipmentColumn;

pub use maintenance_plan::Entity as MaintenancePlanEntity;
pub use maintenance_plan::Model as MaintenancePlanModel;
pub use maintenance_plan::Column as MaintenancePlanColumn;

pub use employee::Entity as EmployeeEntity;
pub use employee::Model as EmployeeModel;
pub use employee::Column as EmployeeColumn;

pub use skill_certificate::Entity as SkillCertificateEntity;
pub use skill_certificate::Model as SkillCertificateModel;
pub use skill_certificate::Column as SkillCertificateColumn;

pub use inspection_order::Entity as InspectionOrderEntity;
pub use inspection_order::Model as InspectionOrderModel;
pub use inspection_order::Column as InspectionOrderColumn;

pub use defect_record::Entity as DefectRecordEntity;
pub use defect_record::Model as DefectRecordModel;
pub use defect_record::Column as DefectRecordColumn;

pub use warehouse::Entity as WarehouseEntity;
pub use warehouse::Model as WarehouseModel;
pub use warehouse::Column as WarehouseColumn;

pub use location::Entity as LocationEntity;
pub use location::Model as LocationModel;
pub use location::Column as LocationColumn;

pub use workshop::Entity as WorkshopEntity;
pub use workshop::Model as WorkshopModel;
pub use workshop::Column as WorkshopColumn;

pub use production_line::Entity as ProductionLineEntity;
pub use production_line::Model as ProductionLineModel;
pub use production_line::Column as ProductionLineColumn;
