# SCM 供应链管理系统

## 项目简介

SCM（Supply Chain Management）是一个供应链管理系统的后端服务，基于 Rust 语言开发，使用 Axum 框架和 Sea-ORM 数据库 ORM。

## 功能模块

### 1. 供应商管理（Supplier）
- 供应商信息管理
- 供应商分类与评级
- 联系人信息维护
- 银行账户信息管理

### 2. 采购订单管理（Purchase Order）
- 采购订单创建与维护
- 订单状态管理（草稿、已提交、已审批、已完成）
- 订单明细管理
- 供应商关联

### 3. 采购订单明细（Purchase Order Item）
- 订单明细行项管理
- 物料信息与数量维护
- 价格与金额计算
- 预计交付日期跟踪

## 技术栈

- **Web 框架**: Axum 0.7
- **数据库 ORM**: Sea-ORM 0.12
- **数据库**: PostgreSQL
- **异步运行时**: Tokio
- **序列化**: Serde
- **日志**: Tracing
- **配置管理**: Config

## 项目结构

```
scm/
├── Cargo.toml              # 项目配置文件
├── README.md               # 项目说明文档
└── src/
    ├── api/                # API 路由层
    │   ├── mod.rs
    │   ├── supplier.rs     # 供应商 API
    │   └── purchase_order.rs # 采购订单 API
    ├── entity/             # 数据库实体层
    │   ├── mod.rs
    │   ├── supplier.rs     # 供应商实体
    │   ├── purchase_order.rs # 采购订单实体
    │   └── purchase_order_item.rs # 订单明细实体
    ├── service/            # 业务逻辑层
    │   ├── mod.rs
    │   ├── supplier.rs     # 供应商服务
    │   └── purchase_order.rs # 采购订单服务
    ├── migration/          # 数据库迁移
    │   ├── mod.rs
    │   ├── m20240101_000001_create_suppliers.rs
    │   ├── m20240101_000002_create_purchase_orders.rs
    │   └── m20240101_000003_create_purchase_order_items.rs
    ├── config/             # 配置模块
    ├── response/           # 响应格式
    ├── utils/              # 工具模块
    └── main.rs             # 程序入口
```

## API 接口

### 供应商接口

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/scm/tenants/:tenant_id/suppliers` | 获取供应商列表 |
| POST | `/api/v1/scm/tenants/:tenant_id/suppliers` | 创建供应商 |
| GET | `/api/v1/scm/tenants/:tenant_id/suppliers/:id` | 获取单个供应商 |
| PUT | `/api/v1/scm/tenants/:tenant_id/suppliers/:id` | 更新供应商 |
| DELETE | `/api/v1/scm/tenants/:tenant_id/suppliers/:id` | 删除供应商 |

### 采购订单接口

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/scm/tenants/:tenant_id/purchase-orders` | 获取采购订单列表 |
| POST | `/api/v1/scm/tenants/:tenant_id/purchase-orders` | 创建采购订单 |
| GET | `/api/v1/scm/tenants/:tenant_id/purchase-orders/:id` | 获取单个采购订单 |
| PUT | `/api/v1/scm/tenants/:tenant_id/purchase-orders/:id` | 更新采购订单 |
| DELETE | `/api/v1/scm/tenants/:tenant_id/purchase-orders/:id` | 删除采购订单 |

## 数据库设计

### 供应商表（scm_suppliers）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 主键 |
| tenant_id | UUID | 租户ID |
| supplier_code | Text | 供应商编码 |
| supplier_name | Text | 供应商名称 |
| contact_person | Text | 联系人 |
| contact_phone | Text | 联系电话 |
| contact_email | Text | 联系邮箱 |
| address | Text | 地址 |
| bank_name | Text | 银行名称 |
| bank_account | Text | 银行账号 |
| tax_number | Text | 税号 |
| supplier_type | Text | 供应商类型 |
| credit_level | Text | 信用等级 |
| remarks | Text | 备注 |
| status | Text | 状态 |
| created_at | Timestamp | 创建时间 |
| updated_at | Timestamp | 更新时间 |

### 采购订单表（scm_purchase_orders）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 主键 |
| tenant_id | UUID | 租户ID |
| order_no | Text | 订单号 |
| supplier_id | UUID | 供应商ID |
| order_date | Date | 订单日期 |
| expected_delivery_date | Date | 预计交付日期 |
| payment_terms | Text | 付款条件 |
| delivery_address | Text | 交付地址 |
| contact_person | Text | 联系人 |
| contact_phone | Text | 联系电话 |
| total_amount | Decimal | 总金额 |
| currency | Text | 币种 |
| remarks | Text | 备注 |
| status | Text | 状态 |
| created_by | UUID | 创建人 |
| approved_by | UUID | 审批人 |
| approved_at | Timestamp | 审批时间 |
| created_at | Timestamp | 创建时间 |
| updated_at | Timestamp | 更新时间 |

### 采购订单明细表（scm_purchase_order_items）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 主键 |
| tenant_id | UUID | 租户ID |
| order_id | UUID | 订单ID |
| material_id | UUID | 物料ID |
| material_name | Text | 物料名称 |
| specification | Text | 规格 |
| unit | Text | 单位 |
| quantity | Decimal | 数量 |
| unit_price | Decimal | 单价 |
| total_price | Decimal | 总价 |
| expected_delivery_date | Date | 预计交付日期 |
| remarks | Text | 备注 |
| created_at | Timestamp | 创建时间 |
| updated_at | Timestamp | 更新时间 |

## 配置

配置文件支持从 `config.json` 或环境变量加载。默认配置：

```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 8081
  },
  "database": {
    "url": "postgres://postgres:postgres@localhost:5432/scm"
  }
}
```

环境变量支持：
- `SCM_SERVER_HOST`: 服务器地址
- `SCM_SERVER_PORT`: 服务器端口
- `SCM_DATABASE_URL`: 数据库连接字符串

## 运行

```bash
# 编译项目
cargo build

# 运行服务
cargo run

# 运行测试
cargo test
```

## 健康检查

```bash
curl http://localhost:8081/health
```

## 开发计划

- [ ] 采购申请管理
- [ ] 供应商报价管理
- [ ] 采购合同管理
- [ ] 供应商绩效评估
- [ ] 采购统计分析
- [ ] 供应商门户集成
- [ ] 电子招标管理
- [ ] 采购审批流程
- [ ] 库存对接

## 许可证

MIT License
