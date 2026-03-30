# 如何实现协议驱动 (Protocol Driver)

本文档说明如何参考 Modbus Driver 实现其他协议驱动（如 BACnet、OPC UA 等）。

## 架构说明

```
┌─────────────────────────────────────────────────────────┐
│                  Protocol Driver (xxx-driver)            │
├─────────────────────────────────────────────────────────┤
│  MultiDeviceDriver<XXXDriver>                           │
│    └── DeviceInstanceManager<XXXDriver>                 │
│         └── Vec<DeviceInstance<XXXDriver>>              │
│              └── XXXDriver (impl Driver)                │
│                   ├── BaseDriver                        │
│                   ├── ProtocolContext (连接管理)        │
│                   ├── Vec<DeviceProfile> (设备配置)     │
│                   └── ProtocolAttributes (协议属性)     │
└─────────────────────────────────────────────────────────┘
         │                                    │
         │ ZMQ (发布/订阅)                    │ 协议通信
         ▼                                    ▼
    DeviceManager                    物理设备/PLC
    (设备应用层)
```

## 核心组件

### 1. Driver Trait

所有协议驱动必须实现 `driver_core::driver::Driver` trait：

```rust
#[async_trait]
pub trait Driver: Send + Sync {
    fn metadata(&self) -> DriverMetadata;
    fn device_name(&self) -> Option<&str>;
    async fn initialize(&mut self, config: DriverConfig) -> Result<()>;
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn read(&self) -> Result<Vec<DataPoint>>;
    async fn write(&self, data_point: &DataPoint) -> Result<()>;
    async fn status(&self) -> DriverStatus;
}
```

### 2. 协议特定配置

定义协议特定的属性解析器：

```rust
// config.rs
#[derive(Debug, Clone)]
pub struct ProtocolAttributes {
    pub object_type: ObjectType,      // 协议对象类型
    pub object_identifier: u32,       // 对象标识符
    pub property_id: PropertyId,      // 属性标识符
    pub data_type: DataType,          // 数据类型
}

pub struct ProtocolAttributeParser;

impl ProtocolAttributeParser {
    pub fn parse(attributes: &HashMap<String, serde_json::Value>) 
        -> Result<ProtocolAttributes, String> {
        // 解析协议特定的属性
        let object_type = Self::parse_object_type(
            attributes.get("objectType")
        )?;
        let object_identifier = Self::parse_identifier(
            attributes.get("objectIdentifier")
        )?;
        let property_id = Self::parse_property_id(
            attributes.get("propertyId")
        )?;
        let data_type = Self::parse_data_type(
            attributes.get("dataType")
        )?;
        
        Ok(ProtocolAttributes {
            object_type,
            object_identifier,
            property_id,
            data_type,
        })
    }
}
```

### 3. 连接管理

管理协议连接：

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

pub type ProtocolContext = Arc<Mutex<Option<ProtocolClient>>>;

pub fn create_context() -> ProtocolContext {
    Arc::new(Mutex::new(None))
}

impl XXXDriver {
    async fn ensure_connected(&self) -> Result<()> {
        let mut ctx_lock = self.context.lock().await;
        
        if ctx_lock.is_none() {
            let client = ProtocolClient::connect(&self.host, self.port).await?;
            *ctx_lock = Some(client);
            tracing::info!("Connected to protocol server");
        }
        Ok(())
    }
}
```

### 4. 批量读取优化

将连续的读取请求合并为批量操作：

```rust
struct BatchRequest<'a> {
    object_type: ObjectType,
    start_id: u32,
    count: u32,
    resources: Vec<(&'a DeviceResource, ProtocolAttributes)>,
}

fn group_continuous_requests(
    mut resources: Vec<(&DeviceResource, ProtocolAttributes)>
) -> Vec<BatchRequest> {
    if resources.is_empty() {
        return Vec::new();
    }

    resources.sort_by_key(|(_, attrs)| attrs.object_identifier);

    let mut batches = Vec::new();
    let mut current_batch: Option<(u32, u32, Vec<_>)> = None;

    for (resource, attrs) in resources.drain(..) {
        match current_batch {
            None => {
                current_batch = Some((attrs.object_identifier, 
                                     attrs.object_identifier, 
                                     vec![(resource, attrs)]));
            }
            Some((batch_start, batch_end, mut batch_resources)) => {
                if attrs.object_identifier == batch_end + 1 {
                    let new_end = attrs.object_identifier;
                    batch_resources.push((resource, attrs));
                    current_batch = Some((batch_start, new_end, batch_resources));
                } else {
                    let count = batch_end - batch_start + 1;
                    batches.push(BatchRequest {
                        object_type: attrs.object_type,
                        start_id: batch_start,
                        count,
                        resources: batch_resources,
                    });
                    current_batch = Some((attrs.object_identifier, 
                                         attrs.object_identifier, 
                                         vec![(resource, attrs)]));
                }
            }
        }
    }

    if let Some((batch_start, batch_end, batch_resources)) = current_batch {
        let count = batch_end - batch_start + 1;
        batches.push(BatchRequest {
            object_type: batch_resources[0].1.object_type,
            start_id: batch_start,
            count,
            resources: batch_resources,
        });
    }

    batches
}
```

### 5. 数据转换

将协议原始数据转换为标准格式：

```rust
fn convert_raw_data(
    raw_data: &RawBatchData,
    base_id: u32,
    attrs: ProtocolAttributes,
    properties: &ValueProperties,
) -> Result<DataValue> {
    let offset = (attrs.object_identifier - base_id) as usize;
    
    match raw_data {
        RawBatchData::Values(values) => {
            let raw_value = &values[offset];
            DataValueConverter::convert(raw_value, properties)
        }
    }
}
```

## 完整实现示例

### driver.rs

```rust
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use driver_core::{
    config::DriverConfig,
    driver::{BaseDriver, Driver},
    types::{DataPoint, DataValue, DeviceProfile, Quality},
};
use std::collections::HashMap;
use tokio::sync::MutexGuard;

use crate::config::{ProtocolContext, ProtocolAttributes, ProtocolAttributeParser};

#[derive(Debug)]
pub struct XXXDriver {
    base: BaseDriver,
    profiles: Vec<DeviceProfile>,
    context: ProtocolContext,
    host: String,
    port: u16,
}

impl XXXDriver {
    pub fn new() -> Self {
        Self {
            base: BaseDriver::new(),
            profiles: Vec::new(),
            context: crate::config::create_context(),
            host: "localhost".to_string(),
            port: 47808, // BACnet default port
        }
    }

    pub fn add_profile(&mut self, profile: DeviceProfile) {
        self.profiles.push(profile);
    }

    async fn ensure_connected(&self) -> Result<()> {
        let mut ctx_lock = self.context.lock().await;
        
        if ctx_lock.is_none() {
            let client = ProtocolClient::connect(&self.host, self.port).await?;
            *ctx_lock = Some(client);
            tracing::info!("Connected to protocol server at {}:{}", self.host, self.port);
        }
        Ok(())
    }

    async fn read_batch(
        &self,
        object_type: ObjectType,
        start_id: u32,
        count: u32,
    ) -> Result<RawBatchData> {
        self.ensure_connected().await?;
        
        let mut ctx_lock = self.context.lock().await;
        let client = ctx_lock.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not connected"))?;

        let data = client.read_objects(object_type, start_id, count).await?;
        Ok(RawBatchData::Values(data))
    }
}

#[derive(Debug)]
enum RawBatchData {
    Values(Vec<ProtocolValue>),
}

unsafe impl Send for XXXDriver {}
unsafe impl Sync for XXXDriver {}

#[async_trait]
impl Driver for XXXDriver {
    fn metadata(&self) -> driver_core::types::DriverMetadata {
        driver_core::types::DriverMetadata {
            name: "xxx-driver".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: "xxx".to_string(),
            description: "XXX protocol driver".to_string(),
            author: "iThings Team".to_string(),
            tags: vec!["xxx".to_string()],
        }
    }

    async fn initialize(&mut self, config: DriverConfig) -> Result<()> {
        tracing::debug!("Initializing XXX driver");
        
        // 加载设备配置
        if let Some(profile_json) = config.custom.get("profile") {
            let profile: DeviceProfile = serde_json::from_value(profile_json.clone())?;
            self.add_profile(profile);
        }
        
        if let Some(profiles_json) = config.custom.get("profiles") {
            if let serde_json::Value::Array(profiles_array) = profiles_json {
                for profile_json in profiles_array {
                    let profile: DeviceProfile = serde_json::from_value(profile_json.clone())?;
                    self.add_profile(profile);
                }
            }
        }

        // 加载连接配置
        if let Some(host) = config.custom.get("host").and_then(|v| v.as_str()) {
            self.host = host.to_string();
        }
        if let Some(port) = config.custom.get("port").and_then(|v| v.as_u64()) {
            self.port = port as u16;
        }

        self.base.initialize(config).await?;
        Ok(())
    }

    async fn connect(&mut self) -> Result<()> {
        self.ensure_connected().await?;
        self.base.set_status(driver_core::types::DriverStatus::Running).await;
        tracing::info!("XXX driver connected");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        let mut ctx_lock = self.context.lock().await;
        *ctx_lock = None;
        self.base.set_status(driver_core::types::DriverStatus::Stopped).await;
        Ok(())
    }

    async fn read(&self) -> Result<Vec<DataPoint>> {
        let mut data_points = Vec::new();

        for profile in &self.profiles {
            let mut grouped_requests: HashMap<ObjectType, Vec<_>> = HashMap::new();
            
            for resource in &profile.device_resources {
                if !resource.properties.value.read_write.can_read() {
                    continue;
                }

                let attrs = match ProtocolAttributeParser::parse(&resource.attributes) {
                    Ok(a) => a,
                    Err(e) => {
                        tracing::error!("Failed to parse attributes: {}", e);
                        continue;
                    }
                };

                grouped_requests.entry(attrs.object_type)
                    .or_default()
                    .push((resource, attrs));
            }

            for (object_type, resources) in grouped_requests {
                let requests = group_continuous_requests(resources);
                
                for batch in requests {
                    match self.read_batch(object_type, batch.start_id, batch.count).await {
                        Ok(raw_data) => {
                            for (resource, attrs) in batch.resources {
                                match convert_raw_data(&raw_data, batch.start_id, attrs, 
                                                      &resource.properties.value) {
                                    Ok(value) => {
                                        data_points.push(DataPoint {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            name: resource.name.clone(),
                                            value,
                                            quality: Quality::Good,
                                            timestamp: Utc::now(),
                                            metadata: HashMap::new(),
                                            units: resource.properties.value.units.clone(),
                                        });
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to convert: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Batch read failed: {}", e);
                        }
                    }
                }
            }
        }

        Ok(data_points)
    }

    async fn write(&self, data_point: &DataPoint) -> Result<()> {
        // 实现写入逻辑
        Ok(())
    }

    async fn status(&self) -> DriverStatus {
        self.base.status().await
    }
}
```

## 设备 Profile 编写指南

### Profile 结构

```json
{
  "name": "Device Name",
  "description": "Device description",
  "deviceCommands": [
    {
      "name": "command_name",
      "deviceResources": [
        {
          "name": "resource_name",
          "description": "Resource description",
          "attributes": {
            "objectType": "analogInput",
            "objectIdentifier": 1,
            "propertyId": "presentValue",
            "dataType": "float"
          },
          "properties": {
            "value": {
              "type": "Float32",
              "readWrite": "R",
              "units": "degreesCelsius"
            }
          }
        }
      ]
    }
  ]
}
```

### 属性说明

| 属性 | 类型 | 说明 | 示例 |
|------|------|------|------|
| `objectType` | string | 协议对象类型 | "analogInput", "binaryOutput" |
| `objectIdentifier` | number | 对象标识符 | 1, 2, 100 |
| `propertyId` | string | 属性标识符 | "presentValue", "statusFlags" |
| `dataType` | string | 数据类型 | "float", "int", "bool" |

### 完整示例

```json
{
  "name": "BACnet Temperature Sensor",
  "description": "BACnet temperature sensor device",
  "deviceCommands": [
    {
      "name": "read_temperature",
      "deviceResources": [
        {
          "name": "temperature",
          "description": "Current temperature",
          "attributes": {
            "objectType": "analogInput",
            "objectIdentifier": 1,
            "propertyId": "presentValue",
            "dataType": "float"
          },
          "properties": {
            "value": {
              "type": "Float32",
              "readWrite": "R",
              "units": "degreesCelsius"
            }
          }
        },
        {
          "name": "status",
          "description": "Device status",
          "attributes": {
            "objectType": "binaryInput",
            "objectIdentifier": 10,
            "propertyId": "presentValue",
            "dataType": "bool"
          },
          "properties": {
            "value": {
              "type": "Boolean",
              "readWrite": "R"
            }
          }
        }
      ]
    }
  ]
}
```

## main.rs 实现

```rust
use anyhow::Result;
use clap::Parser;
use driver_core::{DriverConfig, MultiDeviceDriver};
use driver_xxx::XXXDriver;
use tokio::signal;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use common::config::group::DeviceGroupConfig;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "config.json")]
    configfile: String,
    
    #[arg(short, long, default_value = "info")]
    loglevel: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let filter = EnvFilter::from_default_env()
        .add_directive(format!("xxx_driver={}", args.loglevel).parse().unwrap());
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    let group_config = DeviceGroupConfig::from_file(&args.configfile)?;

    let base_config = DriverConfig {
        driver_name: "xxx-driver".to_string(),
        driver_type: "xxx".to_string(),
        device_instance_id: "xxx-driver-group".to_string(),
        poll_interval_ms: 1000,
        zmq: Default::default(),
        logging: Default::default(),
        custom: Default::default(),
    };

    let mut driver = MultiDeviceDriver::<XXXDriver>::new(base_config.clone());
    driver.initialize(base_config).await?;

    for device in &group_config.devices {
        let device_config = DeviceInstanceConfig {
            device_instance_id: device.device_id.clone(),
            device_profile: None,
            custom: device.driver.custom.clone(),
            poll_interval_ms: Some(device.poll_interval_ms),
        };
        driver.handle_config_update(device_config).await?;
    }

    let shutdown = signal::ctrl_c();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                tracing::info!("Shutdown signal received");
                break;
            }
            result = driver.run_polling_loop() => {
                if let Err(e) = result {
                    tracing::error!("Polling loop error: {}", e);
                }
                break;
            }
        }
    }

    driver.device_manager_mut().stop_all().await?;
    Ok(())
}
```

## 编译和运行

```bash
# 编译
cargo build --release -p driver-xxx

# 运行
./target/release/driver-xxx --configfile config.json --loglevel info
```

## 关键要点

1. **实现 Driver trait**: 所有协议驱动必须实现完整的 Driver trait
2. **批量读取优化**: 使用批量读取提高性能
3. **连接管理**: 实现自动重连和连接池
4. **错误处理**: 确保错误不会导致驱动崩溃
5. **Profile 解析**: 支持灵活的设备配置格式
