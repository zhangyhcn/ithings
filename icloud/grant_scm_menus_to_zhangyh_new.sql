-- 为 zhangyh 用户重新授权最新的供应链菜单
-- 执行前请确保已执行 scm_menu_redesign.sql

-- 1. 删除 zhangyh 用户的所有SCM菜单权限
DELETE FROM user_menus 
WHERE user_id = (SELECT id FROM users WHERE username = 'zhangyh')
  AND menu_id IN (SELECT id FROM menus WHERE path LIKE '/scm%');

-- 2. 为 zhangyh 用户添加所有新的SCM菜单权限
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

-- 3. 验证授权结果
SELECT 
    m.name AS menu_name,
    m.path AS menu_path,
    m.icon,
    CASE 
        WHEN um.user_id IS NOT NULL THEN '已授权'
        ELSE '未授权'
    END AS authorization_status
FROM menus m
LEFT JOIN user_menus um ON m.id = um.menu_id 
    AND um.user_id = (SELECT id FROM users WHERE username = 'zhangyh')
WHERE m.path LIKE '/scm%'
ORDER BY m.path;
