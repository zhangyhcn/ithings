-- 更新财务管理菜单结构
-- 将会计科目管理从总账管理中分离出来

-- 1. 更新总账管理的名称和路径
UPDATE menus 
SET 
    name = '总账查询',
    path = '/finance/accounting/general-ledger',
    component = '@/pages/finance/accounting/general-ledger/List',
    updated_at = NOW()
WHERE path = '/finance/accounting/general-ledger';

-- 2. 添加会计科目管理菜单（在总账查询之后）
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/finance/accounting' LIMIT 1),
    '会计科目管理',
    '/finance/accounting/chart-of-accounts',
    '@/pages/finance/accounting/chart-of-accounts/List',
    'BookOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.finance.accounting.chartOfAccounts',
    NOW(),
    NOW()
);

-- 3. 更新其他菜单的排序（保持顺序）
UPDATE menus SET sort_order = 3 WHERE path = '/finance/accounting/voucher';
UPDATE menus SET sort_order = 4 WHERE path = '/finance/accounting/receivable';
UPDATE menus SET sort_order = 5 WHERE path = '/finance/accounting/payable';
UPDATE menus SET sort_order = 6 WHERE path = '/finance/accounting/asset';

-- 4. 为admin角色授权新的会计科目管理菜单
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path = '/finance/accounting/chart-of-accounts'
ON CONFLICT (role_id, menu_id) DO NOTHING;

-- 5. 查询更新后的菜单结构
SELECT 
    m1.name as "一级菜单",
    m2.name as "二级菜单", 
    m3.name as "三级菜单",
    m3.path,
    m3.sort_order
FROM menus m1
JOIN menus m2 ON m2.parent_id = m1.id
JOIN menus m3 ON m3.parent_id = m2.id
WHERE m1.path = '/finance'
ORDER BY m2.sort_order, m3.sort_order;
