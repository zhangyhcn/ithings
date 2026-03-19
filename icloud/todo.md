# 设备管理系统设计

remote_transport在租户表里增加一个配置，每个租户可以有自己的配置

## 菜单结构
**设备管理** 一级菜单，包含以下子菜单：
- 产品定义
- 设备定义  
- 驱动定义
- 节点列表
- 站点定义
- 模板管理

---

## 数据库表设计

### 1. `products` - 产品表（租户级别）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 产品ID |
| tenant_id | UUID | 归属租户 |
| name | string | 产品名称 |
| description | text | 产品描述 |
| thing_model | JSONB | 标准物模型 `{properties, events, services}` |
| created_at | timestamp | 创建时间 |
| updated_at | timestamp | 更新时间 |

### 2. `drivers` - 驱动表（租户级别）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 驱动ID |
| tenant_id | UUID | 归属租户 |
| name | string | 驱动名称 |
| description | text | 驱动描述 |
| protocol_type | string | 协议类型 (modbus-tcp, modbus-rtu, opc-ua, dl/t645 等) |
| image | string | Docker 镜像地址 |
| version | string | 驱动版本 |
| created_at | timestamp | 创建时间 |
| updated_at | timestamp | 更新时间 |

> **说明**：驱动只展示，不支持增删改。驱动版本更新由 CI/CD 流水线自动完成：
> - 更新数据库记录
> - 自动编译 → 构建镜像 → 推送镜像到 registry

### 3. `nodes` - 节点表（租户级别）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 节点ID |
| tenant_id | UUID | 归属租户 |
| name | string | 节点名称 |
| address | string | 节点地址 |
| k8s_context | string | K8s 配置上下文 |
| is_shared | boolean | 是否共享节点 |
| status | string | 节点状态 (online/offline) |
| last_sync | timestamp | 最后同步时间 |
| created_at | timestamp | 创建时间 |
| updated_at | timestamp | 更新时间 |

> **说明**：
> - `is_shared = true`: 共享节点（云端 K3s 集群，多租户共享）
> - `is_shared = false`: 私有节点（边缘网关/ARM 设备，绑定特定站点，不共享）
> - 定时同步（每 300 秒）：通过 K8s client 同步节点状态

### 4. `sites` - 站点表（组织级别）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 站点ID |
| tenant_id | UUID | 归属租户 |
| org_id | UUID | 归属组织 |
| name | string | 站点名称 |
| description | text | 站点描述 |
| node_id | UUID | 关联节点 |
| created_at | timestamp | 创建时间 |
| updated_at | timestamp | 更新时间 |

### 5. `device_instances` - 设备实例表（站点级别）
| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 设备实例ID |
| tenant_id | UUID | 归属租户 |
| org_id | UUID | 归属组织 |
| site_id | UUID | 归属站点 |
| name | string | 设备名称 |
| brand_model | string | 品牌型号 |
| product_id | UUID | 关联产品 |
| driver_id | UUID | 关联驱动 |
| poll_interval_ms | int | 轮询间隔 (毫秒) |
| device_type | string | 设备类型 |
| driver_config | JSONB | 驱动配置 `{custom: {...}, profile: {...}}` |
| thing_model | JSONB | 设备物模型（继承产品后可修改） |
| node_id | UUID | 绑定节点 |
| status | string | 设备状态 (running/stopped/error) |
| created_at | timestamp | 创建时间 |
| updated_at | timestamp | 更新时间 |

---

## 功能定义

### 1. 产品定义（租户级别）
- 产品ID (uuid)
- 产品名称（例如：三相电表、单相电表、充电桩、温度传感器等）
- 标准物模型
  - properties: 属性定义
  - events: 事件定义
  - services: 服务/命令定义

---

### 2. 设备定义（租户级别）
- 设备ID (uuid)
- 设备名称
- 品牌型号
- 关联产品ID（继承产品物模型）
- 轮询间隔
- 设备类型
- 驱动配置：
  - 选择驱动类型
  - 定义 device profile：包含 `device_resources`, `device_commands`，以及协议私有配置（Modbus 寄存器地址、数据类型、缩放系数等）

---

### 3. 驱动定义（租户级别）
- 只展示，不支持新增、修改、删除
- 驱动ID、名称、描述、协议类型
- Docker 镜像地址
- 支持驱动版本更新：
  - 更新数据库记录
  - CI/CD 流水线自动编译 → 构建镜像 → 推送镜像到 registry
  - （这部分由 DevOps 流水线实现，本项目不重复造轮子）

---

### 4. 节点列表（租户级别）
- 展示所有节点（通过 K8s client 获取）
- 可以修改共享属性
- **字段**：节点ID、名称、地址、状态等 K8s 节点信息
- **共享节点**标记：
  - `true`: 共享节点（云端 K3s 集群，多个租户/组织/站点共享使用）
  - `false`: 私有节点（边缘网关/ARM 设备，绑定到特定站点，不共享）

---

### 5. 站点定义（组织级别，归属租户）
- 站点ID、名称
- 关联租户、组织
- 关联节点（选择在哪个节点运行）
- 设备列表：该站点下的所有设备实例
- 创建完成后，设备实例自动绑定到节点，形成完整部署关系

**层级寻址**：`tenantId → orgId → siteId → deviceInstanceId`  
**Topic 层级**：`{tenant_id}/{org_id}/{site_id}/{device_id}/data`

---

### 6. 模板管理
- 管理 Kubernetes 部署模板
- 包括：
  - ConfigMap 模板
  - Sidecar 模式 Deployment 模板

---

## 部署流程

1. **完成定义**：产品 → 驱动 → 设备 → 站点 → 绑定节点
2. **生成配置**：
   - 根据默认模板，填充设备实例的配置信息
   - 自动生成 `{site-name}-group.json` 配置文件
   - 自动生成 ConfigMap
   - 自动生成 sidecar 模式的 Kubernetes Deployment YAML
3. **部署方式**：
   - 可以统一发布整个站点的所有设备
   - 也可以单独新增/修改单个设备后重新发布
4. **Kubernetes 部署模式**：
   - **sidecar 模式**：一个 Pod 包含两个容器
     - `container-1`: `driver` - 负责采集，启动后绑定端口等待配置推送
     - `container-2`: `device` - 读取配置，下发给driver，接收采集数据，通过 MQTT/Kafka 远程上报
5. **通过 K8s client 发布**：
   - 可以独立发布单个站点
   - 也可以批量发布多个站点

---

## 配置文件结构（最终生成）

```json
{
  "tenant_id": "tenant-001",
  "org_id": "org-001",
  "site_id": "site-001",
  "namespace_id": "building-a",
  "remote_transport": {
    "type": "mqtt",
    "broker": "tcp://broker.example.com:1883",
    "username": "user",
    "password": "password",
    "client_id": "electric-meter-group-building-a"
  },
  "devices": [
    {
      "device_instance_id": "meter-001",
      "device_name": "Main Entrance Electricity Meter",
      "device_type": "electricity-meter",
      "poll_interval_ms": 1000,
      "driver": {
        "device_instance_id": "meter-001",
        "driver_name": "modbus-driver",
        "driver_type": "modbus",
        "poll_interval_ms": 1000,
        "zmq": {
          "enabled": true,
          "publisher_address": "tcp://*:5555",
          "topic": "modbus/data"
        },
        "logging": {
          "level": "info",
          "format": "json"
        },
        "custom": {
          "host": "172.25.219.101",
          "port": 502,
          "slave_id": 1,
          "profile": {
            "apiVersion": "v1",
            "name": "electricity-meter-three-phase",
            "manufacturer": "ABB",
            "model": "3PH-Meter",
            "labels": ["electricity", "modbus", "three-phase"],
            "description": "Three-phase electricity meter",
            "device_resources": [
              {
                "name": "voltage_a",
                "isHidden": false,
                "description": "Phase A voltage",
                "attributes": {
                  "primaryTable": "INPUT_REGISTERS",
                  "startingAddress": 0,
                  "rawType": "INT16",
                  "scale": 0.1
                },
                "properties": {
                  "value": {
                    "type": "Float32",
                    "readWrite": "R",
                    "defaultValue": 0,
                    "scale": 0.1,
                    "units": "V",
                    "minimum": 0,
                    "maximum": 500
                  }
                }
              }
            ]
          }
        }
      },
      "thing_model": {
        "model_id": "electricity-meter.three-phase.1.0",
        "model_version": "1.0",
        "device_type": "electricity-meter",
        "manufacturer": "ABB",
        "description": "Three-phase electricity meter",
        "properties": [
          {
            "identifier": "voltage_a",
            "name": "Voltage Phase A",
            "type": "float",
            "unit": "V",
            "access": "R",
            "range": [0, 500],
            "default_value": 0,
            "description": "Phase A voltage"
          }
        ],
        "events": [...],
        "rules": [...],
        "commands": [...]
      }
    }
  ]
}
```
