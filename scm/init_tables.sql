-- SCM 基础表结构

-- 供应商表
CREATE TABLE IF NOT EXISTS scm_suppliers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    supplier_code TEXT NOT NULL,
    supplier_name TEXT NOT NULL,
    contact_person TEXT,
    contact_phone TEXT,
    contact_email TEXT,
    address TEXT,
    bank_name TEXT,
    bank_account TEXT,
    tax_number TEXT,
    supplier_type TEXT,
    credit_level TEXT,
    remarks TEXT,
    status TEXT DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 物料分类表
CREATE TABLE IF NOT EXISTS material_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    parent_id UUID,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 物料表
CREATE TABLE IF NOT EXISTS materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    category_id UUID,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(200) NOT NULL,
    specification VARCHAR(200),
    model VARCHAR(100),
    unit VARCHAR(20) NOT NULL,
    unit_weight DECIMAL(15,3),
    unit_volume DECIMAL(15,3),
    barcode VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active',
    safety_stock DECIMAL(15,3) DEFAULT 0,
    max_stock DECIMAL(15,3) DEFAULT 0,
    min_order_qty DECIMAL(15,3) DEFAULT 0,
    lead_time INTEGER DEFAULT 0,
    purchase_price DECIMAL(15,2),
    sale_price DECIMAL(15,2),
    cost_price DECIMAL(15,2),
    remark TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 仓库表
CREATE TABLE IF NOT EXISTS warehouses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(200) NOT NULL,
    warehouse_type VARCHAR(20) DEFAULT 'normal',
    address TEXT,
    manager VARCHAR(100),
    phone VARCHAR(50),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 仓库库位表
CREATE TABLE IF NOT EXISTS warehouse_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    warehouse_id UUID NOT NULL,
    zone VARCHAR(50),
    row_num INTEGER,
    shelf VARCHAR(50),
    level INTEGER,
    location_code VARCHAR(50) NOT NULL,
    location_name VARCHAR(100),
    location_type VARCHAR(20) DEFAULT 'normal',
    capacity DECIMAL(15,3) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 库存表
CREATE TABLE IF NOT EXISTS inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    warehouse_id UUID NOT NULL,
    location_id UUID,
    material_id UUID NOT NULL,
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) DEFAULT 0,
    frozen_qty DECIMAL(15,3) DEFAULT 0,
    available_qty DECIMAL(15,3) DEFAULT 0,
    cost_price DECIMAL(15,2),
    production_date TIMESTAMP,
    expiry_date TIMESTAMP,
    status VARCHAR(20) DEFAULT 'normal',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 采购订单表
CREATE TABLE IF NOT EXISTS scm_purchase_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL,
    order_date DATE NOT NULL,
    expected_delivery_date DATE,
    payment_terms TEXT,
    delivery_address TEXT,
    contact_person VARCHAR(100),
    contact_phone VARCHAR(50),
    total_amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10),
    remarks TEXT,
    status VARCHAR(20) DEFAULT 'draft',
    created_by UUID,
    approved_by UUID,
    approved_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);

-- 采购订单明细表
CREATE TABLE IF NOT EXISTS scm_purchase_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_id UUID NOT NULL,
    material_id UUID NOT NULL,
    material_name TEXT NOT NULL,
    specification TEXT,
    unit TEXT,
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2) NOT NULL,
    total_price DECIMAL(15,2) NOT NULL,
    expected_delivery_date DATE,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 供应商报价表
CREATE TABLE IF NOT EXISTS supplier_quotations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    supplier_id UUID NOT NULL,
    material_id UUID NOT NULL,
    price DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    min_qty DECIMAL(15,3) DEFAULT 1,
    max_qty DECIMAL(15,3),
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    lead_time INTEGER DEFAULT 0,
    payment_terms TEXT,
    remarks TEXT,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 招投标表
CREATE TABLE IF NOT EXISTS biddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bidding_no VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    bidding_type VARCHAR(20) DEFAULT 'public',
    publish_date DATE NOT NULL,
    deadline TIMESTAMP NOT NULL,
    contact_person VARCHAR(100),
    contact_phone VARCHAR(50),
    description TEXT,
    requirements TEXT,
    status VARCHAR(20) DEFAULT 'draft',
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, bidding_no)
);

-- 招投标明细表
CREATE TABLE IF NOT EXISTS bidding_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bidding_id UUID NOT NULL,
    material_id UUID NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    specification TEXT,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 投标记录表
CREATE TABLE IF NOT EXISTS bidding_bids (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bidding_id UUID NOT NULL,
    supplier_id UUID NOT NULL,
    bidding_date TIMESTAMP NOT NULL DEFAULT NOW(),
    total_amount DECIMAL(15,2),
    remarks TEXT,
    status VARCHAR(20) DEFAULT 'submitted',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 投标明细表
CREATE TABLE IF NOT EXISTS bidding_bid_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bid_id UUID NOT NULL,
    material_id UUID NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2) NOT NULL,
    total_price DECIMAL(15,2) NOT NULL,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 采购合同表
CREATE TABLE IF NOT EXISTS contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    contract_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL,
    title VARCHAR(200) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    total_amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    payment_terms TEXT,
    delivery_terms TEXT,
    quality_terms TEXT,
    remarks TEXT,
    status VARCHAR(20) DEFAULT 'draft',
    signed_by UUID,
    signed_at TIMESTAMP,
    created_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, contract_no)
);

-- 库存盘点单表
CREATE TABLE IF NOT EXISTS stocktaking_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    stocktaking_no VARCHAR(50) NOT NULL,
    warehouse_id UUID NOT NULL,
    stocktaking_date DATE NOT NULL,
    stocktaking_type VARCHAR(20) DEFAULT 'full',
    status VARCHAR(20) DEFAULT 'draft',
    remarks TEXT,
    created_by UUID,
    confirmed_by UUID,
    confirmed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, stocktaking_no)
);

-- 库存盘点明细表
CREATE TABLE IF NOT EXISTS stocktaking_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    stocktaking_id UUID NOT NULL,
    material_id UUID NOT NULL,
    location_id UUID,
    system_qty DECIMAL(15,3) NOT NULL DEFAULT 0,
    actual_qty DECIMAL(15,3) NOT NULL DEFAULT 0,
    variance_qty DECIMAL(15,3) GENERATED ALWAYS AS (actual_qty - system_qty) STORED,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_scm_supplier_tenant_org ON scm_suppliers(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_material_category_tenant_org ON material_categories(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_material_tenant_org ON materials(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_material_code ON materials(code);
CREATE INDEX IF NOT EXISTS idx_warehouse_tenant_org ON warehouses(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_warehouse_location_tenant_org ON warehouse_locations(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_inventory_tenant_org ON inventory(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_scm_purchase_order_tenant_org ON scm_purchase_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_supplier_quotation_tenant_org ON supplier_quotations(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_supplier_quotation_material ON supplier_quotations(material_id);
CREATE INDEX IF NOT EXISTS idx_bidding_tenant_org ON biddings(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_bidding_item_bidding ON bidding_items(bidding_id);
CREATE INDEX IF NOT EXISTS idx_contract_tenant_org ON contracts(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_stocktaking_tenant_org ON stocktaking_orders(tenant_id, org_id);
CREATE INDEX IF NOT EXISTS idx_stocktaking_item_order ON stocktaking_items(stocktaking_id);
