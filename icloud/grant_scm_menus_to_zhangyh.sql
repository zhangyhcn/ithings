-- 给用户 zhangyh 的所有角色添加 SCM 菜单权限
-- 执行此脚本前，请确保已经执行了 add_scm_menus.sql

-- 1. 查看用户 zhangyh 的角色（可选，用于调试）
-- SELECT u.username, r.name as role_name, r.slug 
-- FROM users u
-- JOIN user_roles ur ON u.id = ur.user_id
-- JOIN roles r ON ur.role_id = r.id
-- WHERE u.username = 'zhangyh';

-- 2. 给用户 zhangyh 的所有角色添加 SCM 菜单权限
-- 这将为 zhangyh 拥有的每个角色添加 3 个 SCM 菜单的权限
INSERT INTO role_menus (role_id, menu_id)
SELECT DISTINCT
    ur.role_id,
    m.id
FROM user_roles ur
CROSS JOIN menus m
WHERE ur.user_id = (SELECT id FROM users WHERE username = 'zhangyh' LIMIT 1)
  AND m.path IN ('/scm', '/scm/supplier', '/scm/purchase-order')
  AND NOT EXISTS (
      SELECT 1 FROM role_menus rm 
      WHERE rm.role_id = ur.role_id AND rm.menu_id = m.id
  )
ON CONFLICT (role_id, menu_id) DO NOTHING;

-- 3. 验证结果（可选）
-- SELECT u.username, r.name as role_name, m.name as menu_name, m.path
-- FROM users u
-- JOIN user_roles ur ON u.id = ur.user_id
-- JOIN roles r ON ur.role_id = r.id
-- JOIN role_menus rm ON r.id = rm.role_id
-- JOIN menus m ON rm.menu_id = m.id
-- WHERE u.username = 'zhangyh' AND m.path LIKE '/scm%'
-- ORDER BY m.sort_order;
