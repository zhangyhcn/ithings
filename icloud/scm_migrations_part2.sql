-- SCM 扩展表结构迁移脚本
-- 执行方式：docker exec -i postgres psql -U postgres -d scm < scm_migrations_part2.sql

-- 入库单表
CREATE TABLE IF NOT EXISTS inbound_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    order_type VARCHAR(20) NOT NULL,
    source_order_id UUID,
    warehouse_id UUID NOT NULL,
    supplier_id UUID,
    status VARCHAR(20) DEFAULT 'draft',
    total_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);

-- 入库单明细表
CREATE TABLE IF NOT EXISTS inbound_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    inbound_order_id UUID NOT NULL,
    material_id UUID NOT NULL,
    location_id UUID,
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2),
    production_date TIMESTAMP,
    expiry_date TIMESTAMP,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 出库单表
CREATE TABLE IF NOT EXISTS outbound_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    order_type VARCHAR(20) NOT NULL,
    source_order_id UUID,
    warehouse_id UUID NOT NULL,
    status VARCHAR(20) DEFAULT 'draft',
    total_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);

-- 出库单明细表
CREATE TABLE IF NOT EXISTS outbound_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    outbound_order_id UUID NOT NULL,
    material_id UUID NOT NULL,
    location_id UUID,
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2),
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 库存移动记录表
CREATE TABLE IF NOT EXISTS inventory_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    movement_no VARCHAR(50) NOT NULL,
    movement_type VARCHAR(20) NOT NULL,
    warehouse_id UUID NOT NULL,
    from_location_id UUID,
    to_location_id UUID,
    material_id UUID NOT NULL,
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    reason TEXT,
    operator VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, movement_no)
);

-- BOM表
CREATE TABLE IF NOT EXISTS boms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    material_id UUID NOT NULL,
    version VARCHAR(20) NOT NULL,
    name VARCHAR(200) NOT NULL,
    status VARCHAR(20) DEFAULT 'active',
    effective_date TIMESTAMP,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, material_id, version)
);

-- BOM明细表
CREATE TABLE IF NOT EXISTS bom_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bom_id UUID NOT NULL,
    material_id UUID NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    scrap_rate DECIMAL(5,2) DEFAULT 0,
    sequence INT DEFAULT 0,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 生产订单表
CREATE TABLE IF NOT EXISTS production_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    material_id UUID NOT NULL,
    bom_id UUID,
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    planned_start_date TIMESTAMP,
    planned_end_date TIMESTAMP,
    actual_start_date TIMESTAMP,
    actual_end_date TIMESTAMP,
    status VARCHAR(20) DEFAULT 'planned',
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);

-- 销售订单表
CREATE TABLE IF NOT EXISTS sales_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    customer_name VARCHAR(200) NOT NULL,
    customer_contact VARCHAR(100),
    customer_phone VARCHAR(50),
    customer_address TEXT,
    order_date TIMESTAMP DEFAULT NOW(),
    delivery_date TIMESTAMP,
    total_amount DECIMAL(15,2) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'draft',
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);

-- 销售订单明细表
CREATE TABLE IF NOT EXISTS sales_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    sales_order_id UUID NOT NULL,
    material_id UUID NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    unit_price DECIMAL(15,2) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    delivered_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 发货单表
CREATE TABLE IF NOT EXISTS delivery_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    delivery_no VARCHAR(50) NOT NULL,
    sales_order_id UUID,
    warehouse_id UUID NOT NULL,
    carrier VARCHAR(100),
    tracking_no VARCHAR(100),
    status VARCHAR(20) DEFAULT 'pending',
    ship_date TIMESTAMP,
    delivery_date TIMESTAMP,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, delivery_no)
);

-- 应付账款表
CREATE TABLE IF NOT EXISTS accounts_payable (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    payable_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL,
    source_type VARCHAR(20) NOT NULL,
    source_order_id UUID,
    total_amount DECIMAL(15,2) NOT NULL,
    paid_amount DECIMAL(15,2) DEFAULT 0,
    currency VARCHAR(10) DEFAULT 'CNY',
    due_date TIMESTAMP,
    status VARCHAR(20) DEFAULT 'unpaid',
    payment_date TIMESTAMP,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, payable_no)
);

-- 成本核算表
CREATE TABLE IF NOT EXISTS cost_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    material_id UUID NOT NULL,
    cost_type VARCHAR(20) NOT NULL,
    period VARCHAR(20) NOT NULL,
    total_cost DECIMAL(15,2) NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit_cost DECIMAL(15,2) NOT NULL,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, material_id, cost_type, period)
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_inbound_tenant_org ON inbound_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_outbound_tenant_org ON outbound_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_movement_tenant_org ON inventory_movements(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_bom_tenant_org ON boms(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_production_tenant_org ON production_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_sales_tenant_org ON sales_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tenant_org ON delivery_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_payable_tenant_org ON accounts_payable(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_cost_tenant_org ON cost_records(tenant_id, org_id);
