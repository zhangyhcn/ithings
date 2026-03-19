# iThings Drivers

工业协议驱动框架，支持多种协议（Modbus、OPC UA、BACnet、MQTT、HTTP、TCP）的设备连接和数据采集。

## 目录结构

```
drivers/
├── Cargo.toml           # Workspace 配置
├── driver-core/         # 公共核心库
│   ├── src/
│   │   ├── config.rs    # 配置管理
│   │   ├── driver.rs    # 驱动 trait 定义
│   │   ├── types.rs    # 类型定义
│   │   └── publisher.rs # ZeroMQ 发布
│   └── Cargo.toml
├── modbus/              # Modbus 驱动
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── driver.rs
│   │   └── config.rs
│   ├── Cargo.toml
│   └── config.json      # 示例配置
├── opcua/               # OPC UA 驱动（待实现）
├── bacnet/              # BACnet 驱动（待实现）
├── mqtt/                # MQTT 驱动（待实现）
├── http/                # HTTP 驱动（待实现）
└── tcp/                 # TCP 驱动（待实现）
```

## 编译

```bash
# 编译所有驱动
cd drivers
cargo build

# 编译特定驱动
cargo build -p driver-modbus
```

## 启动方式

### 基本启动

```bash
# 使用默认配置 (config.json)
./target/debug/modbus-driver

# 指定配置文件
./target/debug/modbus-driver config.json

# 使用命令行参数指定配置文件
./target/debug/modbus-driver -c /path/to/config.json
```

### 启动参数

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--configfile` | `-c` | 配置文件路径 | `config.json` |
| `--loglevel` | `-l` | 日志级别 | `info` |
| `--help` | `-h` | 显示帮助信息 | - |
| `--version` | `-V` | 显示版本信息 | - |

### 使用示例

```bash
# 使用默认配置
./target/debug/modbus-driver

# 指定配置文件（短格式）
./target/debug/modbus-driver -c /etc/app/modbus.json

# 指定配置文件（长格式）
./target/debug/modbus-driver --configfile=/etc/app/modbus.json

# 指定日志级别
./target/debug/modbus-driver -l debug

# 同时指定配置和日志级别
./target/debug/modbus-driver -c /etc/app/modbus.json -l debug

# 显示帮助
./target/debug/modbus-driver -h

# 显示版本
./target/debug/modbus-driver -V
```

### 环境变量配置

除了配置文件，还可以通过环境变量覆盖配置：

```bash
export DRIVER_ZMQ_ENABLED=true
export DRIVER_ZMQ_PUBLISHER_ADDRESS=tcp://*:5555
export DRIVER_ZMQ_TOPIC=modbus/data

# 日志配置
export DRIVER_LOGGING_LEVEL=debug

# 自定义配置
export DRIVER_CUSTOM_HOST=192.168.1.100
export DRIVER_CUSTOM_PORT=502
export DRIVER_CUSTOM_SLAVE_ID=1
```

### Kubernetes 部署

结合 K8s ConfigMap 使用：

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: modbus-driver
spec:
  containers:
  - name: modbus-driver
    image: ithings/modbus-driver:latest
    args:
      - "--configfile=/etc/app/config/driver.json"
      - "--loglevel=$(LOG_LEVEL)"
    env:
      - name: LOG_LEVEL
        valueFrom:
          configMapKeyRef:
            name: app-config
            key: log.level
    volumeMounts:
      - name: config
        mountPath: /etc/app/config
        readOnly: true
  volumes:
  - name: config
    configMap:
      name: driver-config
```

## 配置文件格式

### JSON 格式

```json
{
  "driver_name": "modbus-driver",
  "driver_type": "modbus",
  "poll_interval_ms": 1000,
  "zmq": {
    "enabled": true,
    "publisher_address": "tcp://*:5555",
    "topic": "modbus/data",
    "high_water_mark": 1000
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
      "name": "Device-Profile",
      "deviceResources": [
        {
          "name": "Temperature",
          "attributes": {
            "primaryTable": "INPUT_REGISTERS",
            "startingAddress": "0",
            "rawType": "INT16"
          },
          "properties": {
            "value": {
              "type": "Int16",
              "readWrite": "R",
              "scale": "0.1",
              "units": "°C"
            }
          }
        }
      ]
    }
  }
}
```

### 配置说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `driver_name` | String | 驱动名称 |
| `driver_type` | String | 驱动类型 |
| `poll_interval_ms` | u64 | 轮询间隔(毫秒) |
| `zmq.enabled` | bool | 是否启用 ZeroMQ |
| `zmq.publisher_address` | String | ZeroMQ 发布地址 |
| `zmq.topic` | String | ZeroMQ 主题 |
| `logging.level` | String | 日志级别(debug/info/warn/error) |
| `custom.host` | String | 设备主机地址 |
| `custom.port` | u16 | 设备端口 |
| `custom.slave_id` | u8 | Modbus 从站 ID |
| `custom.profile` | Object | 设备Profile定义 |

## 设备 Profile

设备 Profile 定义了设备的资源属性，包括：

- **attributes**: 协议特有属性（Modbus 的 primaryTable、startingAddress、rawType 等）
- **properties**: 标准化属性（类型、读写权限、缩放、偏移、单位等）

支持的属性类型：
- `Bool`, `Int8`, `Int16`, `Int32`, `Int64`
- `UInt8`, `UInt16`, `UInt32`, `UInt64`
- `Float32`, `Float64`, `String`, `Binary`

支持的 Modbus 表：
- `COILS` - 读写线圈
- `DISCRETE_INPUTS` - 只读离散输入
- `HOLDING_REGISTERS` - 读写保持寄存器
- `INPUT_REGISTERS` - 只读输入寄存器
