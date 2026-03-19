# Modbus Driver

Modbus TCP/RTU 驱动程序，用于工业设备数据采集。

## 功能特性

- ✅ Modbus TCP 协议支持
- ✅ 支持多种数据类型（Coils、Discrete Inputs、Input Registers、Holding Registers）
- ✅ 自动重连机制
- ✅ 长连接/短连接模式
- ✅ EdgeX Foundry 设备配置文件兼容
- ✅ ZeroMQ 数据发布
- ✅ 运行时动态日志级别调整
- ✅ 数据类型自动转换（scale、offset）

## 快速开始

### 编译

```bash
cd /root/source/rust/ithings/drivers
cargo build -p driver-modbus --release
```

### 运行

```bash
./target/release/modbus-driver -c /etc/app/modbus.json
```

### 命令行参数

```bash
modbus-driver [OPTIONS]

选项:
  -c, --configfile <FILE>    配置文件路径 [默认: config.json]
  -l, --loglevel <LEVEL>     日志级别 (debug, info, warn, error) [默认: info]
  -h, --help                 显示帮助信息
  -V, --version              显示版本信息
```

## 配置文件

配置文件使用 JSON 格式，包含驱动配置和设备配置文件（Device Profile）。

### 完整配置示例

```json
{
  "driver_name": "modbus-driver",
  "driver_version": "0.1.0",
  "publish_endpoint": "tcp://*:5555",
  "custom": {
    "host": "localhost",
    "port": 502,
    "slave_id": 1,
    "connectionMode": "long",
    "profile": {
      "apiVersion": "v2",
      "name": "Modbus-Temperature-Sensor",
      "manufacturer": "EdgeX",
      "model": "MB-TS100",
      "labels": ["modbus", "temperature", "sensor"],
      "description": "Modbus TCP temperature sensor device profile",
      "deviceResources": [
        {
          "name": "Temperature",
          "isHidden": false,
          "description": "Current temperature reading (0.1°C resolution)",
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
              "scale": "0.1",
              "units": "°C",
              "minimum": "-400",
              "maximum": "1200"
            }
          }
        },
        {
          "name": "Humidity",
          "isHidden": false,
          "description": "Current humidity reading (0.1% resolution)",
          "attributes": {
            "primaryTable": "INPUT_REGISTERS",
            "startingAddress": "1",
            "rawType": "UINT16"
          },
          "properties": {
            "value": {
              "type": "UInt16",
              "readWrite": "R",
              "defaultValue": "0",
              "scale": "0.1",
              "units": "%RH",
              "minimum": "0",
              "maximum": "1000"
            }
          }
        },
        {
          "name": "RelayStatus",
          "isHidden": false,
          "description": "Relay control register",
          "attributes": {
            "primaryTable": "COILS",
            "startingAddress": "0"
          },
          "properties": {
            "value": {
              "type": "Bool",
              "readWrite": "RW",
              "defaultValue": "false"
            }
          }
        }
      ]
    }
  }
}
```

### 配置说明

#### 基础配置

| 字段 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| `driver_name` | String | 驱动名称 | - |
| `driver_version` | String | 驱动版本 | - |
| `publish_endpoint` | String | ZeroMQ 发布端点 | `tcp://*:5555` |

#### Modbus 连接配置

| 字段 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| `host` | String | Modbus 服务器地址 | `localhost` |
| `port` | Number | Modbus 服务器端口 | `502` |
| `slave_id` | Number | 从站 ID | `1` |
| `connectionMode` | String | 连接模式：`long`（长连接）或 `short`（短连接） | `long` |

#### 设备配置文件（Device Profile）

设备配置文件遵循 EdgeX Foundry 标准，包含设备资源定义：

**设备资源属性（attributes）：**

| 字段 | 说明 | 可选值 |
|------|------|--------|
| `primaryTable` | Modbus 数据区 | `COILS`, `DISCRETE_INPUTS`, `INPUT_REGISTERS`, `HOLDING_REGISTERS` |
| `startingAddress` | 起始地址（字符串格式） | "0", "1", ... |
| `rawType` | 原始数据类型 | `INT16`, `UINT16`, `INT32`, `UINT32`, `FLOAT32`, `FLOAT64` |
| `count` | 寄存器数量（可选） | - |

**数据属性（properties）：**

| 字段 | 说明 | 示例 |
|------|------|------|
| `type` | 数据类型 | `Int16`, `UInt16`, `Float32`, `Bool` |
| `readWrite` | 读写权限 | `R`（只读）, `W`（只写）, `RW`（读写） |
| `defaultValue` | 默认值 | `"0"`, `"false"` |
| `scale` | 缩放因子 | `"0.1"` |
| `offset` | 偏移量 | `"0"` |
| `units` | 单位 | `"°C"`, `"%RH"` |
| `minimum` | 最小值 | `"-400"` |
| `maximum` | 最大值 | `"1200"` |

## 连接模式

### 长连接模式（Long-Lived）

- 保持 TCP 连接一直打开
- 连接成功后复用同一个连接
- 只有发生错误/超时才断开并重连
- **适用场景**：频繁采集，网络稳定

### 短连接模式（Short-Lived）

- 每次读取都重新建立 TCP 连接
- 读取完成后立即关闭连接
- **适用场景**：不频繁采集，节省资源

配置方式：

```json
{
  "custom": {
    "connectionMode": "short"
  }
}
```

## 日志级别

### 启动时指定

```bash
# 使用 debug 级别（详细日志）
./modbus-driver -c /etc/app/modbus.json -l debug

# 使用 info 级别（默认，简洁日志）
./modbus-driver -c /etc/app/modbus.json -l info

# 使用 warn 级别（只显示警告和错误）
./modbus-driver -c /etc/app/modbus.json -l warn

# 使用 error 级别（只显示错误）
./modbus-driver -c /etc/app/modbus.json -l error
```

### 运行时动态调整（Linux/Unix）

驱动运行时可以通过 Unix 信号动态修改日志级别，无需重启：

```bash
# 查找进程 PID
ps aux | grep modbus-driver

# 切换到 debug 级别（详细日志）
kill -SIGUSR1 <PID>

# 切换到 info 级别（简洁日志）
kill -SIGUSR2 <PID>
```

或者一行命令：

```bash
# 切换到 debug
kill -SIGUSR1 $(pgrep modbus-driver)

# 切换到 info
kill -SIGUSR2 $(pgrep modbus-driver)
```

### 日志级别说明

| 级别 | 说明 | 显示内容 |
|------|------|----------|
| `debug` | 详细调试信息 | 所有日志，包括每次轮询、连接过程、资源检查等 |
| `info` | 重要信息（默认） | 连接成功、数据点值、读取完成、错误等 |
| `warn` | 警告信息 | 警告和错误 |
| `error` | 错误信息 | 仅错误 |

## 数据采集

### 采集流程

1. 启动驱动，加载配置文件
2. 初始化 Modbus 连接
3. 每秒轮询一次所有可读资源
4. 读取数据并应用转换（scale、offset）
5. 通过 ZeroMQ 发布数据

### 数据输出示例

```
INFO driver_modbus::driver: Profile loaded: Modbus-Temperature-Sensor, 3 device resources
INFO modbus_driver: Driver initialized, starting polling loop with long-lived connection mode
INFO modbus_driver: Starting polling loop with interval 1000ms
INFO driver_modbus::driver: Connected to Modbus server at localhost:502 successfully
INFO driver_modbus::driver: Temperature = Float64(25.5) [Good]
INFO driver_modbus::driver: Humidity = Float64(45.0) [Good]
INFO driver_modbus::driver: RelayStatus = Bool(true) [Good]
INFO driver_modbus::driver: Read 3 data points
```

### 数据质量

每个数据点都包含质量标识：

- **Good**: 数据读取成功
- **Bad**: 数据读取失败

## 自动重连

驱动内置自动重连机制：

1. **连接失败**：每秒自动重试连接
2. **连接超时**：10 秒超时后自动重试
3. **读取错误**：断开连接，下次轮询自动重连
4. **网络中断**：检测到错误后自动重连

驱动不会因为连接失败而退出，会持续重试直到连接成功。

## 数据类型转换

驱动支持自动数据类型转换：

### 缩放（Scale）

```json
{
  "scale": "0.1"
}
```

实际值 = 原始值 × scale

示例：原始值 `255`，scale `0.1` → 实际值 `25.5`

### 偏移（Offset）

```json
{
  "offset": "-40"
}
```

实际值 = 原始值 + offset

示例：原始值 `255`，offset `-40` → 实际值 `215`

### 组合转换

```json
{
  "scale": "0.1",
  "offset": "-40"
}
```

实际值 = (原始值 × scale) + offset

示例：原始值 `255`，scale `0.1`，offset `-40` → 实际值 `(255 × 0.1) + (-40) = -14.5`

## EdgeX 兼容性

驱动完全兼容 EdgeX Foundry 设备配置文件格式：

- ✅ 支持 `apiVersion` 和 `api_version` 两种命名方式
- ✅ 支持 `deviceResources` 和 `device_resources` 两种命名方式
- ✅ 支持 `primaryTable` 和 `primary_table` 两种命名方式
- ✅ 支持 `startingAddress` 和 `starting_address` 两种命名方式
- ✅ 支持 `rawType` 和 `raw_type` 两种命名方式
- ✅ 支持 `Uint16` 和 `UInt16` 等多种类型命名方式
- ✅ 支持字符串格式的数值（如 `"1200"` 自动转换为 `1200.0`）

## Docker 部署

### 构建镜像

```bash
cd /root/source/rust/ithings/drivers/modbus
docker build -t modbus-driver:latest .
```

### 运行容器

```bash
docker run -d \
  --name modbus-driver \
  -v /etc/app/modbus.json:/etc/app/modbus.json:ro \
  -p 5555:5555 \
  modbus-driver:latest
```

### Kubernetes 部署

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: modbus-driver
spec:
  replicas: 1
  selector:
    matchLabels:
      app: modbus-driver
  template:
    metadata:
      labels:
        app: modbus-driver
    spec:
      containers:
      - name: modbus-driver
        image: modbus-driver:latest
        args:
        - "-c"
        - "/etc/app/modbus.json"
        - "-l"
        - "info"
        volumeMounts:
        - name: config
          mountPath: /etc/app
          readOnly: true
      volumes:
      - name: config
        configMap:
          name: modbus-driver-config
```

## 故障排查

### 连接失败

```
ERROR driver_modbus::driver: Failed to connect to Modbus server: Connection refused
```

**解决方案**：
1. 检查 Modbus 服务器是否运行
2. 检查 IP 和端口是否正确
3. 检查防火墙设置

### 配置文件错误

```
ERROR modbus_driver: Failed to parse device profile: missing field `apiVersion`
```

**解决方案**：
1. 检查 JSON 格式是否正确
2. 检查必填字段是否存在
3. 使用 JSON 格式化工具验证

### 数据读取失败

```
ERROR driver_modbus::driver: Failed to read resource Temperature: Invalid data
```

**解决方案**：
1. 检查寄存器地址是否正确
2. 检查数据类型是否匹配
3. 检查从站 ID 是否正确

## 开发

### 项目结构

```
modbus/
├── Cargo.toml          # 项目配置
├── README.md           # 本文档
└── src/
    ├── lib.rs          # 库入口
    ├── driver.rs       # Modbus 驱动实现
    ├── config.rs       # 配置解析
    └── main.rs         # 主程序入口
```

### 依赖

- `tokio`: 异步运行时
- `tokio-modbus`: Modbus 协议实现
- `tracing`: 日志系统
- `serde`: 序列化
- `driver-core`: 驱动核心库

## 许可证

MIT License

## 联系方式

iThings Team
