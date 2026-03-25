use sea_orm::{DatabaseBackend, DatabaseConnection, Statement, ConnectionTrait};

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let migrations = vec![
        // 产品表
        r#"CREATE TABLE IF NOT EXISTS mes_products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            product_no VARCHAR(50) NOT NULL UNIQUE,
            name VARCHAR(200) NOT NULL,
            specification VARCHAR(200),
            unit VARCHAR(20),
            product_type VARCHAR(50),
            description TEXT,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 工单表
        r#"CREATE TABLE IF NOT EXISTS mes_work_orders (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            order_no VARCHAR(50) NOT NULL UNIQUE,
            erp_order_no VARCHAR(50),
            product_id UUID NOT NULL,
            product_name VARCHAR(200) NOT NULL,
            quantity DECIMAL(18,4) NOT NULL,
            completed_qty DECIMAL(18,4) DEFAULT 0,
            status VARCHAR(20) DEFAULT 'pending',
            priority INTEGER DEFAULT 0,
            plan_start_time TIMESTAMP,
            plan_end_time TIMESTAMP,
            actual_start_time TIMESTAMP,
            actual_end_time TIMESTAMP,
            workshop_id UUID,
            production_line_id UUID,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 工艺路线表
        r#"CREATE TABLE IF NOT EXISTS mes_process_routes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            product_id UUID NOT NULL,
            route_name VARCHAR(100) NOT NULL,
            version VARCHAR(20) DEFAULT '1.0',
            status VARCHAR(20) DEFAULT 'draft',
            is_default BOOLEAN DEFAULT false,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 工序表
        r#"CREATE TABLE IF NOT EXISTS mes_processes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            route_id UUID NOT NULL,
            process_no VARCHAR(20) NOT NULL,
            process_name VARCHAR(100) NOT NULL,
            sequence INTEGER NOT NULL,
            work_station_id UUID,
            standard_time DECIMAL(10,2),
            setup_time DECIMAL(10,2),
            process_params JSONB,
            next_process_id UUID,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 排程计划表
        r#"CREATE TABLE IF NOT EXISTS mes_schedule_plans (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            plan_no VARCHAR(50) NOT NULL UNIQUE,
            work_order_id UUID NOT NULL,
            process_id UUID NOT NULL,
            equipment_id UUID,
            operator_id UUID,
            plan_quantity DECIMAL(18,4) NOT NULL,
            status VARCHAR(20) DEFAULT 'pending',
            start_time TIMESTAMP,
            end_time TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 物料表
        r#"CREATE TABLE IF NOT EXISTS mes_materials (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            material_no VARCHAR(50) NOT NULL UNIQUE,
            material_name VARCHAR(200) NOT NULL,
            specification VARCHAR(200),
            unit VARCHAR(20),
            material_type VARCHAR(50),
            safety_stock DECIMAL(18,4),
            max_stock DECIMAL(18,4),
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 仓库表
        r#"CREATE TABLE IF NOT EXISTS mes_warehouses (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            warehouse_no VARCHAR(50) NOT NULL UNIQUE,
            warehouse_name VARCHAR(100) NOT NULL,
            warehouse_type VARCHAR(50),
            location VARCHAR(200),
            description TEXT,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 库位表
        r#"CREATE TABLE IF NOT EXISTS mes_locations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            warehouse_id UUID NOT NULL,
            location_no VARCHAR(50) NOT NULL,
            location_name VARCHAR(100) NOT NULL,
            location_type VARCHAR(50),
            description TEXT,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 库存表
        r#"CREATE TABLE IF NOT EXISTS mes_inventories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            material_id UUID NOT NULL,
            warehouse_id UUID,
            location_id UUID,
            batch_no VARCHAR(50),
            quantity DECIMAL(18,4) DEFAULT 0,
            locked_qty DECIMAL(18,4) DEFAULT 0,
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            UNIQUE(material_id, warehouse_id, location_id, batch_no)
        )"#,
        // 出入库单表
        r#"CREATE TABLE IF NOT EXISTS mes_stock_movements (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            movement_no VARCHAR(50) NOT NULL UNIQUE,
            movement_type VARCHAR(20) NOT NULL,
            work_order_id UUID,
            material_id UUID NOT NULL,
            quantity DECIMAL(18,4) NOT NULL,
            batch_no VARCHAR(50),
            operator_id UUID,
            status VARCHAR(20) DEFAULT 'pending',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 工站表
        r#"CREATE TABLE IF NOT EXISTS mes_work_stations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            station_no VARCHAR(50) NOT NULL UNIQUE,
            station_name VARCHAR(100) NOT NULL,
            workshop_id UUID,
            production_line_id UUID,
            equipment_id UUID,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 生产记录表
        r#"CREATE TABLE IF NOT EXISTS mes_production_records (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            work_order_id UUID NOT NULL,
            process_id UUID NOT NULL,
            equipment_id UUID,
            operator_id UUID,
            batch_no VARCHAR(50),
            quantity DECIMAL(18,4) NOT NULL,
            good_qty DECIMAL(18,4),
            defect_qty DECIMAL(18,4),
            start_time TIMESTAMP,
            end_time TIMESTAMP,
            process_data JSONB,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 检验单表
        r#"CREATE TABLE IF NOT EXISTS mes_inspection_orders (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            inspection_no VARCHAR(50) NOT NULL UNIQUE,
            inspection_type VARCHAR(20) NOT NULL,
            work_order_id UUID,
            material_id UUID,
            batch_no VARCHAR(50),
            sample_qty INTEGER,
            pass_qty INTEGER,
            defect_qty INTEGER,
            result VARCHAR(20) DEFAULT 'pending',
            inspector_id UUID,
            inspect_time TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 不良记录表
        r#"CREATE TABLE IF NOT EXISTS mes_defect_records (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            inspection_id UUID NOT NULL,
            defect_type_id UUID,
            defect_code VARCHAR(50),
            quantity INTEGER NOT NULL,
            description TEXT,
            disposition VARCHAR(20) DEFAULT 'pending',
            status VARCHAR(20) DEFAULT 'pending',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 设备表
        r#"CREATE TABLE IF NOT EXISTS mes_equipments (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            equipment_no VARCHAR(50) NOT NULL UNIQUE,
            equipment_name VARCHAR(100) NOT NULL,
            equipment_type VARCHAR(50),
            model VARCHAR(100),
            manufacturer VARCHAR(100),
            purchase_date DATE,
            workshop_id UUID,
            status VARCHAR(20) DEFAULT 'idle',
            ip_address VARCHAR(50),
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 保养计划表
        r#"CREATE TABLE IF NOT EXISTS mes_maintenance_plans (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            equipment_id UUID NOT NULL,
            plan_type VARCHAR(20) NOT NULL,
            plan_date DATE,
            content TEXT,
            status VARCHAR(20) DEFAULT 'pending',
            executor_id UUID,
            execute_time TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 员工表
        r#"CREATE TABLE IF NOT EXISTS mes_employees (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            employee_no VARCHAR(20) NOT NULL UNIQUE,
            name VARCHAR(50) NOT NULL,
            department_id UUID,
            position VARCHAR(50),
            phone VARCHAR(20),
            status VARCHAR(20) DEFAULT 'active',
            entry_date DATE,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 技能证书表
        r#"CREATE TABLE IF NOT EXISTS mes_skill_certificates (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            employee_id UUID NOT NULL,
            skill_type VARCHAR(50),
            certificate_no VARCHAR(50),
            level VARCHAR(20),
            issue_date DATE,
            expire_date DATE,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 车间表
        r#"CREATE TABLE IF NOT EXISTS mes_workshops (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            workshop_no VARCHAR(50) NOT NULL UNIQUE,
            workshop_name VARCHAR(100) NOT NULL,
            location VARCHAR(200),
            description TEXT,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
        // 产线表
        r#"CREATE TABLE IF NOT EXISTS mes_production_lines (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id UUID NOT NULL,
            workshop_id UUID,
            line_no VARCHAR(50) NOT NULL UNIQUE,
            line_name VARCHAR(100) NOT NULL,
            description TEXT,
            status VARCHAR(20) DEFAULT 'active',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )"#,
    ];

    for sql in migrations {
        db.execute(Statement::from_string(DatabaseBackend::Postgres, sql))
            .await?;
    }

    Ok(())
}
