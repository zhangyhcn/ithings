import requests
import json

# 后端API地址
base_url = "http://localhost:9003/api/v1"

# 测试账号
login_data = {
    "username": "test_admin",
    "password": "123456"
}

print("=== 测试退出登录功能 ===")

# 1. 登录获取token
print("\n1. 正在登录获取token...")
try:
    login_response = requests.post(f"{base_url}/auth/login", json=login_data)
    login_response.raise_for_status()
    login_result = login_response.json()
    access_token = login_result.get("data", {}).get("access_token")
    
    if not access_token:
        print("❌ 登录失败，未获取到token")
        print("响应内容:", login_result)
        exit(1)
        
    print(f"✅ 登录成功，获取到access_token: {access_token[:50]}...")
    
except Exception as e:
    print(f"❌ 登录失败: {e}")
    exit(1)

# 设置请求头
headers = {
    "Authorization": f"Bearer {access_token}",
    "Content-Type": "application/json"
}

# 2. 测试访问需要认证的接口（获取菜单列表）...
print("\n2. 测试访问需要认证的接口（获取菜单列表）...")
try:
    menu_response = requests.get(f"{base_url}/menus", headers=headers)
    if menu_response.status_code == 200:
        print("✅ 认证成功，可以正常访问接口")
    else:
        print(f"❌ 访问失败，状态码: {menu_response.status_code}")
        print("响应内容:", menu_response.text)
        
except Exception as e:
    print(f"❌ 请求失败: {e}")

# 3. 测试退出登录
print("\n3. 测试退出登录...")
try:
    logout_response = requests.post(f"{base_url}/auth/logout", headers=headers)
    logout_response.raise_for_status()
    logout_result = logout_response.json()
    
    if logout_result.get("code") == 200:
        print("✅ 退出登录成功")
    else:
        print("❌ 退出登录失败")
        print("响应内容:", logout_result)
        
except Exception as e:
    print(f"❌ 退出登录请求失败: {e}")
    exit(1)

# 4. 测试退出登录后token是否失效
print("\n4. 测试退出登录后token是否失效...")
try:
    menu_response = requests.get(f"{base_url}/menus", headers=headers)
    if menu_response.status_code == 401:
        response_data = menu_response.json()
        if "Token has been revoked" in response_data.get("message", ""):
            print("✅ token已失效，退出登录功能正常工作")
        else:
            print(f"⚠️  返回401但消息不符: {response_data.get('message')}")
    else:
        print(f"❌  token仍然有效，状态码: {menu_response.status_code}")
        print("响应内容:", menu_response.text)
        
except Exception as e:
    print(f"❌ 请求失败: {e}")

print("\n=== 测试完成 ===")
