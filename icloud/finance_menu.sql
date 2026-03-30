-- 财务管理菜单结构
-- 执行前请备份数据

-- 1. 创建一级菜单: 财务管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    NULL,
    '财务管理',
    '/finance',
    'Layout',
    'AccountBookOutlined',
    7,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance',
    NOW(),
    NOW()
);

-- ========================================
-- 2. 二级菜单: 财务会计
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance' LIMIT 1),
    '财务会计',
    '/finance/accounting',
    'Layout',
    'AuditOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting',
    NOW(),
    NOW()
);

-- 2.1 总账管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '总账管理',
    '/finance/accounting/general-ledger',
    '@/pages/finance/accounting/general-ledger/List',
    'BookOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.generalLedger',
    NOW(),
    NOW()
);

-- 2.2 凭证管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '凭证管理',
    '/finance/accounting/voucher',
    '@/pages/finance/accounting/voucher/List',
    'FileTextOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.voucher',
    NOW(),
    NOW()
);

-- 2.3 应收账款
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '应收账款',
    '/finance/accounting/receivable',
    '@/pages/finance/accounting/receivable/List',
    'DollarOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.receivable',
    NOW(),
    NOW()
);

-- 2.4 应付账款
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '应付账款',
    '/finance/accounting/payable',
    '@/pages/finance/accounting/payable/List',
    'PayCircleOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.payable',
    NOW(),
    NOW()
);

-- 2.5 固定资产
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '固定资产',
    '/finance/accounting/asset',
    '@/pages/finance/accounting/asset/List',
    'DesktopOutlined',
    5,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.asset',
    NOW(),
    NOW()
);

-- ========================================
-- 3. 二级菜单: 管理会计
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance' LIMIT 1),
    '管理会计',
    '/finance/management',
    'Layout',
    'LineChartOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.management',
    NOW(),
    NOW()
);

-- 3.1 成本管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/management' LIMIT 1),
    '成本管理',
    '/finance/management/cost',
    '@/pages/finance/management/cost/List',
    'CalculatorOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.management.cost',
    NOW(),
    NOW()
);

-- 3.2 预算管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/management' LIMIT 1),
    '预算管理',
    '/finance/management/budget',
    '@/pages/finance/management/budget/List',
    'FundOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.management.budget',
    NOW(),
    NOW()
);

-- 3.3 资金管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/management' LIMIT 1),
    '资金管理',
    '/finance/management/fund',
    '@/pages/finance/management/fund/List',
    'BankOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.management.fund',
    NOW(),
    NOW()
);

-- ========================================
-- 4. 二级菜单: 运营支持
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance' LIMIT 1),
    '运营支持',
    '/finance/operation',
    'Layout',
    'ToolOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.operation',
    NOW(),
    NOW()
);

-- 4.1 现金银行
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/operation' LIMIT 1),
    '现金银行',
    '/finance/operation/cash',
    '@/pages/finance/operation/cash/List',
    'MoneyCollectOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.operation.cash',
    NOW(),
    NOW()
);

-- 4.2 费用管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/operation' LIMIT 1),
    '费用管理',
    '/finance/operation/expense',
    '@/pages/finance/operation/expense/List',
    'ShoppingCartOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.operation.expense',
    NOW(),
    NOW()
);

-- 4.3 票据管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/operation' LIMIT 1),
    '票据管理',
    '/finance/operation/invoice',
    '@/pages/finance/operation/invoice/List',
    'FilePdfOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.operation.invoice',
    NOW(),
    NOW()
);

-- ========================================
-- 5. 二级菜单: 财务报告与合规
-- ========================================
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance' LIMIT 1),
    '财务报告与合规',
    '/finance/report',
    'Layout',
    'BarChartOutlined',
    4,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.report',
    NOW(),
    NOW()
);

-- 5.1 财务报表
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/report' LIMIT 1),
    '财务报表',
    '/finance/report/statement',
    '@/pages/finance/report/statement/List',
    'AreaChartOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.report.statement',
    NOW(),
    NOW()
);

-- 5.2 税务管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/report' LIMIT 1),
    '税务管理',
    '/finance/report/tax',
    '@/pages/finance/report/tax/List',
    'PercentageOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.report.tax',
    NOW(),
    NOW()
);

-- 5.3 合并报表
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/report' LIMIT 1),
    '合并报表',
    '/finance/report/consolidation',
    '@/pages/finance/report/consolidation/List',
    'PieChartOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.report.consolidation',
    NOW(),
    NOW()
);

-- ========================================
-- 6. 为admin角色授权
-- ========================================
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path LIKE '/finance%'
ON CONFLICT (role_id, menu_id) DO NOTHING;
