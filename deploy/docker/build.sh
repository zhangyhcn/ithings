#!/bin/bash
# Build script for ithings docker images

set -e

REGISTRY_ADDR=172.17.0.1:30500

# 本地编译（只编译需要发布的组件）
echo "Building required components locally..."
cargo build --release -p driver-modbus
cargo build --release -p driver-bacnet
cargo build --release -p device-meter
cargo build --release -p zmq-router

# 拷贝二进制到 Dockerfile 所在目录
echo "Copying binaries to docker directory..."
cp target/release/modbus-driver deploy/docker/
cp target/release/driver-bacnet deploy/docker/
cp target/release/device-meter deploy/docker/
cp target/release/zmq-router deploy/docker/

# 构建 docker 镜像，context 就是 deploy/docker 目录
# 命名规范：driver/xxx 用于驱动镜像，device/xxx 用于设备镜像
echo "Building modbus-driver image..."
docker build -f deploy/docker/modbus-driver.Dockerfile -t $REGISTRY_ADDR/driver/modbus-driver:latest deploy/docker

echo "Building bacnet-driver image..."
docker build -f deploy/docker/bacnet-driver.Dockerfile -t $REGISTRY_ADDR/driver/bacnet-driver:latest deploy/docker

echo "Building device-meter image..."
docker build -f deploy/docker/device-meter.Dockerfile -t $REGISTRY_ADDR/device/device-meter:latest deploy/docker

echo "Building zmq-router image..."
docker build -f deploy/docker/zmq-router.Dockerfile -t $REGISTRY_ADDR/infra/zmq-router:latest deploy/docker

# 清理复制的二进制
rm -f deploy/docker/modbus-driver deploy/docker/driver-bacnet deploy/docker/device-meter deploy/docker/zmq-router

# 推送到本地 registry
echo "Pushing images to registry..."
docker push $REGISTRY_ADDR/driver/modbus-driver:latest
docker push $REGISTRY_ADDR/driver/bacnet-driver:latest
docker push $REGISTRY_ADDR/device/device-meter:latest
docker push $REGISTRY_ADDR/infra/zmq-router:latest

echo "Done!"
echo "Images:"
echo "  - $REGISTRY_ADDR/driver/modbus-driver:latest"
echo "  - $REGISTRY_ADDR/driver/bacnet-driver:latest"
echo "  - $REGISTRY_ADDR/device/device-meter:latest"
echo "  - $REGISTRY_ADDR/infra/zmq-router:latest"
