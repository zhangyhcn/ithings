-- 为 zhangyh 用户重新授权最新的供应链菜单（通过角色）
-- 执行前请确保已执行 scm_menu_redesign.sql

-- 方法1: 如果使用 user_menus 表
DELETE FROM user_menus 
WHERE user_id = (SELECT id FROM users WHERE username = 'zhangyh')
  AND menu_id IN (SELECT id FROM menus WHERE path LIKE '/scm%');

INSERT INTO user_menus (user_id, menu_id, created_at)
SELECT
    u.id,
    m.id,
    NOW()
FROM users u
CROSS JOIN menus m
WHERE u.username = 'zhangyh'
  AND m.path LIKE '/scm%'
ON CONFLICT (user_id, menu_id) DO NOTHING;

-- 方法2: 如果使用 role_menus 表（确保 admin 角色有权限）
INSERT INTO role_menus (role_id, menu_id)
SELECT
    r.id,
    m.id
FROM roles r
CROSS JOIN menus m
WHERE r.slug = 'admin'
  AND m.path LIKE '/scm%'
ON CONFLICT (role_id, menu_id) DO NOTHING;

-- 验证授权结果
SELECT 
    m.name AS menu_name,
    m.path AS menu_path,
    m.icon,
    m.sort_order
FROM menus m
WHERE m.path LIKE '/scm%'
ORDER BY m.sort_order, m.path;

-- 如果 zhangyh 用户有 admin 角色，查看其权限
SELECT 
    u.username,
    r.name AS role_name,
    COUNT(m.id) AS menu_count
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
LEFT JOIN role_menus rm ON r.id = rm.role_id
LEFT JOIN menus m ON rm.menu_id = m.id AND m.path LIKE '/scm%'
WHERE u.username = 'zhangyh'
GROUP BY u.username, r.name;
