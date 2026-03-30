# MES 制造执行系统

## 项目简介

MES（Manufacturing Execution System）制造执行系统是位于上层计划管理系统（ERP）与底层工业控制系统之间的车间级管理信息系统。本项目使用 Rust + Axum + Sea-ORM 技术栈实现。

## 功能模块

### 核心业务模块

| 模块 | 说明 | 状态 |
|------|------|------|
| 产品管理 | 产品基础信息、规格型号管理 | ✅ 已完成 |
| 工单管理 | 生产订单接收、执行、完工汇报 | ✅ 已完成 |
| 工艺路线 | 产品工艺定义、工序管理 | ✅ 已完成 |
| 生产调度 | 生产任务排程、资源分配 | ✅ 已完成 |
| 物料管理 | 物料信息管理 | ✅ 已完成 |
| 库存管理 | 库存查询、调整、锁定/解锁 | ✅ 已完成 |
| 出入库管理 | 出入库单据管理 | ✅ 已完成 |
| 生产记录 | 生产执行记录、过程数据采集 | ✅ 已完成 |
| 质量检验 | 检验单管理、检验结果提交 | ✅ 已完成 |
| 不良管理 | 不良记录、不良品处置 | ✅ 已完成 |
| 设备管理 | 设备台账管理 | ✅ 已完成 |
| 维护计划 | 设备保养维护计划 | ✅ 已完成 |
| 员工管理 | 员工信息管理 | ✅ 已完成 |

### 基础设施模块

| 模块 | 说明 | 状态 |
|------|------|------|
| 车间管理 | 车间基础信息 | ✅ 已完成 |
| 产线管理 | 生产产线管理 | ✅ 已完成 |
| 仓库管理 | 仓库基础信息 | ✅ 已完成 |
| 库位管理 | 仓库库位管理 | ✅ 已完成 |
| 工站管理 | 生产工站管理 | ✅ 已完成 |

## 项目结构

```
mes/
├── Cargo.toml              # 项目配置
├── README.md               # 项目说明
├── API文档.md              # API接口文档
├── docs/
│   └── MES系统设计文档.md    # 系统设计文档
├── src/
│   ├── main.rs             # 程序入口
│   ├── config.rs           # 配置管理
│   ├── entity/             # 数据实体定义
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── work_order.rs
│   │   ├── process_route.rs
│   │   ├── process.rs
│   │   ├── schedule_plan.rs
│   │   ├── material.rs
│   │   ├── inventory.rs
│   │   ├── stock_movement.rs
│   │   ├── production_record.rs
│   │   ├── inspection_order.rs
│   │   ├── defect_record.rs
│   │   ├── equipment.rs
│   │   ├── maintenance_plan.rs
│   │   ├── employee.rs
│   │   ├── workshop.rs
│   │   ├── production_line.rs
│   │   ├── warehouse.rs
│   │   ├── location.rs
│   │   └── work_station.rs
│   ├── service/            # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── work_order.rs
│   │   └── ... (其他服务)
│   ├── api/                # API路由层
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── work_order.rs
│   │   └── ... (其他API)
│   ├── migration/          # 数据库迁移
│   ├── response.rs         # 统一响应格式
│   └── utils/              # 工具函数
└── migrations/             # SQL迁移文件
```

## 技术栈

| 技术 | 版本 | 说明 |
|------|------|------|
| Rust | 1.75+ | 编程语言 |
| Axum | 0.7 | Web框架 |
| Sea-ORM | 0.12 | ORM框架 |
| PostgreSQL | 15+ | 数据库 |
| Tokio | 1.x | 异步运行时 |
| Serde | 1.x | 序列化 |
| UUID | 1.x | 唯一标识 |

## 快速开始

### 环境要求

- Rust 1.75+
- PostgreSQL 15+
- Cargo

### 编译运行

```bash
# 进入项目目录
cd mes

# 编译项目
cargo build --release

# 运行项目
cargo run --release
```

### 配置说明

通过环境变量或 `.env` 文件配置：

```env
# 数据库配置
DATABASE_URL=postgresql://user:password@localhost:5432/mes

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8082
```

### 健康检查

```bash
curl http://localhost:8082/health
# 响应: OK
```

## API 示例

### 创建产品

```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/{tenant_id}/products \
  -H "Content-Type: application/json" \
  -d '{
    "product_no": "P001",
    "name": "测试产品",
    "unit": "件"
  }'
```

### 创建工单

```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/{tenant_id}/work-orders \
  -H "Content-Type: application/json" \
  -d '{
    "product_id": "uuid",
    "product_name": "测试产品",
    "quantity": 100
  }'
```

### 开始生产

```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/{tenant_id}/work-orders/{id}/start
```

### 完工汇报

```bash
curl -X POST http://localhost:8082/api/v1/mes/tenants/{tenant_id}/work-orders/{id}/complete \
  -H "Content-Type: application/json" \
  -d '{"completed_qty": 100}'
```

## API 路由一览

### 基础路径

```
/api/v1/mes/tenants/{tenant_id}
```

### 路由列表

| 模块 | 路由 | 方法 | 说明 |
|------|------|------|------|
| 产品管理 | `/products` | GET, POST | 产品列表、创建 |
| 产品管理 | `/products/{id}` | GET, PUT, DELETE | 产品详情、更新、删除 |
| 工单管理 | `/work-orders` | GET, POST | 工单列表、创建 |
| 工单管理 | `/work-orders/{id}` | GET | 工单详情 |
| 工单管理 | `/work-orders/{id}/start` | POST | 开始生产 |
| 工单管理 | `/work-orders/{id}/complete` | POST | 完工汇报 |
| 工艺路线 | `/process-routes` | GET, POST | 工艺路线列表、创建 |
| 工艺路线 | `/process-routes/{id}` | GET, PUT, DELETE | 详情、更新、删除 |
| 工艺路线 | `/process-routes/{id}/set-default` | POST | 设为默认 |
| 工序管理 | `/processes` | GET, POST | 工序列表、创建 |
| 工序管理 | `/processes/{id}` | GET, PUT, DELETE | 工序详情、更新、删除 |
| 排程计划 | `/schedule-plans` | GET, POST | 排程列表、创建 |
| 排程计划 | `/schedule-plans/{id}` | GET, DELETE | 排程详情、删除 |
| 排程计划 | `/schedule-plans/{id}/start` | POST | 开始排程 |
| 排程计划 | `/schedule-plans/{id}/complete` | POST | 完成排程 |
| 物料管理 | `/materials` | GET, POST | 物料列表、创建 |
| 物料管理 | `/materials/{id}` | GET, PUT, DELETE | 物料详情、更新、删除 |
| 库存管理 | `/inventories` | GET | 库存列表 |
| 库存管理 | `/inventories/{id}` | GET | 库存详情 |
| 库存管理 | `/inventories/adjust` | POST | 库存调整 |
| 库存管理 | `/inventories/lock` | POST | 库存锁定 |
| 库存管理 | `/inventories/unlock` | POST | 库存解锁 |
| 出入库 | `/stock-movements` | GET, POST | 出入库列表、创建 |
| 出入库 | `/stock-movements/{id}` | GET | 出入库详情 |
| 出入库 | `/stock-movements/{id}/execute` | POST | 执行出入库 |
| 出入库 | `/stock-movements/{id}/cancel` | POST | 取消出入库 |
| 生产记录 | `/production-records` | GET, POST | 生产记录列表、创建 |
| 生产记录 | `/production-records/{id}` | GET, PUT, DELETE | 记录详情、更新、删除 |
| 检验单 | `/inspection-orders` | GET, POST | 检验单列表、创建 |
| 检验单 | `/inspection-orders/{id}` | GET, DELETE | 检验单详情、删除 |
| 检验单 | `/inspection-orders/{id}/submit` | POST | 提交检验结果 |
| 不良记录 | `/defect-records` | GET, POST | 不良记录列表、创建 |
| 不良记录 | `/defect-records/{id}` | GET, PUT, DELETE | 记录详情、更新、删除 |
| 不良记录 | `/defect-records/{id}/handle` | POST | 处理不良品 |
| 设备管理 | `/equipments` | GET, POST | 设备列表、创建 |
| 设备管理 | `/equipments/{id}` | GET, PUT, DELETE | 设备详情、更新、删除 |
| 维护计划 | `/maintenance-plans` | GET, POST | 维护计划列表、创建 |
| 维护计划 | `/maintenance-plans/{id}` | GET, PUT, DELETE | 计划详情、更新、删除 |
| 维护计划 | `/maintenance-plans/{id}/execute` | POST | 执行维护 |
| 员工管理 | `/employees` | GET, POST | 员工列表、创建 |
| 员工管理 | `/employees/{id}` | GET, PUT, DELETE | 员工详情、更新、删除 |
| 车间管理 | `/workshops` | GET, POST | 车间列表、创建 |
| 车间管理 | `/workshops/{id}` | GET, PUT, DELETE | 车间详情、更新、删除 |
| 产线管理 | `/production-lines` | GET, POST | 产线列表、创建 |
| 产线管理 | `/production-lines/{id}` | GET, PUT, DELETE | 产线详情、更新、删除 |
| 仓库管理 | `/warehouses` | GET, POST | 仓库列表、创建 |
| 仓库管理 | `/warehouses/{id}` | GET, PUT, DELETE | 仓库详情、更新、删除 |
| 库位管理 | `/locations` | GET, POST | 库位列表、创建 |
| 库位管理 | `/locations/{id}` | GET, PUT, DELETE | 库位详情、更新、删除 |
| 工站管理 | `/work-stations` | GET, POST | 工站列表、创建 |
| 工站管理 | `/work-stations/{id}` | GET, PUT, DELETE | 工站详情、更新、删除 |

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
  "message": "错误信息"
}
```

## 多租户设计

系统采用多租户设计，所有业务数据通过 `tenant_id` 字段进行隔离。API 路径中包含租户ID：

```
/api/v1/mes/tenants/{tenant_id}/...
```

## 相关文档

- [API接口文档](./API文档.md) - 详细的API接口说明
- [系统设计文档](./docs/MES系统设计文档.md) - 系统架构和设计说明

## 许可证

MIT License
