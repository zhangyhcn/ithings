#!/usr/bin/env python3
import psycopg2

DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'dbname': 'icloud',
    'user': 'postgres',
    'password': '123123',
}

# 需要修正的路径映射
FIX_PATHS = [
    # (current_path, correct_path)
    ('/config/configmap', '/config/config_map'),
    ('/config/secret', '/config/secret'),
    ('/resources/crd', '/resources/crd'),
    ('/resources/operator', '/resources/operator'),
    ('/resources/controller', '/resources/controller'),
    ('/device/product', '/device/product'),
    ('/device/driver', '/device/driver'),
    ('/device/node', '/device/node'),
    ('/device/instance', '/device/instance'),
]

def main():
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()

    print("Fixing menu paths...")

    fixed_count = 0
    for old_path, new_path in FIX_PATHS:
        cursor.execute("SELECT id, path FROM menus WHERE path = %s;", (old_path,))
        result = cursor.fetchone()
        if result is None:
            print(f"Menu not found: {old_path}")
            continue

        menu_id = result[0]
        if old_path == new_path:
            continue

        cursor.execute("UPDATE menus SET path = %s WHERE id = %s;", (new_path, menu_id))
        fixed_count += 1
        print(f"Fixed: {old_path} -> {new_path}")

    # 同时修正 component 路径
    FIX_COMPONENTS = [
        ('@/pages/config/config_map/List', '@/pages/config/config_map/List'),
        ('@/pages/config/secret/List', '@/pages/config/secret/List'),
        ('@/pages/resource/crd/List', '@/pages/resource/crd/List'),
        ('@/pages/resource/operator/List', '@/pages/resource/operator/List'),
        ('@/pages/resource/controller/List', '@/pages/resource/controller/List'),
        ('@/pages/device/product/List', '@/pages/device/product/List'),
        ('@/pages/device/driver/List', '@/pages/device/driver/List'),
        ('@/pages/device/node/List', '@/pages/device/node/List'),
        ('@/pages/device/instance/List', '@/pages/device/instance/List'),
    ]

    print()
    print("Checking components...")

    for old_comp, new_comp in FIX_COMPONENTS:
        cursor.execute("SELECT id, component FROM menus WHERE component = %s;", (old_comp,))
        result = cursor.fetchone()
        if result is None:
            print(f"Component not found: {old_comp}")
            continue

        # component already correct
        if old_comp == new_comp:
            continue

        menu_id = result[0]
        cursor.execute("UPDATE menus SET component = %s WHERE id = %s;", (new_comp, menu_id))
        fixed_count += 1
        print(f"Fixed component: {old_comp} -> {new_comp}")

    conn.commit()
    conn.close()

    print()
    print(f"Done! Fixed {fixed_count} items.")
    print("Please restart frontend and refresh browser.")

if __name__ == '__main__':
    main()
