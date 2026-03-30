# SCM 系统实施指南

## 当前状态

### 已完成模块
1. ✅ 供应商管理
2. ✅ 采购订单（PurchaseOrder）

### 已创建数据库表结构
- ✅ material_categories (物料分类)
- ✅ materials (物料主数据)
- ✅ warehouses (仓库)
- ✅ warehouse_locations (库位)
- ✅ inventory (库存)

## 菜单结构设计

```sql
-- 一级菜单：供应链管理（已存在）
-- 二级菜单结构：

-- 基础数据
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '基础数据',
    '/scm/basic',
    'Layout',
    'DatabaseOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.basic',
    NOW(),
    NOW()
);

-- 物料管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/basic' LIMIT 1),
    '物料管理',
    '/scm/basic/material',
    '@/pages/scm/basic/material/List',
    'AppstoreOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.basic.material',
    NOW(),
    NOW()
);

-- 仓库管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/basic' LIMIT 1),
    '仓库管理',
    '/scm/basic/warehouse',
    '@/pages/scm/basic/warehouse/List',
    'HomeOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.basic.warehouse',
    NOW(),
    NOW()
);

-- 库存查询
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/basic' LIMIT 1),
    '库存查询',
    '/scm/basic/inventory',
    '@/pages/scm/basic/inventory/List',
    'SearchOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.basic.inventory',
    NOW(),
    NOW()
);

-- 采购管理（已存在供应商和采购订单）
-- 添加采购报价、招投标、采购合同

-- 仓储管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '仓储管理',
    '/scm/warehouse',
    'Layout',
    'ContainerOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse',
    NOW(),
    NOW()
);

-- 入库管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/warehouse' LIMIT 1),
    '入库管理',
    '/scm/warehouse/inbound',
    '@/pages/scm/warehouse/inbound/List',
    'LoginOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse.inbound',
    NOW(),
    NOW()
);

-- 出库管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/warehouse' LIMIT 1),
    '出库管理',
    '/scm/warehouse/outbound',
    '@/pages/scm/warehouse/outbound/List',
    'LogoutOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse.outbound',
    NOW(),
    NOW()
);

-- 库存盘点
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/warehouse' LIMIT 1),
    '库存盘点',
    '/scm/warehouse/stocktaking',
    '@/pages/scm/warehouse/stocktaking/List',
    'AuditOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse.stocktaking',
    NOW(),
    NOW()
);

-- 生产管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '生产管理',
    '/scm/production',
    'Layout',
    'ToolOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production',
    NOW(),
    NOW()
);

-- BOM管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/production' LIMIT 1),
    'BOM管理',
    '/scm/production/bom',
    '@/pages/scm/production/bom/List',
    'ApartmentOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production.bom',
    NOW(),
    NOW()
);

-- 生产订单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/production' LIMIT 1),
    '生产订单',
    '/scm/production/order',
    '@/pages/scm/production/order/List',
    'FileTextOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production.order',
    NOW(),
    NOW()
);

-- 订单管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '订单管理',
    '/scm/order',
    'Layout',
    'ShoppingCartOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.order',
    NOW(),
    NOW()
);

-- 销售订单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/order' LIMIT 1),
    '销售订单',
    '/scm/order/sales',
    '@/pages/scm/order/sales/List',
    'FileDoneOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.order.sales',
    NOW(),
    NOW()
);

-- 发货管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/order' LIMIT 1),
    '发货管理',
    '/scm/order/delivery',
    '@/pages/scm/order/delivery/List',
    'SendOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.order.delivery',
    NOW(),
    NOW()
);

-- 财务结算
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '财务结算',
    '/scm/finance',
    'Layout',
    'AccountBookOutlined',
    6,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance',
    NOW(),
    NOW()
);

-- 应付账款
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/finance' LIMIT 1),
    '应付账款',
    '/scm/finance/payable',
    '@/pages/scm/finance/payable/List',
    'MoneyCollectOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance.payable',
    NOW(),
    NOW()
);

-- 成本核算
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/finance' LIMIT 1),
    '成本核算',
    '/scm/finance/cost',
    '@/pages/scm/finance/cost/List',
    'CalculatorOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance.cost',
    NOW(),
    NOW()
);
```

## 实施步骤

### 第一步：添加菜单到数据库
执行 `scm/add_scm_extended_menus.sql`

### 第二步：实现后端服务
参考已实现的 `supplier.rs` 和 `purchase_order.rs`，为每个模块创建：
1. Entity (已完成)
2. Service (CRUD 操作)
3. API (路由和handler)

### 第三步：前端路由配置
在 `.umirc.ts` 中添加所有路由

### 第四步：前端页面实现
为每个功能模块创建 List.tsx 页面

## 优先级建议

**P0 - 立即实施（核心基础）：**
1. 物料管理
2. 仓库管理
3. 库存查询

**P1 - 近期实施（采购与仓储）：**
1. 入库管理
2. 出库管理
3. 库存盘点

**P2 - 中期实施（生产与订单）：**
1. BOM管理
2. 生产订单
3. 销售订单
4. 发货管理

**P3 - 远期实施（高级功能）：**
1. 招投标管理
2. 采购合同
3. 应付账款
4. 成本核算

## 技术栈

- 后端：Rust + Axum 0.7 + Sea-ORM 0.12
- 数据库：PostgreSQL
- 前端：React + Ant Design Pro + UmiJS

## 参考实现

详见已实现的模块：
- `scm/src/entity/supplier.rs`
- `scm/src/service/supplier.rs`
- `scm/src/api/supplier.rs`
- `web/src/pages/scm/supplier/List.tsx`
