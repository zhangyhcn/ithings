# 编译指南

## 项目结构

```
ithings/
├── Cargo.toml              # 工作区配置
├── common/                 # 公共库（设备通用组件）
│   └── Cargo.toml
├── drivers/                # 驱动项目
│   ├── Cargo.toml
│   ├── driver-core/        # 驱动核心抽象
│   └── modbus/             # Modbus 驱动实现
├── devices/                # 设备项目
│   ├── Cargo.toml
│   └── meter/              # 电表设备实现
└── icloud/
```

## 环境要求

- Rust 1.70+ 版本
- CMake（编译 rdkafka 需要）
- 其他系统依赖：
  - Ubuntu/Debian: `sudo apt install build-essential cmake libssl-dev`
  - CentOS/RHEL: `sudo yum groupinstall "Development Tools" && sudo yum install cmake openssl-devel`

## 编译整个项目

在项目根目录编译所有 crate：

```bash
cd /path/to/ithings
cargo build
```

编译 release 版本：

```bash
cargo build --release
```

## 编译单个设备

编译电表设备：

```bash
cd /path/to/ithings
cargo build -p device-meter
```

编译后可执行文件在：
- Debug: `target/debug/device-meter`
- Release: `target/release/device-meter`

## 编译单个驱动

编译 Modbus 驱动：

```bash
cd /path/to/ithings
cargo build -p driver-modbus
```

编译后可执行文件在：
- Debug: `target/debug/modbus-driver`
- Release: `target/release/modbus-driver`

## 单独编译 common 公共库

```bash
cargo build -p device-common
```

## 编译 workspace 中所有项目

```bash
cargo build --workspace
```

## 清除编译产物

```bash
cargo clean
```

## 检查编译错误

```bash
cargo check
```

检查特定 crate：

```bash
cargo check -p device-meter
cargo check -p driver-modbus
```

## 运行测试

```bash
cargo test
```

## 添加新设备

1. 在 `devices/` 目录下创建新目录：
```bash
mkdir -p devices/your-device
```

2. 创建 `devices/your-device/Cargo.toml`：
```toml
[package]
name = "device-your-device"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
device-common.workspace = true
driver-core.workspace = true
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
async-trait.workspace = true
uuid.workspace = true
config.workspace = true
clap.workspace = true
```

3. 创建 `src/main.rs` 和 `src/your_driver.rs`，实现 `Driver` trait

## 添加新驱动

1. 在 `drivers/` 目录下创建新目录：
```bash
mkdir -p drivers/your-driver
```

2. 创建 `drivers/your-driver/Cargo.toml`，参考 `modbus/Cargo.toml` 格式

3. 实现 `Driver` trait

## 配置示例

### 设备配置示例（MQTT + ZMQ 驱动通信）

```json
{
  "device_name": "my-meter",
  "device_type": "electricity-meter",
  "poll_interval_ms": 1000,
  "zmq": {
    "enabled": true,
    "subscriber_address": "tcp://localhost:5556",
    "write_topic": "driver/write"
  },
  "mqtt": {
    "enabled": true,
    "broker_address": "tcp://localhost:1883",
    "client_id": "meter-client",
    "topic_prefix": "devices",
    "qos": 1
  },
  "kafka": {
    "enabled": false,
    "brokers": "localhost:9092",
    "topic_prefix": "devices",
    "write_topic": "driver-write",
    "consumer_group": "device-group"
  },
  "driver": {
    "enabled": true,
    "server_address": "tcp://localhost:7777"
  },
  "logging": {
    "level": "info",
    "format": "json"
  },
  "custom": {
    "publisher_type": "mqtt",
    "subscriber_type": "zmq",
    "driver_comm_type": "zmq",
    "resources": ["voltage", "current", "power"]
  }
}
```

### 设备配置示例（Kafka + ZMQ 驱动通信）

```json
{
  "device_name": "my-meter",
  "device_type": "electricity-meter",
  "poll_interval_ms": 1000,
  "zmq": {
    "enabled": true,
    "subscriber_address": "tcp://localhost:5556",
    "write_topic": "driver/write"
  },
  "mqtt": {
    "enabled": false
  },
  "kafka": {
    "enabled": true,
    "brokers": "localhost:9092",
    "topic_prefix": "devices",
    "write_topic": "driver-write",
    "consumer_group": "device-group"
  },
  "driver": {
    "enabled": true,
    "server_address": "tcp://localhost:7777"
  },
  "logging": {
    "level": "info",
    "format": "json"
  },
  "custom": {
    "publisher_type": "kafka",
    "subscriber_type": "kafka",
    "driver_comm_type": "zmq",
    "resources": ["voltage", "current", "power"]
  }
}
```

## 运行示例

### 运行设备

```bash
# Debug 版本
./target/debug/device-meter -c /path/to/config.json

# Release 版本
./target/release/device-meter -c /path/to/config.json
```

### 运行驱动

```bash
# Debug 版本
./target/debug/modbus-driver -c /path/to/modbus.json

# Release 版本
./target/release/modbus-driver -c /path/to/modbus.json
```

## 常见问题

### 1. 编译 rdkafka 失败

**解决方法：**
```bash
# 安装依赖
sudo apt install cmake libssl-dev

# 清除并重试
cargo clean
cargo build
```

### 2. 找不到 device-common

**解决方法：**
确保在项目根目录编译，或者使用 `cargo build --workspace`

### 3. 依赖版本冲突

**解决方法：**
更新 Cargo.lock：
```bash
cargo update
```

### 4. linker 错误

**解决方法：**
```bash
cargo clean
cargo build
```

## 架构说明

- **driver**: 负责和硬件通信，读取数据，写入数据
- **device**: 负责物模型处理，将数据发布到 MQTT/Kafka，接收写请求发送给 driver
- **common**: 公共抽象，包括：
  - 配置定义
  - 发布订阅抽象接口（支持 MQTT/Kafka）
  - driver-device 通信抽象（支持 ZMQ/InMemory/TcpSocket）
  - 类型定义
