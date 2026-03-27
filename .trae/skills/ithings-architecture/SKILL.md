---
name: "ithings-architecture"
description: "iThings 物联网平台领域模型架构设计文档，定义了产品、设备定义、设备组、设备实例、驱动等概念关系。Invoke when designing or implementing new features related to domain model."
---

# iThings 领域模型架构设计

## 核心概念层级关系

```
租户 (Tenant)
└── 组织 (Organization)
    └── 站点 (Site)
        └── 产品 (Product)
        │   └── 定义标准物模型 (thing_model)
        │   └── 不包含镜像信息
        └── 设备定义 (Device)
            └── 关联产品 (继承物模型)
            └── device_image: 设备运行容器镜像 (必选)
            └── driver_image: 驱动容器镜像 (可选)
            └── device_profile: 默认驱动配置
            └── 具体实现，包含驱动点位映射等信息
        └── 设备组 (Device Group)
            └── 对应一个 Kubernetes Deployment
            └── 不存储镜像信息
            └── 多个设备实例共享同一个 Deployment/Pod
        └── 设备实例 (Device Instance)
            └── 从设备定义选择创建
            └── 关联设备定义（继承设备镜像、驱动配置）
            └── 关联产品（继承物模型，从设备定义获取）
            └── driver_config: 完整驱动配置（可覆盖默认配置）
            └── 轮询间隔等实例特定配置
```

## 表结构设计

### products (产品表)
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | Uuid | ✅ | 主键 |
| tenant_id | Uuid | ✅ | 租户ID |
| name | String | ✅ | 产品名称 |
| description | Text | ❌ | 描述 |
| thing_model | Json | ✅ | 标准物模型定义 |
| rule | Json | ✅ | 规则定义 |
| created_at | Timestamp | ✅ | 创建时间 |
| updated_at | Timestamp | ✅ | 更新时间 |

**说明**: 产品只定义标准物模型，**不包含镜像信息**。镜像信息在设备定义层面。

### devices (设备定义表)
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | Uuid | ✅ | 主键 |
| tenant_id | Uuid | ✅ | 租户ID |
| product_id | Uuid | ❌ | 关联产品ID |
| name | String | ✅ | 设备定义名称 |
| model | String | ❌ | 设备型号 |
| manufacturer | String | ❌ | 厂商 |
| **device_image** | String | ✅ | **设备容器镜像地址** (例如: `device-meter:latest`) |
| **driver_image** | String | ❌ | **驱动容器镜像地址** (例如: `modbus-driver:latest`) |
| device_profile | Json | ✅ | 默认驱动配置，包含 driver_name, driver_type, zmq, logging 等 |
| description | Text | ❌ | 描述 |
| status | String | ✅ | 状态 |
| created_at | Timestamp | ✅ | 创建时间 |
| updated_at | Timestamp | ✅ | 更新时间 |

**说明**: 设备定义是**具体实现**，继承产品的物模型，添加设备镜像和驱动配置。

### drivers (驱动表)
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | Uuid | ✅ | 主键 |
| tenant_id | Uuid | ✅ | 租户ID |
| name | String | ✅ | 驱动名称 |
| description | Text | ❌ | 描述 |
| protocol_type | String | ✅ | 协议类型 (modbus, opcua, etc) |
| **image** | String | ✅ | 驱动容器镜像地址 |
| version | String | ✅ | 版本 |
| device_profile | Json | ✅ | 默认驱动配置模板 |
| created_at | Timestamp | ✅ | 创建时间 |
| updated_at | Timestamp | ✅ | 更新时间 |

### device_groups (设备组表)
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | Uuid | ✅ | 主键 |
| tenant_id | Uuid | ✅ | 租户ID |
| org_id | Uuid | ✅ | 组织ID |
| site_id | Uuid | ✅ | 站点ID |
| name | String | ✅ | 设备组名称 |
| description | Text | ❌ | 描述 |
| node_id | Uuid | ❌ | 部署节点ID |
| status | String | ✅ | 状态 (active/published/disabled) |
| created_at | Timestamp | ✅ | 创建时间 |
| updated_at | Timestamp | ✅ | 更新时间 |

**说明**: 
- 一个设备组对应一个 Kubernetes Deployment → 一个 Pod
- **设备组不存储镜像信息**，所有镜像信息从设备定义获取
- 同一个设备组内所有设备共享同一个驱动，driver_image 从第一个设备定义获取

### device_instances (设备实例表)
| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | Uuid | ✅ | 主键 |
| tenant_id | Uuid | ✅ | 租户ID |
| group_id | Uuid | ✅ | 所属设备组ID |
| **device_id** | Uuid | ✅ | **关联设备定义ID** |
| **product_id** | Uuid | ❌ | 关联产品ID（从设备定义继承） |
| name | String | ✅ | 设备实例名称 |
| **driver_config** | Json | ✅ | **完整驱动配置**，包含:
| | | | `driver_name`, `driver_type`, `poll_interval_ms`, `zmq`, `logging`, `custom` |
| thing_model | Json | ✅ | 自定义物模型（覆盖产品默认值）|
| poll_interval_ms | Integer | ✅ | 轮询间隔(ms) |
| node_id | Uuid | ❌ | 部署节点ID |
| status | String | ✅ | 状态 |
| created_at | Timestamp | ✅ | 创建时间 |
| updated_at | Timestamp | ✅ | 更新时间 |

**说明**: 
- 添加设备实例时，**从设备定义列表选择**，而不是从产品选择
- 产品信息从设备定义关联获取，不需要用户选择
- 每个设备实例可以**独立修改驱动配置**（例如不同的 ZMQ 地址）

## Kubernetes 部署架构

### 部署对应关系
- **一个设备组** → **一个 Deployment** → **一个 Pod** → **N + 1 个容器**
  1. **device 容器**: 每个不同的设备镜像一个容器，相同镜像自动去重复用
     - 所有 device 容器共享同一个 ConfigMap
     - 从 ConfigMap 读取 `/config/config.json`
     - `config.json` 包含完整 `DeviceGroupConfig`，其中 `devices` 数组包含所有设备实例配置
     - device 容器负责初始化所有设备，通过 ZMQ 将每个设备的驱动配置发送给 driver 容器
     - 启动参数: `["-c", "/config/config.json", "--group"]`

  2. **driver 容器**: 一个驱动容器，使用设备定义指定的镜像
     - 同一个设备组只运行一个 driver 容器（假设组内所有设备使用同一个驱动）
     - 从 ConfigMap 读取 `/config/driver-config.json`
     - `driver-config.json` 包含 driver 级别的基础配置
     - driver 启动后绑定 ZMQ 端口，等待 device 发送配置
     - 启动参数: `["-c", "/config/driver-config.json"]`

### ConfigMap 结构
```
ConfigMap: <device-group-id>-config
├── config.json          # DeviceGroupConfig - 所有设备实例配置
│   ├── tenant_id
│   ├── org_id
│   ├── site_id
│   ├── namespace_id
│   ├── remote_transport
│   └── devices[]         # 每个设备实例完整配置
│       ├── device_id
│       ├── device_name
│       ├── device_type
│       ├── poll_interval_ms
│       ├── driver         # 完整 driver_config 从数据库读取
│       │   ├── driver_name
│       │   ├── driver_type
│       │   ├── poll_interval_ms
│       │   ├── zmq
│       │   ├── logging
│       │   └── custom
│       └── thing_model
│           ├── model_id
│           ├── model_version
│           ├── device_type
│           ├── manufacturer
│           ├── description
│           └── properties (from product.thing_model)
└── driver-config.json    # Driver 基础配置
    ├── driver_name
    ├── driver_type
    ├── device_instance_id
    ├── poll_interval_ms
    ├── zmq
    ├── logging
    └── custom
```

### 数据流向
1. 用户在前端创建产品 → 定义物模型
2. 用户创建设备定义 → 选择产品 → 输入设备镜像 → 输入驱动镜像 → 填写驱动配置 → 保存
3. 用户创建设备组 → 只需要名称、描述，不需要配置镜像 → 保存
4. 用户添加设备实例到设备组 → **选择已定义好的设备** → 自动从设备定义填充 product_id 和默认 driver_config → **用户可编辑修改 ZMQ 地址等参数** → 保存
5. 用户点击发布 → 后端:
   - 查询设备组基本信息
   - 查询该组下所有设备实例
   - 对每个实例，查询关联的设备定义 → 获取 device_image，driver_image，然后从设备定义获取 product_id → 查询产品获取 thing_model
   - 收集所有唯一的 device_image，自动生成多个 device 容器配置
   - driver_image 使用第一个设备定义指定的（假设同组共享同一个驱动）
   - 构造 DeviceGroupConfig，创建 ConfigMap
   - 渲染 Deployment 模板，创建 Deployment（包含多个device容器 + 一个driver容器）
6. Kubernetes 启动 Pod:
   - 每个 device 容器读取同一个 config.json，初始化对应设备
   - device 通过 ZMQ 将每个设备的驱动配置发送给 driver
   - driver 接收配置，开始轮询采集

## 前端页面结构

### 页面列表
| 页面 | 位置 | 功能 |
|------|------|------|
| 产品列表 | `/device/product` | CRUD 产品，定义物模型 |
| 驱动列表 | `/device/driver` | CRUD 驱动，管理驱动镜像和默认配置 |
| 设备定义列表 | `/device/device` | CRUD 设备定义，选择产品，配置设备镜像和驱动配置 |
| 设备组列表 | `/device/group` | CRUD 设备组，发布到 Kubernetes，**内嵌设备实例列表** |
| 设备实例列表 | `/device/instance` | CRUD 设备实例，独立配置驱动 |
| 节点列表 | `/device/node` | 管理 Kubernetes 节点 |

### 设备实例创建流程（已更新）
1. 用户打开设备组详情
2. 点击"添加设备实例"
3. **选择设备定义** → 自动从设备定义获取 product_id 和默认 driver_config → 填充到 driver_config JSON 编辑器
4. 用户**可以编辑 JSON 修改 ZMQ 地址等参数** → 满足不同实例不同配置
5. 输入设备实例名称、轮询间隔 → 保存
6. 发布时:
   - 收集所有设备定义的 device_image，去重后为每个唯一镜像创建一个 device 容器
   - driver_image 使用第一个设备定义的驱动镜像
   - 所有设备实例配置都写入 ConfigMap
   - **所有配置都从数据库读取，没有硬编码**

## 设计原则

1. **配置来源正确**:
   - 设备组添加设备 → **从设备定义选择**，不是从产品选择 ✅
   - 产品信息 → 从设备定义关联获取 ✅
   - 设备镜像 → 从设备定义获取 ✅
   - 驱动镜像 → 从设备定义获取 ✅

2. **分离关注点**:
   - 产品：只关心物模型标准
   - 设备定义：关心具体实现（镜像+驱动配置）
   - 设备组：关心部署边界（一个组一个 Deployment）
   - 设备实例：关心实例特定配置

3. **灵活性**:
   - 同一个驱动可以在不同设备组使用不同配置
   - 同一个产品可以有多个设备定义（不同厂商/型号/镜像）
   - 同一个设备定义可以有多个实例（不同地址不同配置）
   - 同一个设备组支持多个不同设备镜像（每个镜像一个容器）

4. **Kubernetes 架构**:
   - 一个设备组 → 一个 Deployment → 一个 Pod → 多个 device 容器 + 一个 driver 容器
   - 多个 device 容器按镜像去重，相同镜像共享一个容器
   - device 容器管理多个设备，通过 ZMQ 分发配置给 driver
   - driver 容器运行驱动协议采集
