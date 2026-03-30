-- SCM 系统扩展菜单
-- 执行前请确保已执行 add_scm_menus.sql

-- 1. 基础数据二级菜单
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
) ON CONFLICT DO NOTHING;

-- 1.1 物料管理
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
) ON CONFLICT DO NOTHING;

-- 1.2 仓库管理
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
) ON CONFLICT DO NOTHING;

-- 1.3 库存查询
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
) ON CONFLICT DO NOTHING;

-- 2. 采购管理扩展（供应商报价、招投标、采购合同）
-- 2.1 供应商报价
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '供应商报价',
    '/scm/quotation',
    '@/pages/scm/quotation/List',
    'DollarOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.quotation',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 2.2 招投标管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '招投标管理',
    '/scm/bidding',
    '@/pages/scm/bidding/List',
    'FileSearchOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.bidding',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 2.3 采购合同
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '采购合同',
    '/scm/contract',
    '@/pages/scm/contract/List',
    'SolutionOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.contract',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 3. 仓储管理二级菜单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '仓储管理',
    '/scm/warehouse',
    'Layout',
    'ContainerOutlined',
    6,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 3.1 入库管理
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
) ON CONFLICT DO NOTHING;

-- 3.2 出库管理
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
) ON CONFLICT DO NOTHING;

-- 3.3 库存盘点
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
) ON CONFLICT DO NOTHING;

-- 4. 生产管理二级菜单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '生产管理',
    '/scm/production',
    'Layout',
    'ToolOutlined',
    7,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 4.1 BOM管理
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
) ON CONFLICT DO NOTHING;

-- 4.2 生产订单
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
) ON CONFLICT DO NOTHING;

-- 5. 订单管理二级菜单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '订单管理',
    '/scm/order',
    'Layout',
    'ShoppingCartOutlined',
    8,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.order',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 5.1 销售订单
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
) ON CONFLICT DO NOTHING;

-- 5.2 发货管理
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
) ON CONFLICT DO NOTHING;

-- 6. 财务结算二级菜单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '财务结算',
    '/scm/finance',
    'Layout',
    'AccountBookOutlined',
    9,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance',
    NOW(),
    NOW()
) ON CONFLICT DO NOTHING;

-- 6.1 应付账款
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
) ON CONFLICT DO NOTHING;

-- 6.2 成本核算
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
) ON CONFLICT DO NOTHING;

-- 7. 为所有admin角色添加这些菜单的权限
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path LIKE '/scm%'
  AND m.path NOT IN ('/scm/supplier', '/scm/purchase-order')
ON CONFLICT (role_id, menu_id) DO NOTHING;
