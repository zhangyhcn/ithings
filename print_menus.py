#!/usr/bin/env python3
import psycopg2

DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'dbname': 'icloud',
    'user': 'postgres',
    'password': '123123',
}

def print_menu_tree(menus, parent_id=None, indent=0):
    for menu in menus:
        if menu[1] == parent_id:
            prefix = '  ' * indent
            print(f"{prefix}• {menu[2]} ({menu[3]}) -> {menu[4]}")
            print_menu_tree(menus, menu[0], indent + 1)

def main():
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()

    cursor.execute("""
        SELECT id, parent_id, name, path, component 
        FROM menus 
        ORDER BY sort_order;
    """)
    menus = cursor.fetchall()

    print(f"Total menus: {len(menus)}\n")
    print("Menu tree:")
    print_menu_tree(menus, None)

    conn.close()

if __name__ == '__main__':
    main()
