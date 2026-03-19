import requests
import json

# 后端API地址
base_url = "http://localhost:9001/api/v1"

# 创建租户的参数
tenant_data = {
    "name": "测试租户",
    "slug": "test_tenant_20260315",
    "admin_username": "test_admin",
    "admin_email": "test_admin@example.com",
    "admin_password": "123456"
}

print("=== 测试1: 创建租户接口 ===")
print("请求参数:", json.dumps(tenant_data, indent=2, ensure_ascii=False))

try:
    response = requests.post(
        f"{base_url}/tenants",
        json=tenant_data
    )
    
    print(f"\n响应状态码: {response.status_code}")
    
    try:
        response_data = response.json()
        print("响应内容:", json.dumps(response_data, indent=2, ensure_ascii=False))
    except:
        print("响应内容不是JSON格式:", response.text)
    
    if response.status_code == 200:
        print("\n✅ 创建租户成功！")
    else:
        print(f"\n❌ 创建租户失败，状态码: {response.status_code}")
        
except Exception as e:
    print(f"\n❌ 请求失败: {e}")

print("\n" + "="*50 + "\n")

print("=== 测试2: 测试新租户管理员登录 ===")
# 使用新创建的租户管理员账号登录
login_data = {
    "username": "test_admin",
    "password": "123456"
}

print("登录参数:", json.dumps(login_data, indent=2, ensure_ascii=False))

try:
    login_response = requests.post(f"{base_url}/auth/login", json=login_data)
    
    print(f"\n响应状态码: {login_response.status_code}")
    
    try:
        login_result = login_response.json()
        print("响应内容:", json.dumps(login_result, indent=2, ensure_ascii=False))
        
        token = login_result.get("data", {}).get("token")
        if token:
            print("\n✅ 新租户管理员登录成功！")
            print("获取到的token:", token[:50] + "...")
        else:
            print("\n❌ 登录成功但未获取到token")
            
    except:
        print("响应内容不是JSON格式:", login_response.text)
        print("\n❌ 登录失败")
        
except Exception as e:
    print(f"\n❌ 登录请求失败: {e}")

