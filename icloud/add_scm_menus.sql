-- 添加供应链管理菜单到数据库
-- 使用gen_random_uuid()生成UUID

-- 1. 添加一级菜单: 供应链管理
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
)
ON CONFLICT DO NOTHING;

-- 2. 添加二级菜单: 供应商管理 (父菜单为供应链管理)
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '供应商管理',
    '/scm/supplier',
    '@/pages/scm/supplier/List',
    'TeamOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.supplier',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 3. 添加二级菜单: 采购订单 (父菜单为供应链管理)
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/scm' LIMIT 1),
    '采购订单',
    '/scm/purchase-order',
    '@/pages/scm/purchase-order/List',
    'FileTextOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.scm.purchaseOrder',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 4. 为所有admin角色添加这些菜单的权限
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path IN ('/scm', '/scm/supplier', '/scm/purchase-order')
ON CONFLICT (role_id, menu_id) DO NOTHING;
