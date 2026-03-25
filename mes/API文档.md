# MES 制造执行系统 API 接口文档

## 基本信息

- **服务地址**: `http://0.0.0.0:8082`
- **API版本**: v1
- **基础路径**: `/api/v1/mes/tenants/{tenant_id}`
- **健康检查**: `GET /health`

## 统一响应格式

### 成功响应

```json
{
  "code": 200,
  "message": "success",
  "data": { /* 业务数据 */ }
}
```

### 错误响应

```json
{
  "code": 400,
  "message": "错误信息描述"
}
```

---

## 1. 产品管理 API

### 1.1 获取产品列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/products
```

**路径参数**:
- `tenant_id`: 租户ID (UUID格式)

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "product_no": "P001",
      "name": "产品名称",
      "specification": "规格型号",
      "unit": "件",
      "product_type": "成品",
      "description": "产品描述",
      "status": "active",
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 1.2 创建产品

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/products
```

**请求体**:
```json
{
  "product_no": "P001",
  "name": "产品名称",
  "specification": "规格型号",
  "unit": "件",
  "product_type": "成品",
  "description": "产品描述"
}
```

**字段说明**:
- `product_no` (必填): 产品编号
- `name` (必填): 产品名称
- `specification` (可选): 规格型号
- `unit` (可选): 单位
- `product_type` (可选): 产品类型
- `description` (可选): 产品描述

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "product_no": "P001",
    "name": "产品名称",
    "specification": "规格型号",
    "unit": "件",
    "product_type": "成品",
    "description": "产品描述",
    "status": "active",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

### 1.3 获取单个产品

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/products/{id}
```

**路径参数**:
- `tenant_id`: 租户ID
- `id`: 产品ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "product_no": "P001",
    "name": "产品名称",
    "specification": "规格型号",
    "unit": "件",
    "product_type": "成品",
    "description": "产品描述",
    "status": "active",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

### 1.4 更新产品

**请求**:
```
PUT /api/v1/mes/tenants/{tenant_id}/products/{id}
```

**请求体**:
```json
{
  "name": "新产品名称",
  "specification": "新规格",
  "unit": "台",
  "product_type": "半成品",
  "description": "新描述",
  "status": "inactive"
}
```

**字段说明**:
- 所有字段均为可选，只更新提交的字段

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "product_no": "P001",
    "name": "新产品名称",
    "specification": "新规格",
    "unit": "台",
    "product_type": "半成品",
    "description": "新描述",
    "status": "inactive",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T11:00:00"
  }
}
```

### 1.5 删除产品

**请求**:
```
DELETE /api/v1/mes/tenants/{tenant_id}/products/{id}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": null
}
```

**注意**: 删除操作为软删除，将状态设置为 "deleted"

---

## 2. 工单管理 API

### 2.1 获取工单列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "order_no": "WO20260324100000",
      "erp_order_no": "ERP001",
      "product_id": "550e8400-e29b-41d4-a716-446655440002",
      "product_name": "产品名称",
      "quantity": "100.0000",
      "completed_qty": "0.0000",
      "status": "pending",
      "priority": 1,
      "plan_start_time": "2026-03-24T08:00:00",
      "plan_end_time": "2026-03-24T17:00:00",
      "actual_start_time": null,
      "actual_end_time": null,
      "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
      "production_line_id": "550e8400-e29b-41d4-a716-446655440004",
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 2.2 创建工单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/work-orders
```

**请求体**:
```json
{
  "erp_order_no": "ERP001",
  "product_id": "550e8400-e29b-41d4-a716-446655440002",
  "product_name": "产品名称",
  "quantity": 100.0,
  "priority": 1,
  "plan_start_time": "2026-03-24T08:00:00Z",
  "plan_end_time": "2026-03-24T17:00:00Z",
  "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
  "production_line_id": "550e8400-e29b-41d4-a716-446655440004"
}
```

**字段说明**:
- `product_id` (必填): 产品ID
- `product_name` (必填): 产品名称
- `quantity` (必填): 生产数量
- `erp_order_no` (可选): ERP订单号
- `priority` (可选): 优先级，默认为0
- `plan_start_time` (可选): 计划开始时间，RFC3339格式
- `plan_end_time` (可选): 计划结束时间，RFC3339格式
- `workshop_id` (可选): 车间ID
- `production_line_id` (可选): 产线ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "WO20260324100000",
    "erp_order_no": "ERP001",
    "product_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_name": "产品名称",
    "quantity": "100.0000",
    "completed_qty": "0.0000",
    "status": "pending",
    "priority": 1,
    "plan_start_time": "2026-03-24T08:00:00",
    "plan_end_time": "2026-03-24T17:00:00",
    "actual_start_time": null,
    "actual_end_time": null,
    "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
    "production_line_id": "550e8400-e29b-41d4-a716-446655440004",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

### 2.3 获取单个工单

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders/{id}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "WO20260324100000",
    "erp_order_no": "ERP001",
    "product_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_name": "产品名称",
    "quantity": "100.0000",
    "completed_qty": "0.0000",
    "status": "pending",
    "priority": 1,
    "plan_start_time": "2026-03-24T08:00:00",
    "plan_end_time": "2026-03-24T17:00:00",
    "actual_start_time": null,
    "actual_end_time": null,
    "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
    "production_line_id": "550e8400-e29b-41d4-a716-446655440004",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

### 2.4 开始工单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/work-orders/{id}/start
```

**说明**: 将工单状态从 "pending" 更新为 "in_progress"，并记录实际开始时间

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "WO20260324100000",
    "erp_order_no": "ERP001",
    "product_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_name": "产品名称",
    "quantity": "100.0000",
    "completed_qty": "0.0000",
    "status": "in_progress",
    "priority": 1,
    "plan_start_time": "2026-03-24T08:00:00",
    "plan_end_time": "2026-03-24T17:00:00",
    "actual_start_time": "2026-03-24T08:30:00",
    "actual_end_time": null,
    "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
    "production_line_id": "550e8400-e29b-41d4-a716-446655440004",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T08:30:00"
  }
}
```

### 2.5 完成工单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/work-orders/{id}/complete
```

**请求体**:
```json
{
  "completed_qty": 100.0
}
```

**字段说明**:
- `completed_qty` (必填): 完成数量

**说明**: 
- 如果完成数量 >= 计划数量，工单状态更新为 "completed"
- 否则状态保持为 "in_progress"
- 同时记录实际结束时间

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_no": "WO20260324100000",
    "erp_order_no": "ERP001",
    "product_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_name": "产品名称",
    "quantity": "100.0000",
    "completed_qty": "100.0000",
    "status": "completed",
    "priority": 1,
    "plan_start_time": "2026-03-24T08:00:00",
    "plan_end_time": "2026-03-24T17:00:00",
    "actual_start_time": "2026-03-24T08:30:00",
    "actual_end_time": "2026-03-24T16:30:00",
    "workshop_id": "550e8400-e29b-41d4-a716-446655440003",
    "production_line_id": "550e8400-e29b-41d4-a716-446655440004",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T16:30:00"
  }
}
```

**工单状态说明**:
- `pending`: 待处理
- `in_progress`: 进行中
- `completed`: 已完成

---

## 3. 物料管理 API

### 3.1 获取物料列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/materials
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "material_no": "M001",
      "material_name": "物料名称",
      "specification": "规格型号",
      "unit": "千克",
      "material_type": "原料",
      "safety_stock": "100.0000",
      "max_stock": "1000.0000",
      "status": "active",
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 3.2 创建物料

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/materials
```

**请求体**:
```json
{
  "material_no": "M001",
  "material_name": "物料名称",
  "specification": "规格型号",
  "unit": "千克",
  "material_type": "原料",
  "safety_stock": 100.0,
  "max_stock": 1000.0
}
```

**字段说明**:
- `material_no` (必填): 物料编号
- `material_name` (必填): 物料名称
- `specification` (可选): 规格型号
- `unit` (可选): 单位
- `material_type` (可选): 物料类型
- `safety_stock` (可选): 安全库存，默认为0
- `max_stock` (可选): 最大库存，默认为0

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "material_no": "M001",
    "material_name": "物料名称",
    "specification": "规格型号",
    "unit": "千克",
    "material_type": "原料",
    "safety_stock": "100.0000",
    "max_stock": "1000.0000",
    "status": "active",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

---

## 4. 设备管理 API

### 4.1 获取设备列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/equipments
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "equipment_no": "E001",
      "equipment_name": "设备名称",
      "equipment_type": "加工设备",
      "model": "型号A",
      "manufacturer": "制造商",
      "purchase_date": "2025-01-01",
      "workshop_id": "550e8400-e29b-41d4-a716-446655440002",
      "status": "idle",
      "ip_address": "192.168.1.100",
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 4.2 创建设备

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/equipments
```

**请求体**:
```json
{
  "equipment_no": "E001",
  "equipment_name": "设备名称",
  "equipment_type": "加工设备",
  "model": "型号A",
  "manufacturer": "制造商",
  "purchase_date": "2025-01-01",
  "workshop_id": "550e8400-e29b-41d4-a716-446655440002",
  "ip_address": "192.168.1.100"
}
```

**字段说明**:
- `equipment_no` (必填): 设备编号
- `equipment_name` (必填): 设备名称
- `equipment_type` (可选): 设备类型
- `model` (可选): 型号
- `manufacturer` (可选): 制造商
- `purchase_date` (可选): 购买日期，格式: YYYY-MM-DD
- `workshop_id` (可选): 车间ID
- `ip_address` (可选): IP地址

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "equipment_no": "E001",
    "equipment_name": "设备名称",
    "equipment_type": "加工设备",
    "model": "型号A",
    "manufacturer": "制造商",
    "purchase_date": "2025-01-01",
    "workshop_id": "550e8400-e29b-41d4-a716-446655440002",
    "status": "idle",
    "ip_address": "192.168.1.100",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

**设备状态说明**:
- `idle`: 空闲
- `running`: 运行中
- `maintenance`: 维护中
- `fault`: 故障

---

## 5. 员工管理 API

### 5.1 获取员工列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/employees
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "employee_no": "EMP001",
      "name": "员工姓名",
      "department_id": "550e8400-e29b-41d4-a716-446655440002",
      "position": "操作员",
      "phone": "13800138000",
      "status": "active",
      "entry_date": "2025-01-01",
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 5.2 创建员工

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/employees
```

**请求体**:
```json
{
  "employee_no": "EMP001",
  "name": "员工姓名",
  "department_id": "550e8400-e29b-41d4-a716-446655440002",
  "position": "操作员",
  "phone": "13800138000",
  "entry_date": "2025-01-01"
}
```

**字段说明**:
- `employee_no` (必填): 员工编号
- `name` (必填): 员工姓名
- `department_id` (可选): 部门ID
- `position` (可选): 职位
- `phone` (可选): 手机号
- `entry_date` (可选): 入职日期，格式: YYYY-MM-DD

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "employee_no": "EMP001",
    "name": "员工姓名",
    "department_id": "550e8400-e29b-41d4-a716-446655440002",
    "position": "操作员",
    "phone": "13800138000",
    "status": "active",
    "entry_date": "2025-01-01",
    "created_at": "2026-03-24T10:00:00",
    "updated_at": "2026-03-24T10:00:00"
  }
}
```

**员工状态说明**:
- `active`: 在职
- `inactive`: 离职
- `on_leave`: 休假

---

## 6. 健康检查

**请求**:
```
GET /health
```

**响应**:
```
OK
```

---

## 7. 错误码说明

| 错误码 | 说明 |
|--------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 404 | 资源未找到 |
| 500 | 服务器内部错误 |

---

## 8. 数据类型说明

### UUID
所有ID均采用UUID格式，示例: `550e8400-e29b-41d4-a716-446655440000`

### 时间格式
- 时间戳: RFC3339格式，例如: `2026-03-24T10:00:00Z`
- 日期: `YYYY-MM-DD` 格式，例如: `2026-03-24`

### 数值类型
数量、库存等数值字段采用高精度Decimal类型，保留4位小数，例如: `100.0000`

---

## 9. 使用示例

### cURL 示例

**创建产品**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/products \
  -H "Content-Type: application/json" \
  -d '{
    "product_no": "P001",
    "name": "测试产品",
    "unit": "件"
  }'
```

**获取产品列表**:
```bash
curl http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/products
```

**创建工单**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/work-orders \
  -H "Content-Type: application/json" \
  -d '{
    "product_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_name": "测试产品",
    "quantity": 100
  }'
```

**开始工单**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/work-orders/550e8400-e29b-41d4-a716-446655440003/start
```

**完成工单**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/work-orders/550e8400-e29b-41d4-a716-446655440003/complete \
  -H "Content-Type: application/json" \
  -d '{
    "completed_qty": 100
  }'
```

---

**文档版本**: v1.0  
**最后更新**: 2026-03-24
