#!/bin/bash
# Full deployment script for ithings to Kubernetes

set -e

echo "=========================================="
echo "iThings Kubernetes Deployment"
echo "=========================================="

# 检查命令是否存在
command -v cargo >/dev/null 2>&1 || {
    echo "Error: cargo not found"
    exit 1
}

command -v docker >/dev/null 2>&1 || {
    echo "Error: docker not found"
    exit 1
}

command -v kubectl >/dev/null 2>&1 || {
    echo "Error: kubectl not found"
    exit 1
}

# 步骤 1: 本地编译
echo ""
echo "[1/6] 🔧  Local cargo build --release..."
cd "$(git rev-parse --show-toplevel)"
cargo build --release

# 步骤 2: 构建 Docker 镜像
echo ""
echo "[2/6] 🐳  Building docker images..."
./deploy/docker/build.sh

# 步骤 3: 创建 namespace
echo ""
echo "[3/6] 📦  Creating namespace ithings..."
kubectl apply -f deploy/k8s/namespace.yaml

# 步骤 4: 创建 configmaps
echo ""
echo "[4/6] 📄  Creating configmaps..."
kubectl apply -f deploy/k8s/configmaps.yaml

# 步骤 5: 部署 applications
echo ""
echo "[5/6] 🚀  Deploying applications..."
kubectl apply -f deploy/k8s/modbus-driver-deployment.yaml
kubectl apply -f deploy/k8s/device-meter-deployment.yaml

# 步骤 6: 等待就绪检查
echo ""
echo "[6/6] 🔍  Waiting for pods to be ready..."
sleep 10

echo ""
echo "=========================================="
echo "Deployment Status"
echo "=========================================="
kubectl get pods -n ithings

echo ""
echo "Checking logs:"
echo ""
echo "modbus-driver logs:"
kubectl logs -n ithings -l app=modbus-driver --tail=20
echo ""
echo "device-meter logs:"
kubectl logs -n ithings -l app=device-meter --tail=20

echo ""
echo "=========================================="
echo "✅ Deployment completed!"
echo "=========================================="
echo ""
echo "To check status:"
echo "  kubectl get pods -n ithings"
echo "To view logs:"
echo "  kubectl logs -n ithings -l app=modbus-driver -f"
echo "  kubectl logs -n ithings -l app=device-meter -f"
