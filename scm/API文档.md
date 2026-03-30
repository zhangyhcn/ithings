# SCM API 文档

## 概述

SCM（供应链管理系统）提供 RESTful API 接口，用于管理供应商、采购订单等供应链相关业务。

**基础路径**: `/api/v1/scm`

## 通用说明

### 认证
所有接口都需要通过 URL 路径参数 `tenant_id` 来标识租户。

### 响应格式

所有接口返回统一的 JSON 格式：

```json
{
  "code": 200,
  "message": "success",
  "data": { ... }
}
```

### 错误码

| 错误码 | 说明 |
|--------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

---

## 供应商管理

### 1. 获取供应商列表

**请求**
```
GET /api/v1/scm/tenants/:tenant_id/suppliers
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "supplier_code": "SUP001",
      "supplier_name": "优质材料供应商",
      "contact_person": "张经理",
      "contact_phone": "13800138000",
      "contact_email": "zhang@supplier.com",
      "address": "上海市浦东新区",
      "bank_name": "工商银行",
      "bank_account": "1234567890",
      "tax_number": "1234567890123",
      "supplier_type": "原材料",
      "credit_level": "A",
      "remarks": "长期合作伙伴",
      "status": "active",
      "created_at": "2024-01-01 10:00:00",
      "updated_at": "2024-01-01 10:00:00"
    }
  ]
}
```

### 2. 创建供应商

**请求**
```
POST /api/v1/scm/tenants/:tenant_id/suppliers
Content-Type: application/json
```

**请求体**
```json
{
  "supplier_code": "SUP001",
  "supplier_name": "优质材料供应商",
  "contact_person": "张经理",
  "contact_phone": "13800138000",
  "contact_email": "zhang@supplier.com",
  "address": "上海市浦东新区",
  "bank_name": "工商银行",
  "bank_account": "1234567890",
  "tax_number": "1234567890123",
  "supplier_type": "原材料",
  "credit_level": "A",
  "remarks": "长期合作伙伴"
}
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "supplier_code": "SUP001",
    "supplier_name": "优质材料供应商",
    "contact_person": "张经理",
    "contact_phone": "13800138000",
    "contact_email": "zhang@supplier.com",
    "address": "上海市浦东新区",
    "bank_name": "工商银行",
    "bank_account": "1234567890",
    "tax_number": "1234567890123",
    "supplier_type": "原材料",
    "credit_level": "A",
    "remarks": "长期合作伙伴",
    "status": "active",
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-01 10:00:00"
  }
}
```

### 3. 获取单个供应商

**请求**
```
GET /api/v1/scm/tenants/:tenant_id/suppliers/:id
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "supplier_code": "SUP001",
    "supplier_name": "优质材料供应商",
    "contact_person": "张经理",
    "contact_phone": "13800138000",
    "contact_email": "zhang@supplier.com",
    "address": "上海市浦东新区",
    "bank_name": "工商银行",
    "bank_account": "1234567890",
    "tax_number": "1234567890123",
    "supplier_type": "原材料",
    "credit_level": "A",
    "remarks": "长期合作伙伴",
    "status": "active",
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-01 10:00:00"
  }
}
```

### 4. 更新供应商

**请求**
```
PUT /api/v1/scm/tenants/:tenant_id/suppliers/:id
Content-Type: application/json
```

**请求体**
```json
{
  "supplier_name": "优质材料供应商（更新）",
  "contact_person": "李经理",
  "contact_phone": "13900139000",
  "status": "active"
}
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "supplier_code": "SUP001",
    "supplier_name": "优质材料供应商（更新）",
    "contact_person": "李经理",
    "contact_phone": "13900139000",
    "contact_email": "zhang@supplier.com",
    "address": "上海市浦东新区",
    "bank_name": "工商银行",
    "bank_account": "1234567890",
    "tax_number": "1234567890123",
    "supplier_type": "原材料",
    "credit_level": "A",
    "remarks": "长期合作伙伴",
    "status": "active",
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-02 14:30:00"
  }
}
```

### 5. 删除供应商

**请求**
```
DELETE /api/v1/scm/tenants/:tenant_id/suppliers/:id
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": null
}
```

---

## 采购订单管理

### 1. 获取采购订单列表

**请求**
```
GET /api/v1/scm/tenants/:tenant_id/purchase-orders
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "order_no": "PO20240101abc12345",
      "supplier_id": "550e8400-e29b-41d4-a716-446655440002",
      "order_date": "2024-01-01",
      "expected_delivery_date": "2024-01-15",
      "payment_terms": "月结30天",
      "delivery_address": "工厂仓库",
      "contact_person": "王采购",
      "contact_phone": "13800138000",
      "total_amount": "100000.0000",
      "currency": "CNY",
      "remarks": "首批采购",
      "status": "draft",
      "created_by": null,
      "approved_by": null,
      "approved_at": null,
      "created_at": "2024-01-01 10:00:00",
      "updated_at": "2024-01-01 10:00:00"
    }
  ]
}
```

### 2. 创建采购订单

**请求**
```
POST /api/v1/scm/tenants/:tenant_id/purchase-orders
Content-Type: application/json
```

**请求体**
```json
{
  "supplier_id": "550e8400-e29b-41d4-a716-446655440002",
  "order_date": "2024-01-01",
  "expected_delivery_date": "2024-01-15",
  "payment_terms": "月结30天",
  "delivery_address": "工厂仓库",
  "contact_person": "王采购",
  "contact_phone": "13800138000",
  "total_amount": 100000.00,
  "currency": "CNY",
  "remarks": "首批采购"
}
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "PO20240101abc12345",
    "supplier_id": "550e8400-e29b-41d4-a716-446655440002",
    "order_date": "2024-01-01",
    "expected_delivery_date": "2024-01-15",
    "payment_terms": "月结30天",
    "delivery_address": "工厂仓库",
    "contact_person": "王采购",
    "contact_phone": "13800138000",
    "total_amount": "100000.0000",
    "currency": "CNY",
    "remarks": "首批采购",
    "status": "draft",
    "created_by": null,
    "approved_by": null,
    "approved_at": null,
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-01 10:00:00"
  }
}
```

### 3. 获取单个采购订单

**请求**
```
GET /api/v1/scm/tenants/:tenant_id/purchase-orders/:id
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "PO20240101abc12345",
    "supplier_id": "550e8400-e29b-41d4-a716-446655440002",
    "order_date": "2024-01-01",
    "expected_delivery_date": "2024-01-15",
    "payment_terms": "月结30天",
    "delivery_address": "工厂仓库",
    "contact_person": "王采购",
    "contact_phone": "13800138000",
    "total_amount": "100000.0000",
    "currency": "CNY",
    "remarks": "首批采购",
    "status": "draft",
    "created_by": null,
    "approved_by": null,
    "approved_at": null,
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-01 10:00:00"
  }
}
```

### 4. 更新采购订单

**请求**
```
PUT /api/v1/scm/tenants/:tenant_id/purchase-orders/:id
Content-Type: application/json
```

**请求体**
```json
{
  "expected_delivery_date": "2024-01-20",
  "payment_terms": "月结45天",
  "total_amount": 120000.00,
  "status": "submitted"
}
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "PO20240101abc12345",
    "supplier_id": "550e8400-e29b-41d4-a716-446655440002",
    "order_date": "2024-01-01",
    "expected_delivery_date": "2024-01-20",
    "payment_terms": "月结45天",
    "delivery_address": "工厂仓库",
    "contact_person": "王采购",
    "contact_phone": "13800138000",
    "total_amount": "120000.0000",
    "currency": "CNY",
    "remarks": "首批采购",
    "status": "submitted",
    "created_by": null,
    "approved_by": null,
    "approved_at": null,
    "created_at": "2024-01-01 10:00:00",
    "updated_at": "2024-01-02 14:30:00"
  }
}
```

### 5. 删除采购订单

**请求**
```
DELETE /api/v1/scm/tenants/:tenant_id/purchase-orders/:id
```

**响应示例**
```json
{
  "code": 200,
  "message": "success",
  "data": null
}
```

---

## 数据字典

### 供应商状态（status）
- `active`: 活跃
- `inactive`: 停用
- `blocked`: 冻结

### 供应商类型（supplier_type）
- `原材料`: 原材料供应商
- `辅料`: 辅助材料供应商
- `设备`: 设备供应商
- `服务`: 服务供应商

### 信用等级（credit_level）
- `A`: 优秀
- `B`: 良好
- `C`: 一般
- `D`: 较差

### 采购订单状态（status）
- `draft`: 草稿
- `submitted`: 已提交
- `approved`: 已审批
- `rejected`: 已拒绝
- `completed`: 已完成
- `cancelled`: 已取消

### 币种（currency）
- `CNY`: 人民币
- `USD`: 美元
- `EUR`: 欧元

---

## 健康检查

**请求**
```
GET /health
```

**响应**
```
OK
```
