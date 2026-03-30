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
  "success": true,
  "data": { /* 业务数据 */ },
  "message": "操作成功"
}
```

### 错误响应

```json
{
  "success": false,
  "data": null,
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
  "success": true,
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

### 1.3 获取单个产品

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/products/{id}
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
  "status": "inactive"
}
```

### 1.5 删除产品

**请求**:
```
DELETE /api/v1/mes/tenants/{tenant_id}/products/{id}
```

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
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "order_no": "WO20260324100000",
      "erp_order_no": "ERP001",
      "product_id": "550e8400-e29b-41d4-a716-446655440002",
      "product_name": "产品名称",
      "quantity": "100.0000",
      "completed_qty": "0.0000",
      "status": "pending",
      "priority": 1,
      "plan_start_time": "2026-03-24T08:00:00",
      "plan_end_time": "2026-03-24T17:00:00"
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

### 2.3 获取单个工单

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders/{id}
```

### 2.4 开始工单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/work-orders/{id}/start
```

**说明**: 将工单状态从 "pending" 更新为 "in_progress"，并记录实际开始时间

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

**说明**: 如果完成数量 >= 计划数量，工单状态更新为 "completed"

**工单状态说明**:
- `pending`: 待处理
- `in_progress`: 进行中
- `completed`: 已完成

---

## 3. 工艺路线管理 API

### 3.1 获取工艺路线列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/process-routes
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "product_id": "550e8400-e29b-41d4-a716-446655440002",
      "route_name": "工艺路线A",
      "version": "1.0",
      "status": "active",
      "is_default": true,
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 3.2 创建工艺路线

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/process-routes
```

**请求体**:
```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440002",
  "route_name": "工艺路线A",
  "version": "1.0",
  "is_default": true
}
```

### 3.3 获取单个工艺路线

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/process-routes/{id}
```

### 3.4 更新工艺路线

**请求**:
```
PUT /api/v1/mes/tenants/{tenant_id}/process-routes/{id}
```

**请求体**:
```json
{
  "route_name": "新名称",
  "version": "2.0",
  "status": "active"
}
```

### 3.5 删除工艺路线

**请求**:
```
DELETE /api/v1/mes/tenants/{tenant_id}/process-routes/{id}
```

### 3.6 设置为默认工艺路线

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/process-routes/{id}/set-default
```

### 3.7 按产品查询工艺路线

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/products/{product_id}/process-routes
```

---

## 4. 工序管理 API

### 4.1 获取工序列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/processes
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "route_id": "550e8400-e29b-41d4-a716-446655440002",
      "process_no": "OP001",
      "process_name": "下料",
      "sequence": 1,
      "work_station_id": "550e8400-e29b-41d4-a716-446655440003",
      "standard_time": "10.50",
      "setup_time": "2.00",
      "process_params": null,
      "next_process_id": null,
      "created_at": "2026-03-24T10:00:00",
      "updated_at": "2026-03-24T10:00:00"
    }
  ]
}
```

### 4.2 创建工序

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/processes
```

**请求体**:
```json
{
  "route_id": "550e8400-e29b-41d4-a716-446655440002",
  "process_no": "OP001",
  "process_name": "下料",
  "sequence": 1,
  "work_station_id": "550e8400-e29b-41d4-a716-446655440003",
  "standard_time": 10.5,
  "setup_time": 2.0
}
```

### 4.3 按工艺路线查询工序

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/process-routes/{route_id}/processes
```

---

## 5. 排程计划管理 API

### 5.1 获取排程列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/schedule-plans
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "plan_no": "SP20260324100000",
      "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
      "process_id": "550e8400-e29b-41d4-a716-446655440003",
      "equipment_id": "550e8400-e29b-41d4-a716-446655440004",
      "operator_id": "550e8400-e29b-41d4-a716-446655440005",
      "plan_quantity": "100.0000",
      "status": "pending",
      "start_time": "2026-03-24T08:00:00",
      "end_time": "2026-03-24T17:00:00"
    }
  ]
}
```

### 5.2 创建排程

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/schedule-plans
```

**请求体**:
```json
{
  "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
  "process_id": "550e8400-e29b-41d4-a716-446655440003",
  "equipment_id": "550e8400-e29b-41d4-a716-446655440004",
  "operator_id": "550e8400-e29b-41d4-a716-446655440005",
  "plan_quantity": 100.0,
  "start_time": "2026-03-24T08:00:00",
  "end_time": "2026-03-24T17:00:00"
}
```

### 5.3 开始排程

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/schedule-plans/{id}/start
```

### 5.4 完成排程

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/schedule-plans/{id}/complete
```

### 5.5 按工单查询排程

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders/{work_order_id}/schedule-plans
```

---

## 6. 物料管理 API

### 6.1 获取物料列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/materials
```

**响应示例**:
```json
{
  "success": true,
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
      "status": "active"
    }
  ]
}
```

### 6.2 创建物料

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

### 6.3 获取单个物料

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/materials/{id}
```

### 6.4 更新物料

**请求**:
```
PUT /api/v1/mes/tenants/{tenant_id}/materials/{id}
```

### 6.5 删除物料

**请求**:
```
DELETE /api/v1/mes/tenants/{tenant_id}/materials/{id}
```

---

## 7. 库存管理 API

### 7.1 获取库存列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/inventories
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "material_id": "550e8400-e29b-41d4-a716-446655440002",
      "warehouse_id": "550e8400-e29b-41d4-a716-446655440003",
      "location_id": "550e8400-e29b-41d4-a716-446655440004",
      "batch_no": "B2026032401",
      "quantity": "100.0000",
      "locked_qty": "10.0000",
      "available_qty": "90.0000"
    }
  ]
}
```

### 7.2 获取单个库存

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/inventories/{id}
```

### 7.3 库存调整

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/inventories/adjust
```

**请求体**:
```json
{
  "material_id": "550e8400-e29b-41d4-a716-446655440002",
  "warehouse_id": "550e8400-e29b-41d4-a716-446655440003",
  "location_id": "550e8400-e29b-41d4-a716-446655440004",
  "batch_no": "B2026032401",
  "quantity": 50.0,
  "adjustment_type": "in"
}
```

**adjustment_type 说明**:
- `in`: 入库
- `out`: 出库
- `adjust`: 盘点调整

### 7.4 库存锁定

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/inventories/lock
```

**请求体**:
```json
{
  "material_id": "550e8400-e29b-41d4-a716-446655440002",
  "warehouse_id": "550e8400-e29b-41d4-a716-446655440003",
  "batch_no": "B2026032401",
  "quantity": 10.0
}
```

### 7.5 库存解锁

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/inventories/unlock
```

### 7.6 按物料查询库存

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/materials/{material_id}/inventories
```

---

## 8. 出入库管理 API

### 8.1 获取出入库单列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/stock-movements
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "movement_no": "SM20260324100000",
      "movement_type": "in",
      "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
      "material_id": "550e8400-e29b-41d4-a716-446655440003",
      "quantity": "100.0000",
      "batch_no": "B2026032401",
      "operator_id": "550e8400-e29b-41d4-a716-446655440004",
      "status": "pending"
    }
  ]
}
```

### 8.2 创建出入库单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/stock-movements
```

**请求体**:
```json
{
  "movement_type": "in",
  "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
  "material_id": "550e8400-e29b-41d4-a716-446655440003",
  "quantity": 100.0,
  "batch_no": "B2026032401",
  "operator_id": "550e8400-e29b-41d4-a716-446655440004"
}
```

**movement_type 说明**:
- `in`: 入库
- `out`: 出库
- `transfer`: 调拨

### 8.3 执行出入库

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/stock-movements/{id}/execute
```

### 8.4 取消出入库

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/stock-movements/{id}/cancel
```

---

## 9. 生产记录 API

### 9.1 获取生产记录列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/production-records
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "work_order_id": "550e8400-e29b-41d4-a716-446655440001",
      "process_id": "550e8400-e29b-41d4-a716-446655440002",
      "equipment_id": "550e8400-e29b-41d4-a716-446655440003",
      "operator_id": "550e8400-e29b-41d4-a716-446655440004",
      "batch_no": "B2026032401",
      "quantity": "100.0000",
      "good_qty": "98.0000",
      "defect_qty": "2.0000",
      "start_time": "2026-03-24T08:00:00",
      "end_time": "2026-03-24T17:00:00",
      "process_data": null
    }
  ]
}
```

### 9.2 创建生产记录

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/production-records
```

**请求体**:
```json
{
  "work_order_id": "550e8400-e29b-41d4-a716-446655440001",
  "process_id": "550e8400-e29b-41d4-a716-446655440002",
  "equipment_id": "550e8400-e29b-41d4-a716-446655440003",
  "operator_id": "550e8400-e29b-41d4-a716-446655440004",
  "batch_no": "B2026032401",
  "quantity": 100.0,
  "start_time": "2026-03-24T08:00:00Z"
}
```

### 9.3 按工单查询生产记录

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders/{work_order_id}/production-records
```

### 9.4 按工序查询生产记录

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/processes/{process_id}/production-records
```

---

## 10. 检验管理 API

### 10.1 获取检验单列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/inspection-orders
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "inspection_no": "IQC20260324100000",
      "inspection_type": "IQC",
      "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
      "material_id": "550e8400-e29b-41d4-a716-446655440003",
      "batch_no": "B2026032401",
      "sample_qty": 10,
      "pass_qty": 10,
      "defect_qty": 0,
      "result": "pass",
      "inspector_id": "550e8400-e29b-41d4-a716-446655440004",
      "inspect_time": "2026-03-24T10:00:00"
    }
  ]
}
```

### 10.2 创建检验单

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/inspection-orders
```

**请求体**:
```json
{
  "inspection_type": "IQC",
  "work_order_id": "550e8400-e29b-41d4-a716-446655440002",
  "material_id": "550e8400-e29b-41d4-a716-446655440003",
  "sample_qty": 10,
  "inspector_id": "550e8400-e29b-41d4-a716-446655440004"
}
```

**inspection_type 说明**:
- `IQC`: 来料检验
- `IPQC`: 过程检验
- `FQC`: 成品检验

### 10.3 提交检验结果

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/inspection-orders/{id}/submit
```

**请求体**:
```json
{
  "pass_qty": 10,
  "defect_qty": 0,
  "result": "pass",
  "inspector_id": "550e8400-e29b-41d4-a716-446655440004"
}
```

**result 说明**:
- `pass`: 合格
- `fail`: 不合格
- `pending`: 待定

### 10.4 按类型查询检验单

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/inspection-orders/type/{inspection_type}
```

### 10.5 按工单查询检验单

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-orders/{work_order_id}/inspection-orders
```

---

## 11. 不良记录 API

### 11.1 获取不良记录列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/defect-records
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "inspection_id": "550e8400-e29b-41d4-a716-446655440002",
      "defect_type_id": "550e8400-e29b-41d4-a716-446655440003",
      "defect_code": "D001",
      "quantity": 2,
      "description": "表面划痕",
      "disposition": "pending",
      "status": "pending"
    }
  ]
}
```

### 11.2 创建不良记录

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/defect-records
```

**请求体**:
```json
{
  "inspection_id": "550e8400-e29b-41d4-a716-446655440002",
  "quantity": 2,
  "defect_type_id": "550e8400-e29b-41d4-a716-446655440003",
  "defect_code": "D001",
  "description": "表面划痕"
}
```

### 11.3 处理不良品

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/defect-records/{id}/handle
```

**请求体**:
```json
{
  "disposition": "rework"
}
```

**disposition 说明**:
- `pending`: 待处理
- `rework`: 返工
- `scrap`: 报废
- `concession`: 特采

### 11.4 按检验单查询不良记录

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/inspections/{inspection_id}/defect-records
```

---

## 12. 设备管理 API

### 12.1 获取设备列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/equipments
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "equipment_no": "E001",
      "equipment_name": "CNC加工中心",
      "equipment_type": "加工设备",
      "model": "VMC850",
      "manufacturer": "海天精工",
      "purchase_date": "2025-01-01",
      "workshop_id": "550e8400-e29b-41d4-a716-446655440002",
      "status": "idle",
      "ip_address": "192.168.1.100"
    }
  ]
}
```

### 12.2 创建设备

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/equipments
```

**请求体**:
```json
{
  "equipment_no": "E001",
  "equipment_name": "CNC加工中心",
  "equipment_type": "加工设备",
  "model": "VMC850",
  "manufacturer": "海天精工",
  "purchase_date": "2025-01-01",
  "workshop_id": "550e8400-e29b-41d4-a716-446655440002",
  "ip_address": "192.168.1.100"
}
```

### 12.3 更新设备

**请求**:
```
PUT /api/v1/mes/tenants/{tenant_id}/equipments/{id}
```

### 12.4 删除设备

**请求**:
```
DELETE /api/v1/mes/tenants/{tenant_id}/equipments/{id}
```

**设备状态说明**:
- `idle`: 空闲
- `running`: 运行中
- `maintenance`: 维护中
- `fault`: 故障

---

## 13. 维护计划 API

### 13.1 获取维护计划列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/maintenance-plans
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "equipment_id": "550e8400-e29b-41d4-a716-446655440001",
      "plan_type": "monthly",
      "plan_date": "2026-03-25",
      "content": "月度保养",
      "status": "pending",
      "executor_id": null,
      "execute_time": null
    }
  ]
}
```

### 13.2 创建维护计划

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/maintenance-plans
```

**请求体**:
```json
{
  "equipment_id": "550e8400-e29b-41d4-a716-446655440001",
  "plan_type": "monthly",
  "plan_date": "2026-03-25",
  "content": "月度保养"
}
```

**plan_type 说明**:
- `daily`: 日常保养
- `weekly`: 周保养
- `monthly`: 月保养
- `yearly`: 年保养

### 13.3 执行维护

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/maintenance-plans/{id}/execute
```

**请求体**:
```json
{
  "executor_id": "550e8400-e29b-41d4-a716-446655440002",
  "content": "已完成保养，更换润滑油"
}
```

### 13.4 按设备查询维护计划

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/equipment/{equipment_id}/maintenance-plans
```

### 13.5 按状态查询维护计划

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/maintenance-plans/status/{status}
```

---

## 14. 员工管理 API

### 14.1 获取员工列表

**请求**:
```
GET /api/v1/mes/tenants/{tenant_id}/employees
```

**响应示例**:
```json
{
  "success": true,
  "message": "success",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
      "employee_no": "EMP001",
      "name": "张三",
      "department_id": "550e8400-e29b-41d4-a716-446655440002",
      "position": "操作员",
      "phone": "13800138000",
      "status": "active",
      "entry_date": "2025-01-01"
    }
  ]
}
```

### 14.2 创建员工

**请求**:
```
POST /api/v1/mes/tenants/{tenant_id}/employees
```

**请求体**:
```json
{
  "employee_no": "EMP001",
  "name": "张三",
  "department_id": "550e8400-e29b-41d4-a716-446655440002",
  "position": "操作员",
  "phone": "13800138000",
  "entry_date": "2025-01-01"
}
```

**员工状态说明**:
- `active`: 在职
- `inactive`: 离职
- `on_leave`: 休假

---

## 15. 基础设施管理 API

### 15.1 车间管理

**获取车间列表**:
```
GET /api/v1/mes/tenants/{tenant_id}/workshops
```

**创建车间**:
```
POST /api/v1/mes/tenants/{tenant_id}/workshops
```

**请求体**:
```json
{
  "workshop_no": "WS001",
  "workshop_name": "一车间",
  "location": "A栋一楼",
  "description": "主生产车间"
}
```

### 15.2 产线管理

**获取产线列表**:
```
GET /api/v1/mes/tenants/{tenant_id}/production-lines
```

**创建产线**:
```
POST /api/v1/mes/tenants/{tenant_id}/production-lines
```

**请求体**:
```json
{
  "workshop_id": "550e8400-e29b-41d4-a716-446655440001",
  "line_no": "PL001",
  "line_name": "一号产线",
  "description": "主产线"
}
```

**按车间查询产线**:
```
GET /api/v1/mes/tenants/{tenant_id}/workshops/{workshop_id}/production-lines
```

### 15.3 仓库管理

**获取仓库列表**:
```
GET /api/v1/mes/tenants/{tenant_id}/warehouses
```

**创建仓库**:
```
POST /api/v1/mes/tenants/{tenant_id}/warehouses
```

**请求体**:
```json
{
  "warehouse_no": "WH001",
  "warehouse_name": "原料仓",
  "warehouse_type": "raw",
  "location": "B栋一楼",
  "description": "原料存储仓库"
}
```

### 15.4 库位管理

**获取库位列表**:
```
GET /api/v1/mes/tenants/{tenant_id}/locations
```

**创建库位**:
```
POST /api/v1/mes/tenants/{tenant_id}/locations
```

**请求体**:
```json
{
  "warehouse_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_no": "L001",
  "location_name": "A-01-01",
  "location_type": "shelf",
  "description": "A区货架第一层"
}
```

**按仓库查询库位**:
```
GET /api/v1/mes/tenants/{tenant_id}/warehouses/{warehouse_id}/locations
```

### 15.5 工站管理

**获取工站列表**:
```
GET /api/v1/mes/tenants/{tenant_id}/work-stations
```

**创建工站**:
```
POST /api/v1/mes/tenants/{tenant_id}/work-stations
```

**请求体**:
```json
{
  "station_no": "ST001",
  "station_name": "下料工站",
  "workshop_id": "550e8400-e29b-41d4-a716-446655440001",
  "production_line_id": "550e8400-e29b-41d4-a716-446655440002",
  "equipment_id": "550e8400-e29b-41d4-a716-446655440003"
}
```

**按车间查询工站**:
```
GET /api/v1/mes/tenants/{tenant_id}/workshops/{workshop_id}/work-stations
```

**按产线查询工站**:
```
GET /api/v1/mes/tenants/{tenant_id}/production-lines/{production_line_id}/work-stations
```

---

## 16. 健康检查

**请求**:
```
GET /health
```

**响应**:
```
OK
```

---

## 17. 错误码说明

| HTTP状态码 | 说明 |
|------------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 404 | 资源未找到 |
| 500 | 服务器内部错误 |

---

## 18. 数据类型说明

### UUID
所有ID均采用UUID格式，示例: `550e8400-e29b-41d4-a716-446655440000`

### 时间格式
- 时间戳: RFC3339格式，例如: `2026-03-24T10:00:00Z`
- 日期: `YYYY-MM-DD` 格式，例如: `2026-03-24`

### 数值类型
数量、库存等数值字段采用高精度Decimal类型，保留4位小数，例如: `100.0000`

---

## 19. 使用示例

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

**库存调整**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/inventories/adjust \
  -H "Content-Type: application/json" \
  -d '{
    "material_id": "550e8400-e29b-41d4-a716-446655440002",
    "quantity": 100,
    "adjustment_type": "in"
  }'
```

**创建检验单**:
```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/550e8400-e29b-41d4-a716-446655440001/inspection-orders \
  -H "Content-Type: application/json" \
  -d '{
    "inspection_type": "IQC",
    "material_id": "550e8400-e29b-41d4-a716-446655440002",
    "sample_qty": 10
  }'
```

---

**文档版本**: v2.0  
**最后更新**: 2026-03-30
