# 设备运行文档

## 架构概述

iThings 采用 **device + driver 分离架构**：

- **device 层**：加载物模型，管理业务逻辑、规则、状态机，负责事件发布和服务接收
- **driver 层**：负责具体通信协议（如 Modbus），根据设备配置和物理设备通信，返回标准化数据
- **通信**：device 和 driver 通过 ZMQ 进行通信，支持分布式部署

## 启动流程

### 1. 启动 driver

driver 负责：
- 加载设备配置（Device Profile）
- 启动 ZMQ 服务器等待 device 连接
- 处理 device 读取请求
- 和物理设备通信，返回标准化数据

```bash
# 启动 modbus-driver
cd /root/source/rust/ithings
./target/debug/modbus-driver -c drivers/modbus/modbus.json
```

**示例配置** `drivers/modbus/modbus.json`：

```json
{
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
    "host": "localhost",
    "port": 502,
    "slave_id": 1,
    "profile": {
      "apiVersion": "v2",
      "name": "Modbus-Temperature-Sensor",
      "manufacturer": "EdgeX",
      "model": "MB-TS100",
      "labels": ["modbus", "temperature", "sensor"],
      "description": "Modbus TCP temperature and humidity sensor",
      "deviceResources": [
        {
          "name": "Temperature",
          "isHidden": false,
          "description": "Current temperature reading",
          "attributes": {
            "primaryTable": "INPUT_REGISTERS",
            "startingAddress": "0",
            "rawType": "INT16"
          },
          "properties": {
            "value": {
              "type": "Int16",
              "readWrite": "R",
              "defaultValue": "0",
              "scale": 0.1,
              "units": "°C",
              "minimum": -400,
              "maximum": 1200
            }
          }
        }
      ]
    }
  }
}
```

### 2. 启动 device

device 负责：
- 加载物模型（Thing Model）
- 初始化 Device Runtime
- 连接 driver
- 定期轮询读取数据
- 更新属性值
- 运行规则评估
- 触发事件并通过 MQTT/Kafka 发布

```bash
# 启动 device-meter（另一个终端）
cd /root/source/rust/ithings/target/debug
./device-meter -c ../../devices/meter/examples/meter-config-integration.json
```

**示例配置** `devices/meter/examples/meter-config-integration.json`：

```json
{
  "device_name": "modbus-meter-1",
  "device_type": "modbus-meter",
  "poll_interval_ms": 1000,
  "driver": {
    "enabled": true,
    "server_address": "tcp://localhost:5555"
  },
  "custom": {
    "thing_model": {
      "model_id": "modbus-temperature-sensor.1.0",
      "model_version": "1.0",
      "device_type": "temperature-sensor",
      "manufacturer": "EdgeX",
      "description": "Modbus temperature and humidity sensor",
      "properties": [
        {
          "identifier": "temperature",
          "name": "Temperature",
          "type": "float",
          "unit": "°C",
          "access": "R",
          "range": [-40, 120],
          "default_value": 0,
          "description": "Current temperature reading"
        }
      ],
      "events": [
        {
          "identifier": "high_temperature",
          "name": "High Temperature",
          "level": "WARN",
          "output_params": [
            {
              "identifier": "temperature",
              "name": "Current Temperature",
              "type": "float"
            }
          ],
          "description": "Triggered when temperature exceeds 30°C"
        }
      ],
      "rules": [
        {
          "identifier": "high_temp_alarm",
          "name": "High Temperature Alarm",
          "conditions": [
            {
              "property_identifier": "temperature",
              "operator": ">",
              "value": 30
            }
          ],
          "condition_logic": "and",
          "actions": [
            {
              "trigger_event": {
                "event_identifier": "high_temperature",
                "data": {
                  "temperature": "{{temperature}}"
                }
              }
            }
          ],
          "enabled": true
        }
      ]
    }
  }
}
```

## 运行时序

```
device 启动
  └─> 加载物模型
      └─> 初始化 DeviceRuntime
          └─> 设置默认属性值
          └─> 连接 driver (ZMQ)
          └─> 启动轮询循环

每次轮询：
  device
  └─> 读取所有可读属性
      └─> ZMQ 请求 → driver
          driver
          └─> 根据 Device Profile 生成 Modbus 请求
              └─> 和物理设备通信
                  └─> 解析数据 → 返回 DataPoint[]
  device
  └─> 更新属性值
      └─> 规则评估
          └─> 满足条件 → 执行动作
              ├─> set_property: 设置属性
              ├─> trigger_event: 发布事件
              ├─> call_service: 调用服务
              └─> log: 输出日志
```

## 物模型配置

### 属性 (Property)

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `identifier` | string | 是 | 唯一标识符 |
| `name` | string | 是 | 名称 |
| `type` | string | 是 | 数据类型：int/float/bool/string/enum/array/object |
| `unit` | string | 否 | 单位 |
| `access` | string | 是 | R/RW/W |
| `range` | [min, max] | 否 | 取值范围 |
| `default_value` | any | 否 | 默认值 |
| `description` | string | 否 | 描述 |

### 事件 (Event)

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `identifier` | string | 是 | 唯一标识符 |
| `name` | string | 是 | 名称 |
| `level` | string | 是 | Info/Warn/Error/Fatal |
| `output_params` | array | 否 | 输出参数 |
| `description` | string | 否 | 描述 |

### 规则 (Rule)

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `identifier` | string | 是 | 唯一标识符 |
| `name` | string | 是 | 名称 |
| `conditions` | array | 是 | 条件列表 |
| `condition_logic` | string | 否 | and/or，默认 and |
| `actions` | array | 是 | 满足条件时执行的动作 |
| `enabled` | bool | 否 | 是否启用，默认 true |

**条件** (RuleCondition):
| 字段 | 说明 |
|------|------|
| `property_identifier` | 属性标识符 |
| `operator` | 运算符：`==` `!=` `>` `>=` `<` `<=` `contains` `matches` |
| `value` | 比较值 |

**动作** (RuleAction):
| 类型 | 说明 |
|------|------|
| `set_property` | 设置属性值: `{ "identifier": "...", "value": ... }` |
| `trigger_event` | 触发事件: `{ "event_identifier": "...", "data": {...} }` |
| `call_service` | 调用服务 |
| `log` | 记录日志 |

## 状态机配置

```json
{
  "state_machine": {
    "identifier": "device_state",
    "name": "Device State Machine",
    "states": [
      {
        "identifier": "init",
        "name": "Initializing",
        "is_initial": true
      },
      {
        "identifier": "running",
        "name": "Running",
        "on_enter": ["start_polling"]
      },
      {
        "identifier": "error",
        "name": "Error",
        "on_enter": ["stop_polling", "trigger_error_event"]
      }
    ],
    "transitions": [
      {
        "from_state": "init",
        "to_state": "running",
        "trigger": "init_complete"
      },
      {
        "from_state": "running",
        "to_state": "error",
        "trigger": "communication_error"
      },
      {
        "from_state": "error",
        "to_state": "running",
        "trigger": "reconnect"
      }
    ]
  }
}
```

## 数据流程

```
物理设备
    │
    ▼
Modbus 帧
    │
    ▼
modbus-driver 解析
    │
    ▼
Vec<DataPoint>
    │
    ▼  ZMQ 网络
device-meter
    │
    ▼
DeviceRuntime 更新属性
    │
    ▼
规则评估
    │
    ▼ 满足条件
触发事件 → RemotePublisher (MQTT/Kafka) → 远程平台
    │
    ▼
业务平台通过 RemoteSubscriber 发送服务调用 → device → 执行 → 返回结果
```

## 环境变量配置

device 支持从环境变量读取配置（前缀 `DEVICE_`）：

```bash
export DEVICE_DEVICE_NAME=my-meter
export DEVICE_DEVICE_TYPE=electricity-meter
export DEVICE_POLL_INTERVAL_MS=1000
export DEVICE_DRIVER_ENABLED=true
export DEVICE_DRIVER_SERVER_ADDRESS=tcp://localhost:5555
./device-meter
```

driver 支持从环境变量读取配置（前缀 `DRIVER_`）：

```bash
export DRIVER_DRIVER_NAME=modbus-driver
export DRIVER_DRIVER_TYPE=modbus
export DRIVER_ZMQ_ENABLED=true
export DRIVER_ZMQ_PUBLISHER_ADDRESS=tcp://*:5555
./modbus-driver
```

## 部署

### 单机部署

```
modbus-driver ←→ device-meter ←→ MQTT/Kafka
```

### 分布式部署

```
[server] modbus-driver (physical network)
    │
    ▼ ZMQ
[gateway] device-meter (thing model, rules, events)
    │
    ▼ MQTT/Kafka
[cloud] business logic
```

## 故障排查

### 问题：`missing field 'xxx'`

**原因**：JSON 配置缺少必填字段，或者 serde 反序列化失败

**解决**：
- 检查 JSON 语法是否正确
- 对照本文档示例检查必填字段
- 如果使用部分配置，确保可选字段已经添加了默认值

### 问题：`Invalid argument` when starting device

**原因**：尝试连接 ZMQ 失败，通常是因为 driver 没有启动

**解决**：
- 先启动 driver，再启动 device
- 检查 driver 地址配置是否正确

### 问题：`Cannot block the current thread from within a runtime`

**原因**：已修复，这是旧版本 bug，重新编译就好了

### 问题：没有输出日志/日志级别不对

**解决**：
- 启动时通过 `-l debug` 设置日志级别
- 默认是 info 级别，debug 会输出更多信息
