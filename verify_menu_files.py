#!/usr/bin/env python3
import os
import psycopg2

DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'dbname': 'icloud',
    'user': 'postgres',
    'password': '123123',
}

WEB_ROOT = '/root/source/rust/ithings/web/src/pages'

def main():
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()

    cursor.execute("""
        SELECT path, component FROM menus ORDER BY path;
    """)
    menus = cursor.fetchall()

    print("=" * 60)
    print("菜单 path (from DB) | 文件路径 | 是否存在")
    print("=" * 60)

    for path, component in menus:
        if component == 'Layout':
            print(f"\n{path} (Layout container)")
            continue
        
        # component: @/pages/device/product/List -> pages/device/product/List.tsx
        if component.startswith('@/pages/'):
            component_path = component[len('@/pages/'):] + '.tsx'
            full_path = os.path.join(WEB_ROOT, component_path)
            exists = os.path.exists(full_path)
            status = "✅" if exists else "❌"
            print(f"  {path:<20} | {component_path:<30} | {status}")

    conn.close()

if __name__ == '__main__':
    main()
