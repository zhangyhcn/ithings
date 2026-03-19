#!/usr/bin/env python3
import psycopg2

DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'dbname': 'icloud',
    'user': 'postgres',
    'password': '123123',
}

TENANT_ID = 'ceca8d9b-82d9-4a9d-a316-0327456919dc'

def main():
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()

    # 查找这个租户的admin角色
    cursor.execute("""
        SELECT id, name, slug FROM roles WHERE tenant_id = %s AND slug = 'admin';
    """, (TENANT_ID,))
    admin_role = cursor.fetchone()
    if not admin_role:
        print(f"ERROR: No admin role found for tenant {TENANT_ID}")
        return

    admin_role_id = admin_role[0]
    print(f"Found admin role: {admin_role_id}")

    # 查找admin角色拥有的所有菜单权限
    cursor.execute("""
        SELECT m.id, m.parent_id, m.name, m.path 
        FROM menus m
        JOIN role_menus rm ON m.id = rm.menu_id
        WHERE rm.role_id = %s
        ORDER BY m.sort_order;
    """, (admin_role_id,))

    permissions = cursor.fetchall()
    print(f"\nAdmin role has {len(permissions)} menu permissions:")
    for p in permissions:
        parent_id = p[1]
        indent = "  " if parent_id is not None else ""
        print(f"{indent}• {p[2]} ({p[3]})")

    # 检查哪些菜单没有权限
    cursor.execute("SELECT id, parent_id, name, path FROM menus ORDER BY sort_order;")
    all_menus = cursor.fetchall()

    permission_ids = {p[0] for p in permissions}
    missing = [m for m in all_menus if m[0] not in permission_ids]

    if missing:
        print(f"\nMissing {len(missing)} permissions:")
        for m in missing:
            print(f"  • {m[2]} ({m[3]}) - id={m[0]}")
        
        # 添加缺失的权限
        confirm = input("\nAdd missing permissions? (y/n): ")
        if confirm.lower() == 'y':
            added = 0
            for m in missing:
                import uuid
                rm_id = uuid.uuid4()
                cursor.execute("""
                    INSERT INTO role_menus (id, role_id, menu_id) VALUES (%s, %s, %s);
                """, (rm_id, admin_role_id, m[0]))
                added += 1
            conn.commit()
            print(f"\nAdded {added} missing permissions!")
    else:
        print("\nAll menus have permissions!")

    conn.close()

if __name__ == '__main__':
    main()
