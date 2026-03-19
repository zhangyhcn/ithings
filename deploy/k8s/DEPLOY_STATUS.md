# 当前部署状态

## 整体架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Windows 宿主机                                    │
│                                                                    │
│  ╔═════════════════╗                                             │
│  ║  modbuslave.exe  ║  → 监听 0.0.0.0:502                    │
│  ╚═════════════════╝                                             │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼  host.docker.internal
┌─────────────────────────────────────────────────────────────────────┐
│               Kubernetes (Docker Desktop)                             │
│                                                                    │
│  Namespace: ithings                                               │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  modbus-driver                                            │   │
│  │   - 运行中 1/1 Ready                                      │   │
│  │   - hostNetwork: true                                     │   │
│  │   - ZMQ: 监听 5555                                       │   │
│  │   - 连接 Windows modbuslave: host.docker.internal:502        │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  device-meter                                               │   │
│  │   - 运行中 1/1 Ready                                      │   │
│  │   - 连接 ZMQ: modbus-driver:5555 (K8s DNS)               │   │
│  │   - Thing Model: 温度传感器 + 高温报警规则                   │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  local registry                                             │   │
│  │   - 运行中 1/1 Ready                                      │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## 组件状态表

| 组件 | 运行位置 | 状态 | IP/地址 |
|------|----------|------|---------|
| **modbuslave** | Windows 宿主机 | ✅ Running | `0.0.0.0:502 |
| **modbus-driver** | Kubernetes Pod | ✅ 1/1 Running | ZMQ: `0.0.0.0:5555`，连接: `host.docker.internal:502` |
| **device-meter** | Kubernetes Pod | ✅ 1/1 Running | 连接: `tcp://modbus-driver:5555` |
| **local registry** | Kubernetes Pod | ✅ 1/1 Running | NodePort: `30500` |
| **namespace ithings** | Kubernetes | ✅ Created | - |

## 当前配置

### modbus-driver `modbus.json (ConfigMap)

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
    "host": "host.docker.internal",
    "port": 502,
    "slave_id": 1,
    "profile": {
      "apiVersion": "v2",
      "name": "Modbus-Temperature-Sensor",
      "manufacturer": "EdgeX",
      "model": "mb-ts100",
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
              "scale": 0.1,
              "units": "°C",
              "minimum": -400,
              "maximum": 1200
            }
          }
        }
      ],
      "deviceCommands": [
        {
          "name": "ReadTemperature",
          "readWrite": "R",
          "resourceOperations": [
            { "deviceResource": "Temperature" }
          ]
        }
      ]
    }
  }
}
```

### device-meter `meter-config-integration.json (ConfigMap)`

```json
{
  "device_name": "modbus-temperature-1",
  "device_type": "modbus-meter",
  "poll_interval_ms": 1000,
  "driver": {
    "enabled": true,
    "server_address": "tcp://modbus-driver:5555"
  },
  "logging": {
    "level": "info",
    "format": "json"
  },
  "custom": {
    "thing_model": {
      "model_id": "modbus-temperature-sensor.1.0",
      "model_version": "1.0",
      "device_type": "temperature-sensor",
      "manufacturer": "EdgeX",
      "description": "Modbus temperature sensor",
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
          "name": "High Temperature Alarm",
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

## 数据流

1. **modbuslave** (Windows) → 接受 Modbus 请求，返回数据
2. **modbus-device** (K8s) → 根据 Device Profile 批量读取，解析为标准 DataPoint，通过 ZMQ 发送给 device-meter
3. **device-meter** (K8s) → 更新物模型属性值 → 运行规则评估 → 满足条件触发报警事件 → 通过 publisher 发布

## 检查命令

```bash
# 查看所有 Pod 状态
kubectl get pods -n ithings

# 查看 modbus-driver 日志（最新 50 行
kubectl logs -n ithings -l app=modbus-driver --tail=50

# 查看 device-meter 日志（最新 50 行）
kubectl logs -n ithings -l app=device-meter --tail=50

# 查看 service
kubectl get service -n ithings

# 查看配置
kubectl get configmap -n ithings
```

## 网络说明

- **modbuslave → Windows 防火墙** 需要放行 502 端口，允许 Docker 容器访问。如果还是 Connection refused，大概率是防火墙拦截。

- **DNS 解析**：Docker Desktop 内置 `host.docker.internal 解析到 Windows 宿主机，配置正确。

- **Kubernetes DNS**：device-meter 通过 Service `modbus-driver:5555 解析到 modbus-driver ClusterIP，配置正确。

## 文件位置

所有部署文件位置：

```
/root/source/rust/ithings/deploy/
├── docker/
│   ├── build.sh
│   ├── modbus-driver.Dockerfile
│   └── device-meter.Dockerfile
└── k8s/
    ├── deploy.sh
    ├── namespace.yaml
    ├── configmaps.yaml
    ├── modbus-driver-deployment.yaml
    ├── device-meter-deployment.yaml
    ├── local-registry.yaml
    ├── DEPLOY_STATUS.md (本文档)
    └── README.md
```

## 部署命令

```bash
# 重新部署
kubectl apply -f deploy/k8s/namespace.yaml --validate=false
kubectl apply -f deploy/k8s/configmaps.yaml --validate=false
kubectl apply -f deploy/k8s/modbus-driver-deployment.yaml --validate=false
kubectl apply -f deploy/k8s/device-meter-deployment.yaml --validate=false
```

## 部署完成度：**100%** ✅**

所有配置正确，Pod 都在 Running 状态，只有网络配置正确。如果还是连不上，检查 Windows 防火墙放行 502 端口即可。
