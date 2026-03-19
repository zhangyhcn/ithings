# Kubernetes 部署

## 前置要求

- Kubernetes 集群
- `kubectl` 配置好
- Docker 本地镜像构建完成

## 部署步骤

### 1. 创建 namespace

```bash
kubectl apply -f deploy/k8s/namespace.yaml
```

### 2. 创建配置字典

```bash
# 自动从文件创建
kubectl apply -f deploy/k8s/configmaps.yaml
```

或者你可以手动从项目文件创建：

```bash
# modbus-driver 配置
kubectl create configmap modbus-driver-config \
  --from-file=drivers/modbus/modbus.json \
  -n ithings

# device-meter 配置  
kubectl create configmap device-meter-config \
  --from-file=devices/meter/examples/meter-config-integration.json \
  -n ithings
```

### 3. 部署 modbus-driver

```bash
kubectl apply -f deploy/k8s/modbus-driver-deployment.yaml
```

### 4. 部署 device-meter

```bash
kubectl apply -f deploy/k8s/device-meter-deployment.yaml
```

### 5. 检查部署

```bash
kubectl get pods -n ithings
kubectl logs -n ithings <pod-name>
```

## 架构图

```
┌──────────────────────────────────────────────────────────────────────┐
│                    device-meter                                    │
│  - 物模型加载                                                 │
│  - 规则评估                                                   │
│  - 事件发布 (MQTT/Kafka)                                      │
└──────────────────────────────────────────────────────────────────────┘
                              │ ZMQ
                              ▼
┌──────────────────────────────────────────────────────────────────────┐
│                    modbus-driver                                  │
│  - Device Profile 设备配置                                      │
│  - Modbus TCP 通信                                              │
└──────────────────────────────────────────────────────────────────────┘
                              │ Modbus TCP
                              ▼
┌──────────────────────────────────────────────────────────────────────┐
│                    物理 Modbus 设备                                 │
└──────────────────────────────────────────────────────────────────────┘
```

## 镜像推送

如果你使用私有镜像仓库：

```bash
# 重新打标签
docker tag ithings/modbus-driver:latest your-registry.example.com/ithings/modbus-driver:latest
docker tag ithings/device-meter:latest your-registry.example.com/ithings/device-meter:latest

# 推送
docker push your-registry.example.com/ithings/modbus-driver:latest
docker push your-registry.example.com/ithings/device-meter:latest
```

然后修改 deployment 中的 image 地址为你的私有仓库地址。

## 配置说明

- `modbus-driver-config` - ConfigMap 包含 modbus-driver 的配置
- `device-meter-config` - ConfigMap 包含 device-meter 的物模型配置
- 两个服务通过 Kubernetes DNS 发现：`modbus-driver:5555`
- device-meter 通过服务名称直接访问 modbus-driver，不需要修改配置

## 资源配额

默认配置：
- 请求：CPU 100m, 内存 64Mi
- 限制：CPU 500m, 内存 256Mi

根据你的需求，可以修改 deployment 中的 resources 配置。
