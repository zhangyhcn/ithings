# iThings Kubernetes Deployment

## 目录结构

```
deploy/
├── docker/
│   ├── build.sh              # 构建 Docker 镜像
│   ├── modbus-driver.Dockerfile    # modbus-driver Dockerfile
│   └── device-meter.Dockerfile  # device-meter Dockerfile
└── k8s/
    ├── deploy.sh               # 一键部署脚本
    ├── namespace.yaml          # namespace 定义
    ├── configmaps.yaml         # 配置字典（包含示例配置）
    ├── modbus-driver-deployment.yaml   # modbus-driver 部署
    ├── device-meter-deployment.yaml    # device-meter 部署
    ├── README.md                # 本文档
```

## 快速开始

### 前置要求

- Rust 1.70+ 开发环境
- Docker
- kubectl 配置好
- 可用的 Kubernetes 集群

### 一键部署

```bash
cd /path/to/ithings
chmod +x deploy/docker/build.sh
chmod +x deploy/k8s/deploy.sh
./deploy/k8s/deploy.sh
```

脚本会自动完成：

1. 🛠️ 本地 cargo build --release
2. 🐳 构建 Docker 镜像
3. 📦 创建 `ithings` namespace
4. 📄 创建 ConfigMaps（包含 device 和 driver 配置）
5. 🚀 部署 modbus-driver 和 device-meter
6. 🔍 等待就绪，显示状态和日志

## 架构

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    device-meter                                    │
│  - 加载物模型                                                │
│  - 定期轮询读取数据                                        │
│  - 规则评估                                                  │
│  - 触发事件 → MQTT/Kafka 发布                                 │
└─────────────────────────────────────────────────────────────────────────────┘
                              │ ZMQ
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    modbus-driver                                   │
│  - 加载 Device Profile                                        │
│  - 监听 ZMQ 等待 device 请求                                   │
│  - 根据 Profile 生成 Modbus 请求                                 │
│  - 和物理设备通信                                              │
│  - 返回标准化数据                                               │
└─────────────────────────────────────────────────────────────────────────────┘
                              │ Modbus TCP
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    物理 Modbus 设备                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 配置说明

### ConfigMap

| ConfigMap | 内容 |
|-----------|------|
| `modbus-driver-config` | modbus-driver 的配置，包含 Device Profile |
| `device-meter-config` | device-meter 的配置，包含 Thing Model |

### 配置模板

**modbus-driver** `drivers/modbus/modbus.json`:
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
  "custom": {
    "host": "localhost",
    "port": 502,
    "slave_id": 1,
    "profile": {
      "deviceResources": [...]
    }
  }
}
```

**device-meter** `devices/meter/examples/meter-config-integration.json`:
```json
{
  "device_name": "modbus-meter-1",
  "device_type": "modbus-meter",
  "poll_interval_ms": 1000,
  "driver": {
    "enabled": true,
    "server_address": "tcp://modbus-driver:5555"
  },
  "custom": {
    "thing_model": {
      "model_id": "modbus-temperature-sensor.1.0",
      "properties": [...],
      "events": [...],
      "rules": [...]
    }
  }
}
```

## 网络

- `modbus-driver` 暴露 `5555` 端口
- device 通过 DNS `modbus-driver:5555` 访问，在同一个 namespace 内自动发现

## 完整部署流程

### 第一步：本地构建

```bash
cd /path/to/ithings
./deploy/docker/build.sh
```

这会：
- 本地 `cargo build --release`
- 构建两个 Docker 镜像

### 第二步：推送到镜像仓库（可选，如果使用私有 registry）

```bash
docker tag ithings/modbus-driver:latest your-registry.example.com/ithings/modbus-driver:latest
docker tag ithings/device-meter:latest your-registry.example.com/ithings/device-meter:latest
docker push your-registry.example.com/ithings/modbus-driver:latest
docker push your-registry.example.com/ithings/device-meter:latest
```

然后修改 `deploy/k8s/modbus-driver-deployment.yaml` 和 `device-meter-deployment.yaml` 中的镜像地址。

### 第三步：部署到 Kubernetes

```bash
./deploy/k8s/deploy.sh
```

### 第四步：检查

```bash
# 查看 Pod 状态
kubectl get pods -n ithings

# 查看日志
kubectl logs -n ithings -l app=modbus-driver -f
kubectl logs -n ithings -l app=device-meter -f
```

## 配置动态添加设备

1. 修改 `deploy/k8s/configmaps.yaml`，添加新的 device 配置
   - 在 `modbus-driver-config` 中添加新的 device profile
   - 在 `device-meter-config` 中添加对应的 thing model

2. 重新应用配置：

```bash
kubectl apply -f deploy/k8s/configmaps.yaml -n ithings
# 重启 Pod
kubectl rollout restart deployment modbus-driver -n ithings
kubectl rollout restart deployment device-meter -n ithings
```

3. device 会自动加载新的配置，driver 也会自动加载新的 profile，开始采集

## 资源配额

默认配置：
- **requests**: cpu 100m, memory 64Mi
- **limits**: cpu 500m, memory 256Mi

根据实际负载修改 `deploy/k8s/*-deployment.yaml` 中的 resources 配置。

## 故障排查

### 镜像拉取失败

- 检查 Docker 网络是否能访问 docker.io
- 确认你已经推送镜像到你的私有 registry 并且修改了 deployment 中的镜像地址
- 使用 `kubectl describe pod <pod-name> -n ithings` 查看具体错误

### 启动失败

- 查看日志：`kubectl logs -n ithings <pod-name>`
- 确认配置文件格式正确，JSON 语法没有问题
- 确认 `driver.server_address` 正确，使用 `modbus-driver:5555` （Kubernetes 集群内 DNS 解析）

### 网络问题

- device 连不上 driver：确认在同一个 namespace，service 正确创建
- 检查 `kubectl get service -n ithings` 确认 service 存在

## 目录结构总览

```
ithings/
├── common/                        # 通用库
│   └── device-core/             # 物模型核心
├── drivers/                     # driver 实现
│   └── modbus/                 # Modbus 驱动
├── devices/                     # device 实现
│   └── meter/                 # 电表 device
├── deploy/                     # 部署相关
│   ├── docker/                # Docker 文件和构建脚本
│   │   ├── build.sh
│   │   ├── modbus-driver.Dockerfile
│   │   └── device-meter.Dockerfile
│   └── k8s/                   # Kubernetes 部署
│       ├── deploy.sh           # 一键部署脚本
│       ├── namespace.yaml      # namespace
│       ├── configmaps.yaml      # 配置字典
│       ├── modbus-driver-deployment.yaml
│       ├── device-meter-deployment.yaml
│       └── README.md          # 本文档
├── doc/                        # 项目文档
└── README.md
```

## 架构设计回顾

| 层次 | 职责 |
|------|------|
| device | 物模型定义、规则引擎、事件发布 |
| driver | 协议通信、数据采集、标准化输出 |
| ZMQ | device ↔ driver 通信 |
| Kubernetes | 编排、配置管理 |

符合项目初始设计：
> - **device-core** 放入 common，提供物模型定义、规则、状态机
> - **driver** 按不同协议实现，device 这边专注业务逻辑
> - **device** 按设备类型实现，加载物模型，连接 driver
> - **Kubernetes** 统一部署配置管理

