-- 添加资源管理菜单到数据库
-- 使用gen_random_uuid()生成UUID

-- 1. 添加一级菜单: 资源管理
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    NULL,
    '资源管理',
    '/resources',
    'Layout',
    'DatabaseOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.resource',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 2. 添加二级菜单: CRD定义 (父菜单为资源管理)
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/resources' LIMIT 1),
    'CRD定义',
    '/resources/crd',
    '@/pages/resource/crd/List',
    'CodeOutlined',
    1,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.resource.crd',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 3. 添加二级菜单: Operator定义 (父菜单为资源管理)
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/resources' LIMIT 1),
    'Operator定义',
    '/resources/operator',
    '@/pages/resource/operator/List',
    'ControlOutlined',
    2,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.resource.operator',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 4. 添加二级菜单: Controller定义 (父菜单为资源管理)
INSERT INTO menus (id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM menus WHERE path = '/resources' LIMIT 1),
    'Controller定义',
    '/resources/controller',
    '@/pages/resource/controller/List',
    'GatewayOutlined',
    3,
    'active',
    '["admin", "editor"]'::jsonb,
    'menu.resource.controller',
    NOW(),
    NOW()
)
ON CONFLICT DO NOTHING;

-- 5. 为所有admin角色添加这些菜单的权限
-- role_menus.id是自增的，不需要指定
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path IN ('/resources', '/resources/crd', '/resources/operator', '/resources/controller')
ON CONFLICT (role_id, menu_id) DO NOTHING;
