#!/usr/bin/env python3
import psycopg2
import uuid
from psycopg2.extensions import AsIs

# 数据库连接配置
DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'dbname': 'icloud',
    'user': 'postgres',
    'password': '123123',
}

# 要添加权限的租户ID
TARGET_TENANT_ID = 'ceca8d9b-82d9-4a9d-a316-0327456919dc'

# 默认菜单定义，和后端代码保持一致
DEFAULT_MENUS = [
    # 一级菜单
    {
        'name': '仪表盘',
        'path': '/dashboard',
        'component': '@/pages/Dashboard',
        'icon': 'DashboardOutlined',
        'parent_path': None,
        'sort_order': 1,
        'status': 'active',
        'roles': ['admin', 'editor', 'user'],
        'i18n_key': 'menu.dashboard',
    },
    {
        'name': '系统设置',
        'path': '/settings',
        'component': 'Layout',
        'icon': 'SettingOutlined',
        'parent_path': None,
        'sort_order': 2,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.setting',
    },
    {
        'name': '资源管理',
        'path': '/resources',
        'component': 'Layout',
        'icon': 'DatabaseOutlined',
        'parent_path': None,
        'sort_order': 3,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.resource',
    },
    {
        'name': '配置管理',
        'path': '/config',
        'component': 'Layout',
        'icon': 'SettingOutlined',
        'parent_path': None,
        'sort_order': 4,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.config',
    },
    {
        'name': '设备管理',
        'path': '/device',
        'component': 'Layout',
        'icon': 'MobileOutlined',
        'parent_path': None,
        'sort_order': 5,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.device',
    },
    # 系统设置二级菜单
    {
        'name': '租户管理',
        'path': '/tenant',
        'component': '@/pages/tenant/List',
        'icon': 'ApartmentOutlined',
        'parent_path': '/settings',
        'sort_order': 1,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.tenant',
    },
    {
        'name': '用户管理',
        'path': '/users',
        'component': '@/pages/user/List',
        'icon': 'UserOutlined',
        'parent_path': '/settings',
        'sort_order': 2,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.user',
    },
    {
        'name': '组织机构',
        'path': '/organization',
        'component': '@/pages/organization/List',
        'icon': 'ApartmentOutlined',
        'parent_path': '/settings',
        'sort_order': 3,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.organization',
    },
    {
        'name': '部门管理',
        'path': '/department',
        'component': '@/pages/department/List',
        'icon': 'TeamOutlined',
        'parent_path': '/settings',
        'sort_order': 4,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.department',
    },
    {
        'name': '站点管理',
        'path': '/site',
        'component': '@/pages/site/List',
        'icon': 'EnvironmentOutlined',
        'parent_path': '/settings',
        'sort_order': 5,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.site',
    },
    {
        'name': '命名空间',
        'path': '/namespace',
        'component': '@/pages/namespace/List',
        'icon': 'FolderOutlined',
        'parent_path': '/settings',
        'sort_order': 6,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.namespace',
    },
    {
        'name': '角色管理',
        'path': '/settings/role',
        'component': '@/pages/role/List',
        'icon': 'TeamOutlined',
        'parent_path': '/settings',
        'sort_order': 7,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.role',
    },
    {
        'name': '用户角色授权',
        'path': '/settings/user-role',
        'component': '@/pages/user-role/List',
        'icon': 'UserOutlined',
        'parent_path': '/settings',
        'sort_order': 8,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.userRole',
    },
    {
        'name': '角色菜单授权',
        'path': '/settings/role-menu',
        'component': '@/pages/role-menu/List',
        'icon': 'MenuOutlined',
        'parent_path': '/settings',
        'sort_order': 9,
        'status': 'active',
        'roles': ['admin'],
        'i18n_key': 'menu.roleMenu',
    },
    # 资源管理二级菜单
    {
        'name': 'CRD定义',
        'path': '/resources/crd',
        'component': '@/pages/resource/crd/List',
        'icon': 'CodeOutlined',
        'parent_path': '/resources',
        'sort_order': 1,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.resource.crd',
    },
    {
        'name': 'Operator定义',
        'path': '/resources/operator',
        'component': '@/pages/resource/operator/List',
        'icon': 'ControlOutlined',
        'parent_path': '/resources',
        'sort_order': 2,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.resource.operator',
    },
    {
        'name': 'Controller定义',
        'path': '/resources/controller',
        'component': '@/pages/resource/controller/List',
        'icon': 'GatewayOutlined',
        'parent_path': '/resources',
        'sort_order': 3,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.resource.controller',
    },
    # 配置管理二级菜单
    {
        'name': '配置项',
        'path': '/config/configmap',
        'component': '@/pages/config/config_map/List',
        'icon': 'FileTextOutlined',
        'parent_path': '/config',
        'sort_order': 1,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.config.configMap',
    },
    {
        'name': '保密字典',
        'path': '/config/secret',
        'component': '@/pages/config/secret/List',
        'icon': 'LockOutlined',
        'parent_path': '/config',
        'sort_order': 2,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.config.secret',
    },
    # 设备管理二级菜单
    {
        'name': '产品管理',
        'path': '/device/product',
        'component': '@/pages/device/product/List',
        'icon': 'BoxOutlined',
        'parent_path': '/device',
        'sort_order': 1,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.device.product',
    },
    {
        'name': '驱动管理',
        'path': '/device/driver',
        'component': '@/pages/device/driver/List',
        'icon': 'HddOutlined',
        'parent_path': '/device',
        'sort_order': 2,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.device.driver',
    },
    {
        'name': '节点管理',
        'path': '/device/node',
        'component': '@/pages/device/node/List',
        'icon': 'ClusterOutlined',
        'parent_path': '/device',
        'sort_order': 3,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.device.node',
    },
    {
        'name': '设备实例',
        'path': '/device/instance',
        'component': '@/pages/device/instance/List',
        'icon': 'DesktopOutlined',
        'parent_path': '/device',
        'sort_order': 4,
        'status': 'active',
        'roles': ['admin', 'editor'],
        'i18n_key': 'menu.device.instance',
    },
]

def main():
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()

    print("Connected to database successfully")

    cursor.execute("SELECT id, path FROM menus;")
    existing_menus = {path: id for id, path in cursor.fetchall()}
    print(f"Found {len(existing_menus)} existing menus")

    path_to_id = existing_menus.copy()
    added_count = 0

    from datetime import datetime
    now = datetime.utcnow()

    for menu_def in DEFAULT_MENUS:
        if menu_def['path'] in existing_menus:
            print(f"Menu already exists: {menu_def['path']} - {menu_def['name']}")
            continue

        parent_id = None
        if menu_def['parent_path']:
            parent_id = path_to_id.get(menu_def['parent_path'])

        menu_id = uuid.uuid4()
        roles = '["' + '","'.join(menu_def['roles']) + '"]'

        sql = """
            INSERT INTO menus (
                id, parent_id, name, path, component, icon, sort_order, status, roles, i18n_key, created_at, updated_at
            ) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s);
        """

        cursor.execute(sql, (
            menu_id,
            parent_id,
            menu_def['name'],
            menu_def['path'],
            menu_def['component'],
            menu_def['icon'],
            menu_def['sort_order'],
            menu_def['status'],
            roles,
            menu_def['i18n_key'],
            now,
            now,
        ))
        path_to_id[menu_def['path']] = menu_id
        added_count += 1
        print(f"Added menu: {menu_def['path']} - {menu_def['name']}")

    print(f"\nAdded {added_count} new menus")

    cursor.execute("""
        SELECT id FROM roles 
        WHERE tenant_id = %s AND slug = 'admin';
    """, (TARGET_TENANT_ID,))

    result = cursor.fetchone()
    if not result:
        print(f"ERROR: admin role not found for tenant {TARGET_TENANT_ID}")
        conn.commit()
        conn.close()
        return

    admin_role_id = result[0]
    print(f"\nFound admin role: {admin_role_id}")

    cursor.execute("""
        SELECT menu_id FROM role_menus WHERE role_id = %s;
    """, (admin_role_id,))

    existing_role_menus = {row[0] for row in cursor.fetchall()}
    print(f"Found {len(existing_role_menus)} existing menu permissions for admin")

    added_perm_count = 0

    for menu_id in path_to_id.values():
        if menu_id not in existing_role_menus:
            role_menu_id = uuid.uuid4()
            cursor.execute("""
                INSERT INTO role_menus (id, role_id, menu_id) VALUES (%s, %s, %s);
            """, (role_menu_id, admin_role_id, menu_id))
            added_perm_count += 1
            print(f"Added permission for menu: {menu_id}")

    print(f"\nAdded {added_perm_count} new menu permissions")
    print(f"Total: {len(existing_role_menus) + added_perm_count} menu permissions for admin")

    conn.commit()
    conn.close()
    print("\nDone!")

if __name__ == '__main__':
    main()
