# SCM 数据库设计方案

## 一级菜单结构

```
供应链管理 (SCM)
├── 基础数据
│   ├── 物料管理
│   ├── 仓库管理
│   └── 库存查询
├── 采购管理
│   ├── 供应商管理（已实现）
│   ├── 采购订单（已实现）
│   ├── 供应商报价
│   ├── 招投标管理
│   └── 采购合同
├── 仓储管理
│   ├── 入库管理
│   ├── 出库管理
│   ├── 库内作业
│   └── 库存盘点
├── 生产管理
│   ├── BOM管理
│   ├── 生产计划
│   └── 车间管理
├── 订单管理
│   ├── 销售订单
│   └── 发货管理
├── 退货管理
│   ├── 退货申请
│   └── 维修管理
├── 财务结算
│   ├── 应付账款
│   └── 成本核算
└── 数据分析
    ├── 库存分析
    ├── 采购分析
    └── 供应商绩效
```

## 数据表设计

### 1. 基础数据模块

#### 1.1 物料分类表 (material_categories)
```sql
CREATE TABLE material_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    parent_id UUID REFERENCES material_categories(id),
    code VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    sort_order INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, code)
);
```

#### 1.2 物料主数据表 (materials)
```sql
CREATE TABLE materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    category_id UUID REFERENCES material_categories(id),
    code VARCHAR(50) NOT NULL,
    name VARCHAR(200) NOT NULL,
    specification VARCHAR(200),
    model VARCHAR(100),
    unit VARCHAR(20) NOT NULL,
    unit_weight DECIMAL(10,3),
    unit_volume DECIMAL(10,3),
    barcode VARCHAR(100),
    status VARCHAR(20) DEFAULT 'active',
    safety_stock DECIMAL(15,3) DEFAULT 0,
    max_stock DECIMAL(15,3) DEFAULT 0,
    min_order_qty DECIMAL(15,3) DEFAULT 0,
    lead_time INT DEFAULT 0,
    purchase_price DECIMAL(15,2),
    sale_price DECIMAL(15,2),
    cost_price DECIMAL(15,2),
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, code)
);
```

#### 1.3 仓库表 (warehouses)
```sql
CREATE TABLE warehouses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(20) DEFAULT 'normal', -- normal, cold, hazardous
    address TEXT,
    manager VARCHAR(100),
    phone VARCHAR(50),
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, code)
);
```

#### 1.4 库位表 (warehouse_locations)
```sql
CREATE TABLE warehouse_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    zone VARCHAR(50), -- A区、B区
    row_num INT,
    shelf VARCHAR(20), -- 货架号
    level INT, -- 层
    location_code VARCHAR(50) NOT NULL,
    location_name VARCHAR(100),
    location_type VARCHAR(20) DEFAULT 'normal', -- normal, picking, storage
    capacity DECIMAL(15,3) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, warehouse_id, location_code)
);
```

#### 1.5 库存表 (inventory)
```sql
CREATE TABLE inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    location_id UUID REFERENCES warehouse_locations(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) DEFAULT 0,
    frozen_qty DECIMAL(15,3) DEFAULT 0,
    available_qty DECIMAL(15,3) DEFAULT 0,
    cost_price DECIMAL(15,2),
    production_date DATE,
    expiry_date DATE,
    status VARCHAR(20) DEFAULT 'normal',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, warehouse_id, material_id, batch_no)
);
```

### 2. 采购与寻源模块

#### 2.1 供应商报价表 (supplier_quotations)
```sql
CREATE TABLE supplier_quotations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    supplier_id UUID NOT NULL REFERENCES suppliers(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    quotation_no VARCHAR(50) NOT NULL,
    price DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    min_order_qty DECIMAL(15,3) DEFAULT 0,
    lead_time INT DEFAULT 0,
    valid_from DATE,
    valid_until DATE,
    status VARCHAR(20) DEFAULT 'active',
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, quotation_no)
);
```

#### 2.2 招投标项目表 (bidding_projects)
```sql
CREATE TABLE bidding_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    project_no VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    project_type VARCHAR(20) DEFAULT 'public', -- public, invite
    status VARCHAR(20) DEFAULT 'draft', -- draft, published, bidding, awarded, closed
    publish_date TIMESTAMP,
    deadline TIMESTAMP,
    contact_person VARCHAR(100),
    contact_phone VARCHAR(50),
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, project_no)
);
```

#### 2.3 招投标明细表 (bidding_items)
```sql
CREATE TABLE bidding_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    project_id UUID NOT NULL REFERENCES bidding_projects(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    specification VARCHAR(200),
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

#### 2.4 投标记录表 (bidding_bids)
```sql
CREATE TABLE bidding_bids (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    project_id UUID NOT NULL REFERENCES bidding_projects(id),
    supplier_id UUID NOT NULL REFERENCES suppliers(id),
    total_amount DECIMAL(15,2),
    bid_date TIMESTAMP DEFAULT NOW(),
    status VARCHAR(20) DEFAULT 'submitted', -- submitted, awarded, rejected
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, project_id, supplier_id)
);
```

#### 2.5 采购合同表 (purchase_contracts)
```sql
CREATE TABLE purchase_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    contract_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL REFERENCES suppliers(id),
    title VARCHAR(200) NOT NULL,
    contract_type VARCHAR(20) DEFAULT 'framework', -- framework, one_time
    total_amount DECIMAL(15,2),
    currency VARCHAR(10) DEFAULT 'CNY',
    sign_date DATE,
    valid_from DATE,
    valid_until DATE,
    status VARCHAR(20) DEFAULT 'active',
    terms TEXT,
    attachment_url TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, contract_no)
);
```

### 3. 仓储管理模块

#### 3.1 入库单表 (inbound_orders)
```sql
CREATE TABLE inbound_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    order_type VARCHAR(20) NOT NULL, -- purchase, return, transfer
    source_order_id UUID, -- 关联采购订单/退货单
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    supplier_id UUID REFERENCES suppliers(id),
    status VARCHAR(20) DEFAULT 'draft', -- draft, pending, completed, cancelled
    total_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);
```

#### 3.2 入库明细表 (inbound_order_items)
```sql
CREATE TABLE inbound_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    inbound_order_id UUID NOT NULL REFERENCES inbound_orders(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    location_id UUID REFERENCES warehouse_locations(id),
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2),
    production_date DATE,
    expiry_date DATE,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

#### 3.3 出库单表 (outbound_orders)
```sql
CREATE TABLE outbound_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    order_type VARCHAR(20) NOT NULL, -- sales, production, transfer
    source_order_id UUID,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    status VARCHAR(20) DEFAULT 'draft',
    total_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);
```

#### 3.4 出库明细表 (outbound_order_items)
```sql
CREATE TABLE outbound_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    outbound_order_id UUID NOT NULL REFERENCES outbound_orders(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    location_id UUID REFERENCES warehouse_locations(id),
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    unit_price DECIMAL(15,2),
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

#### 3.5 库存移动记录表 (inventory_movements)
```sql
CREATE TABLE inventory_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    movement_no VARCHAR(50) NOT NULL,
    movement_type VARCHAR(20) NOT NULL, -- transfer, adjust
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    from_location_id UUID REFERENCES warehouse_locations(id),
    to_location_id UUID REFERENCES warehouse_locations(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    batch_no VARCHAR(100),
    quantity DECIMAL(15,3) NOT NULL,
    reason TEXT,
    operator VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, movement_no)
);
```

#### 3.6 盘点单表 (stocktaking_orders)
```sql
CREATE TABLE stocktaking_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    stocktaking_type VARCHAR(20) DEFAULT 'full', -- full, partial, cycle
    status VARCHAR(20) DEFAULT 'draft', -- draft, in_progress, completed
    start_date DATE,
    end_date DATE,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);
```

#### 3.7 盘点明细表 (stocktaking_items)
```sql
CREATE TABLE stocktaking_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    stocktaking_order_id UUID NOT NULL REFERENCES stocktaking_orders(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    location_id UUID REFERENCES warehouse_locations(id),
    batch_no VARCHAR(100),
    book_qty DECIMAL(15,3), -- 账面数量
    actual_qty DECIMAL(15,3), -- 实盘数量
    variance_qty DECIMAL(15,3), -- 差异数量
    variance_reason TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### 4. 生产管理模块

#### 4.1 BOM表 (boms)
```sql
CREATE TABLE boms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    material_id UUID NOT NULL REFERENCES materials(id),
    version VARCHAR(20) NOT NULL,
    name VARCHAR(200) NOT NULL,
    status VARCHAR(20) DEFAULT 'active',
    effective_date DATE,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, material_id, version)
);
```

#### 4.2 BOM明细表 (bom_items)
```sql
CREATE TABLE bom_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    bom_id UUID NOT NULL REFERENCES boms(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    scrap_rate DECIMAL(5,2) DEFAULT 0,
    sequence INT DEFAULT 0,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

#### 4.3 生产订单表 (production_orders)
```sql
CREATE TABLE production_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    material_id UUID NOT NULL REFERENCES materials(id),
    bom_id UUID REFERENCES boms(id),
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    planned_start_date DATE,
    planned_end_date DATE,
    actual_start_date DATE,
    actual_end_date DATE,
    status VARCHAR(20) DEFAULT 'planned', -- planned, in_progress, completed, cancelled
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);
```

#### 4.4 工单表 (work_orders)
```sql
CREATE TABLE work_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    production_order_id UUID NOT NULL REFERENCES production_orders(id),
    work_order_no VARCHAR(50) NOT NULL,
    process_name VARCHAR(100) NOT NULL,
    sequence INT DEFAULT 0,
    planned_qty DECIMAL(15,3) NOT NULL,
    completed_qty DECIMAL(15,3) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'pending', -- pending, in_progress, completed
    worker VARCHAR(100),
    equipment VARCHAR(100),
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, work_order_no)
);
```

### 5. 订单管理模块

#### 5.1 销售订单表 (sales_orders)
```sql
CREATE TABLE sales_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    order_no VARCHAR(50) NOT NULL,
    customer_name VARCHAR(200) NOT NULL,
    customer_contact VARCHAR(100),
    customer_phone VARCHAR(50),
    customer_address TEXT,
    order_date DATE DEFAULT NOW(),
    delivery_date DATE,
    total_amount DECIMAL(15,2) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'draft', -- draft, confirmed, delivering, completed, cancelled
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, order_no)
);
```

#### 5.2 销售订单明细表 (sales_order_items)
```sql
CREATE TABLE sales_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    sales_order_id UUID NOT NULL REFERENCES sales_orders(id),
    material_id UUID NOT NULL REFERENCES materials(id),
    quantity DECIMAL(15,3) NOT NULL,
    unit VARCHAR(20),
    unit_price DECIMAL(15,2) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    delivered_qty DECIMAL(15,3) DEFAULT 0,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

#### 5.3 发货单表 (delivery_orders)
```sql
CREATE TABLE delivery_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    delivery_no VARCHAR(50) NOT NULL,
    sales_order_id UUID REFERENCES sales_orders(id),
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    carrier VARCHAR(100),
    tracking_no VARCHAR(100),
    status VARCHAR(20) DEFAULT 'pending', -- pending, shipped, delivered
    ship_date TIMESTAMP,
    delivery_date TIMESTAMP,
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, delivery_no)
);
```

### 6. 退货管理模块

#### 6.1 退货申请表 (return_requests)
```sql
CREATE TABLE return_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    return_no VARCHAR(50) NOT NULL,
    return_type VARCHAR(20) NOT NULL, -- customer_return, supplier_return
    source_order_id UUID,
    reason VARCHAR(200),
    status VARCHAR(20) DEFAULT 'submitted', -- submitted, approved, processing, completed
    remark TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, return_no)
);
```

#### 6.2 维修记录表 (repair_records)
```sql
CREATE TABLE repair_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    repair_no VARCHAR(50) NOT NULL,
    material_id UUID NOT NULL REFERENCES materials(id),
    return_request_id UUID REFERENCES return_requests(id),
    defect_description TEXT,
    repair_action TEXT,
    status VARCHAR(20) DEFAULT 'pending', -- pending, in_progress, completed, scrapped
    technician VARCHAR(100),
    start_date DATE,
    end_date DATE,
    cost DECIMAL(15,2) DEFAULT 0,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, repair_no)
);
```

### 7. 财务结算模块

#### 7.1 应付账款表 (accounts_payable)
```sql
CREATE TABLE accounts_payable (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    payable_no VARCHAR(50) NOT NULL,
    supplier_id UUID NOT NULL REFERENCES suppliers(id),
    source_type VARCHAR(20) NOT NULL, -- purchase_order, contract
    source_order_id UUID,
    total_amount DECIMAL(15,2) NOT NULL,
    paid_amount DECIMAL(15,2) DEFAULT 0,
    currency VARCHAR(10) DEFAULT 'CNY',
    due_date DATE,
    status VARCHAR(20) DEFAULT 'unpaid', -- unpaid, partial, paid
    payment_date DATE,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, payable_no)
);
```

#### 7.2 成本核算表 (cost_records)
```sql
CREATE TABLE cost_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    material_id UUID NOT NULL REFERENCES materials(id),
    cost_type VARCHAR(20) NOT NULL, -- purchase, production, logistics
    period VARCHAR(20) NOT NULL, -- 2024-01
    total_cost DECIMAL(15,2) NOT NULL,
    quantity DECIMAL(15,3) NOT NULL,
    unit_cost DECIMAL(15,2) NOT NULL,
    remark TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(tenant_id, org_id, material_id, cost_type, period)
);
```

## 索引设计

所有表都应添加以下索引：
1. tenant_id, org_id 复合索引（租户隔离）
2. 业务单号唯一索引
3. 外键索引
4. 状态索引
5. 创建时间索引

## 实施顺序

**第一阶段（核心基础）：**
1. 物料管理
2. 仓库管理
3. 库存查询
4. 完善供应商管理

**第二阶段（采购与仓储）：**
1. 供应商报价
2. 招投标管理
3. 采购合同
4. 入库管理
5. 出库管理
6. 库存盘点

**第三阶段（生产与订单）：**
1. BOM管理
2. 生产计划
3. 销售订单
4. 发货管理

**第四阶段（高级功能）：**
1. 退货管理
2. 财务结算
3. 数据分析
