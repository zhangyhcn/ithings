-- SCM 菜单结构重新设计
-- 执行前请备份数据

-- 1. 删除旧的SCM菜单权限
DELETE FROM role_menus WHERE menu_id IN (SELECT id FROM menus WHERE path LIKE '/scm%');

-- 2. 删除旧的SCM菜单（从子菜单开始删除）
DELETE FROM menus WHERE path LIKE '/scm%';

-- 3. 创建一级菜单: 供应链管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    NULL,
    '供应链管理',
    '/scm',
    'Layout',
    'ShoppingCartOutlined',
    6,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm',
    NOW(),
    NOW()
);

-- ========================================
-- 4. 二级菜单: 计划与预测
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '计划与预测',
    '/scm/plan',
    'Layout',
    'LineChartOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.plan',
    NOW(),
    NOW()
);

-- 4.1 需求计划
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/plan' LIMIT 1),
    '需求计划',
    '/scm/plan/demand',
    '@/pages/scm/plan/demand/List',
    'AimOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.plan.demand',
    NOW(),
    NOW()
);

-- 4.2 供应计划
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/plan' LIMIT 1),
    '供应计划',
    '/scm/plan/supply',
    '@/pages/scm/plan/supply/List',
    'BranchesOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.plan.supply',
    NOW(),
    NOW()
);

-- 4.3 MRP运算
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/plan' LIMIT 1),
    'MRP运算',
    '/scm/plan/mrp',
    '@/pages/scm/plan/mrp/List',
    'CalculatorOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.plan.mrp',
    NOW(),
    NOW()
);

-- ========================================
-- 5. 二级菜单: 采购管理
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '采购管理',
    '/scm/procurement',
    'Layout',
    'ShoppingOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement',
    NOW(),
    NOW()
);

-- 5.1 供应商管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/procurement' LIMIT 1),
    '供应商管理',
    '/scm/procurement/supplier',
    '@/pages/scm/procurement/supplier/List',
    'TeamOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement.supplier',
    NOW(),
    NOW()
);

-- 5.2 供应商报价
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/procurement' LIMIT 1),
    '供应商报价',
    '/scm/procurement/quotation',
    '@/pages/scm/procurement/quotation/List',
    'DollarOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement.quotation',
    NOW(),
    NOW()
);

-- 5.3 招投标管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/procurement' LIMIT 1),
    '招投标管理',
    '/scm/procurement/bidding',
    '@/pages/scm/procurement/bidding/List',
    'FileSearchOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement.bidding',
    NOW(),
    NOW()
);

-- 5.4 采购合同
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/procurement' LIMIT 1),
    '采购合同',
    '/scm/procurement/contract',
    '@/pages/scm/procurement/contract/List',
    'SolutionOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement.contract',
    NOW(),
    NOW()
);

-- 5.5 采购订单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/procurement' LIMIT 1),
    '采购订单',
    '/scm/procurement/order',
    '@/pages/scm/procurement/order/List',
    'FileTextOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.procurement.order',
    NOW(),
    NOW()
);

-- ========================================
-- 6. 二级菜单: 生产制造
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '生产制造',
    '/scm/production',
    'Layout',
    'ToolOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production',
    NOW(),
    NOW()
);

-- 6.1 BOM管理
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

-- 6.2 生产订单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/production' LIMIT 1),
    '生产订单',
    '/scm/production/order',
    '@/pages/scm/production/order/List',
    'FileDoneOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production.order',
    NOW(),
    NOW()
);

-- 6.3 工艺路线
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/production' LIMIT 1),
    '工艺路线',
    '/scm/production/route',
    '@/pages/scm/production/route/List',
    'BranchesOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.production.route',
    NOW(),
    NOW()
);

-- ========================================
-- 7. 二级菜单: 仓储物流
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '仓储物流',
    '/scm/warehouse',
    'Layout',
    'ContainerOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse',
    NOW(),
    NOW()
);

-- 7.1 入库管理
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

-- 7.2 出库管理
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

-- 7.3 库存盘点
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

-- 7.4 库存查询
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/warehouse' LIMIT 1),
    '库存查询',
    '/scm/warehouse/inventory',
    '@/pages/scm/warehouse/inventory/List',
    'SearchOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse.inventory',
    NOW(),
    NOW()
);

-- 7.5 调拨管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/warehouse' LIMIT 1),
    '调拨管理',
    '/scm/warehouse/transfer',
    '@/pages/scm/warehouse/transfer/List',
    'SwapOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.warehouse.transfer',
    NOW(),
    NOW()
);

-- ========================================
-- 8. 二级菜单: 订单交付
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '订单交付',
    '/scm/delivery',
    'Layout',
    'RocketOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.delivery',
    NOW(),
    NOW()
);

-- 8.1 销售订单
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/delivery' LIMIT 1),
    '销售订单',
    '/scm/delivery/sales',
    '@/pages/scm/delivery/sales/List',
    'FileDoneOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.delivery.sales',
    NOW(),
    NOW()
);

-- 8.2 发货管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/delivery' LIMIT 1),
    '发货管理',
    '/scm/delivery/shipment',
    '@/pages/scm/delivery/shipment/List',
    'SendOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.delivery.shipment',
    NOW(),
    NOW()
);

-- 8.3 物流跟踪
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/delivery' LIMIT 1),
    '物流跟踪',
    '/scm/delivery/logistics',
    '@/pages/scm/delivery/logistics/List',
    'CarOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.delivery.logistics',
    NOW(),
    NOW()
);

-- ========================================
-- 9. 二级菜单: 退货管理
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '退货管理',
    '/scm/return',
    'Layout',
    'RollbackOutlined',
    6,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.return',
    NOW(),
    NOW()
);

-- 9.1 采购退货
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/return' LIMIT 1),
    '采购退货',
    '/scm/return/purchase',
    '@/pages/scm/return/purchase/List',
    'ImportOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.return.purchase',
    NOW(),
    NOW()
);

-- 9.2 销售退货
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/return' LIMIT 1),
    '销售退货',
    '/scm/return/sales',
    '@/pages/scm/return/sales/List',
    'ExportOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.return.sales',
    NOW(),
    NOW()
);

-- 9.3 退货审核
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/return' LIMIT 1),
    '退货审核',
    '/scm/return/audit',
    '@/pages/scm/return/audit/List',
    'CheckCircleOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.return.audit',
    NOW(),
    NOW()
);

-- ========================================
-- 10. 二级菜单: 协同管理
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '协同管理',
    '/scm/collab',
    'Layout',
    'TeamOutlined',
    7,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.collab',
    NOW(),
    NOW()
);

-- 10.1 协同任务
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/collab' LIMIT 1),
    '协同任务',
    '/scm/collab/task',
    '@/pages/scm/collab/task/List',
    'ScheduleOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.collab.task',
    NOW(),
    NOW()
);

-- 10.2 消息通知
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/collab' LIMIT 1),
    '消息通知',
    '/scm/collab/message',
    '@/pages/scm/collab/message/List',
    'BellOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.collab.message',
    NOW(),
    NOW()
);

-- 10.3 文档管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/collab' LIMIT 1),
    '文档管理',
    '/scm/collab/document',
    '@/pages/scm/collab/document/List',
    'FolderOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.collab.document',
    NOW(),
    NOW()
);

-- ========================================
-- 11. 二级菜单: 财务结算
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '财务结算',
    '/scm/finance',
    'Layout',
    'AccountBookOutlined',
    8,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance',
    NOW(),
    NOW()
);

-- 11.1 应付账款
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

-- 11.2 成本核算
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

-- 11.3 发票管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm/finance' LIMIT 1),
    '发票管理',
    '/scm/finance/invoice',
    '@/pages/scm/finance/invoice/List',
    'FilePdfOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.finance.invoice',
    NOW(),
    NOW()
);

-- ========================================
-- 12. 二级菜单: 基础数据 (公共管理)
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '基础数据',
    '/scm/basic',
    'Layout',
    'DatabaseOutlined',
    9,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.basic',
    NOW(),
    NOW()
);

-- 12.1 物料管理
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

-- 12.2 仓库管理
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

-- ========================================
-- 13. 为所有admin角色添加菜单权限
-- ========================================
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path LIKE '/scm%'
ON CONFLICT (role_id, menu_id) DO NOTHING;
