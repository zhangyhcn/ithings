-- 清理国际化菜单记录
-- 首先删除role_menus中关联的记录
DELETE FROM role_menus 
WHERE menu_id IN (
    SELECT id FROM menus 
    WHERE path = '/settings/internationalization' 
       OR name = '国际化'
       OR component = '@/pages/settings/Internationalization'
);

-- 然后删除menus表中的国际化菜单
DELETE FROM menus 
WHERE path = '/settings/internationalization' 
   OR name = '国际化'
   OR component = '@/pages/settings/Internationalization';

-- 重新排序系统设置下的菜单项sort_order
-- 系统配置从原来的8改为7
UPDATE menus 
SET sort_order = 7 
WHERE path = '/settings/system' 
  AND parent_id IN (SELECT id FROM menus WHERE name = '系统设置');
