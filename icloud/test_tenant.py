import requests
import json

# 后端服务地址
BASE_URL = "http://localhost:8002/api/v1"

def test_create_tenant():
    """测试创建租户"""
    url = f"{BASE_URL}/tenants"
    import time
    timestamp = int(time.time())
    payload = {
        "name": f"测试租户{timestamp}",
        "slug": f"test-tenant-{timestamp}",
        "description": "这是一个测试租户",
        "admin_username": f"testadmin{timestamp}",
        "admin_email": f"admin{timestamp}@test.com",
        "admin_password": "123456"
    }
    
    headers = {
        "Content-Type": "application/json"
    }
    
    try:
        response = requests.post(url, json=payload, headers=headers)
        print(f"创建租户状态码: {response.status_code}")
        print(f"响应内容: {json.dumps(response.json(), indent=2, ensure_ascii=False)}")
        
        if response.status_code == 200:
            data = response.json()
            return data.get("data", {})
        else:
            print("创建租户失败")
            return None
    except Exception as e:
        print(f"创建租户请求失败: {e}")
        return None

def test_login(username, password):
    """测试用户登录"""
    url = f"{BASE_URL}/user/login"
    payload = {
        "username": username,
        "password": password
    }
    
    headers = {
        "Content-Type": "application/json"
    }
    
    try:
        response = requests.post(url, json=payload, headers=headers)
        print(f"\n登录状态码: {response.status_code}")
        print(f"响应内容: {json.dumps(response.json(), indent=2, ensure_ascii=False)}")
        
        if response.status_code == 200:
            data = response.json()
            return data.get("data", {}).get("access_token")
        else:
            print("登录失败")
            return None
    except Exception as e:
        print(f"登录请求失败: {e}")
        return None

def test_get_user_menus(token):
    """测试获取用户菜单权限"""
    url = f"{BASE_URL}/menu/user"
    
    headers = {
        "Authorization": f"Bearer {token}"
    }
    
    try:
        response = requests.get(url, headers=headers)
        print(f"\n获取用户菜单状态码: {response.status_code}")
        print(f"响应内容: {json.dumps(response.json(), indent=2, ensure_ascii=False)}")
        
        if response.status_code == 200:
            menus = response.json().get("data", [])
            print(f"\n用户拥有的菜单数量: {len(menus)}")
            for menu in menus:
                print(f"- {menu.get('name')} ({menu.get('path')})")
            return menus
        else:
            print("获取用户菜单失败")
            return None
    except Exception as e:
        print(f"获取用户菜单请求失败: {e}")
        return None

if __name__ == "__main__":
    print("=== 开始测试租户创建和权限功能 ===")
    
    # 1. 测试创建租户
    tenant_data = test_create_tenant()
    if not tenant_data:
        print("创建租户失败，终止测试")
        exit(1)
    
    admin_user = tenant_data.get("admin_user", {})
    username = admin_user.get("username")
    if not username:
        print("获取管理员用户名失败")
        exit(1)
    
    # 2. 测试管理员登录
    token = test_login(username, "123456")
    if not token:
        print("登录失败，终止测试")
        exit(1)
    
    # 3. 测试获取用户菜单权限
    menus = test_get_user_menus(token)
    if menus:
        print("\n=== 测试成功 ===")
        print("租户创建成功，管理员用户可以正常登录并拥有菜单权限")
    else:
        print("\n=== 测试失败 ===")
        print("用户没有获取到菜单权限")
