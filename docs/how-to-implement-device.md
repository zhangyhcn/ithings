# 如何实现一个新的设备驱动

本文档说明如何使用 `DeviceBuilder` 快速实现一个新的设备驱动。

## 架构说明

```
┌─────────────────┐         ZMQ          ┌─────────────────┐
│  device-xxx     │ ◄──────────────────► │  xxx-driver     │
│  (设备应用层)    │      数据/配置        │  (硬件驱动层)    │
│                 │                      │                 │
│  DeviceBuilder  │                      │  Driver trait   │
│  DeviceRuntime  │                      │  MultiDeviceDriver │
└─────────────────┘                      └─────────────────┘
        │                                        │
        │ MQTT                                   │ 硬件协议
        ▼                                        ▼
   云平台/消息队列                           物理设备/PLC
```

**关键区别**：
- **设备应用层** (device-xxx): 使用 `DeviceBuilder` + `DeviceRuntime`，不实现 `Driver` trait
- **硬件驱动层** (xxx-driver): 实现 `Driver` trait，直接操作硬件

## 目录结构

```
devices/
└── my-device/
    ├── Cargo.toml
    └── src/
        ├── main.rs      # 入口程序
        ├── device.rs    # 设备实现
        └── lib.rs       # 库入口
```

## 1. 创建 Cargo.toml

```toml
[package]
name = "device-mydevice"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "device-mydevice"
path = "src/main.rs"

[dependencies]
common = { path = "../../common" }
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
clap = { version = "4", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
uuid = { version = "1", features = ["v4"] }
```

## 2. 实现 device.rs

```rust
use common::{
    DeviceConfig,
    PropertyValue, ServiceParams, ServiceResult,
    DeviceBuilder,
};
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;

pub struct MyDevice {
    config: Option<DeviceConfig>,
    runtime: Option<Arc<common::DeviceRuntime>>,
}

impl Default for MyDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl MyDevice {
    pub fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }

    pub async fn initialize_with_device_config(&mut self, config: DeviceConfig) -> Result<()> {
        tracing::info!("Initializing my device: {}", config.device_name);
        
        // 从配置加载物模型
        let thing_model = DeviceBuilder::load_thing_model_from_config(&config)?;
        
        // 使用 DeviceBuilder 构建运行时
        let runtime = DeviceBuilder::new(config.clone())
            .with_thing_model(thing_model)
            .with_service("my_service", Self::my_service_handler)
            .build()
            .await?;

        self.runtime = Some(runtime);
        self.config = Some(config);
        
        tracing::info!("My device initialized successfully");
        Ok(())
    }

    pub fn get_runtime(&self) -> Option<&Arc<common::DeviceRuntime>> {
        self.runtime.as_ref()
    }

    pub fn poll_interval_ms(&self) -> u64 {
        self.config.as_ref().map(|c| c.poll_interval_ms).unwrap_or(1000)
    }

    // 服务处理函数示例
    pub fn my_service_handler(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
        tracing::info!("my_service called: msg_id={}, service_id={}", msg_id, service_id);
        
        let param_value = params.params.get("param1")
            .and_then(|v| v.value.as_str())
            .unwrap_or("default");

        let mut result_data = HashMap::new();
        result_data.insert(
            "result".to_string(),
            PropertyValue::new("result", serde_json::Value::String(format!("processed: {}", param_value))),
        );
        result_data.insert(
            "timestamp".to_string(),
            PropertyValue::new("timestamp", serde_json::Value::String(chrono::Utc::now().to_rfc3339())),
        );

        ServiceResult::success(msg_id, service_id, result_data)
    }
}
```

## 3. 实现 main.rs

```rust
use anyhow::Result;
use clap::Parser;
use common::DeviceManager;
use device_mydevice::MyDevice;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "my-device")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "My custom device driver", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.json", help = "Configuration file path")]
    configfile: String,
    
    #[arg(short, long, default_value = "info", help = "Log level")]
    loglevel: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let filter = EnvFilter::from_default_env()
        .add_directive(format!("device_mydevice={}", args.loglevel).parse().unwrap())
        .add_directive(format!("common={}", args.loglevel).parse().unwrap());
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Loading devices from config: {}", args.configfile);
    
    let mut manager = DeviceManager::new();
    manager.register_service("my_service", MyDevice::my_service_handler);
    manager.load_from_file(&args.configfile).await?;
    manager.initialize_all().await?;
    
    tracing::info!("Initialized {} devices total", manager.len());
    
    manager.send_driver_config().await?;
    tracing::info!("Sent all driver configurations to drivers");
    
    let default_report_interval = 5000;
    manager.start_reporting_loop(default_report_interval).await;
    
    Ok(())
}
```

## 4. 实现 lib.rs

```rust
pub mod device;

pub use device::MyDevice;
```

## 5. 配置文件示例 (config.json)

```json
{
  "tenant_id": "your-tenant-id",
  "org_id": "your-org-id",
  "site_id": "your-site-id",
  "namespace_id": "your-namespace-id",
  "remote_transport": {
    "type": "mqtt",
    "broker": "tcp://localhost:1883"
  },
  "devices": [
    {
      "device_instance_id": "device-001",
      "device_name": "My Device 1",
      "device_type": "my-device-type",
      "thing_model": {
        "model_id": "my-device-model",
        "model_version": "1.0.0",
        "properties": [
          {
            "identifier": "temperature",
            "name": "温度",
            "type": "float",
            "access": "r",
            "unit": "°C"
          }
        ],
        "services": [
          {
            "identifier": "my_service",
            "name": "我的服务",
            "call_type": "async",
            "input_params": [
              {
                "identifier": "param1",
                "name": "参数1",
                "type": "string"
              }
            ],
            "output_params": [
              {
                "identifier": "result",
                "name": "结果",
                "type": "string"
              }
            ]
          }
        ],
        "events": []
      },
      "driver": {
        "driver_name": "my-driver",
        "driver_type": "my-device-type",
        "zmq": {
          "enabled": true,
          "router_address": "tcp://localhost",
          "router_sub_port": 5550,
          "router_pub_port": 5551
        }
      }
    }
  ]
}
```

## DeviceBuilder API 参考

### 构建方法

| 方法 | 说明 |
|------|------|
| `new(config)` | 创建构建器，传入设备配置 |
| `with_thing_model(model)` | 设置物模型对象 |
| `with_thing_model_from_file(path)` | 从文件加载物模型 |
| `with_thing_model_from_json(json)` | 从 JSON 加载物模型 |
| `with_service(name, handler)` | 注册服务处理函数 |
| `with_rules(rules)` | 设置规则列表 |
| `with_state_machine(sm)` | 设置状态机 |
| `with_device_id(id)` | 设置设备 ID |
| `build().await` | 构建并返回 DeviceRuntime |

### 静态方法

| 方法 | 说明 |
|------|------|
| `load_thing_model_from_config(config)` | 从配置中加载物模型 |

## 服务处理函数签名

```rust
pub type ServiceHandler = fn(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult;
```

### 示例

```rust
pub fn my_handler(msg_id: &str, service_id: &str, params: ServiceParams) -> ServiceResult {
    // 获取参数
    let value = params.params.get("param_name")
        .and_then(|v| v.value.as_str())
        .unwrap_or("default");
    
    // 处理逻辑
    // ...
    
    // 返回结果
    let mut result_data = HashMap::new();
    result_data.insert(
        "result".to_string(),
        PropertyValue::new("result", serde_json::Value::String("success".to_string())),
    );
    
    ServiceResult::success(msg_id, service_id, result_data)
}
```

## 编译和运行

```bash
# 编译
cargo build --release -p device-mydevice

# 运行
./target/release/device-mydevice --configfile config.json --loglevel info
```

## 完整示例

参考 `devices/example` 目录下的完整示例代码。
