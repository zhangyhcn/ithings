#!/bin/bash
# Build script for ithings docker images

set -e

REGISTRY_ADDR=172.17.0.1:30500

# 本地编译
echo "Building project locally..."
cargo build --release

# 拷贝二进制到 Dockerfile 所在目录
echo "Copying binaries to docker directory..."
cp target/release/modbus-driver deploy/docker/
cp target/release/device-meter deploy/docker/

# 构建 docker 镜像，context 就是 deploy/docker 目录
echo "Building modbus-driver image..."
docker build -f deploy/docker/modbus-driver.Dockerfile -t $REGISTRY_ADDR/ithings/modbus-driver:latest deploy/docker

echo "Building device-meter image..."
docker build -f deploy/docker/device-meter.Dockerfile -t $REGISTRY_ADDR/ithings/device-meter:latest deploy/docker

# 清理复制的二进制
rm -f deploy/docker/modbus-driver deploy/docker/device-meter

# 推送到本地 registry
echo "Pushing images to registry..."
docker push $REGISTRY_ADDR/ithings/modbus-driver:latest
docker push $REGISTRY_ADDR/ithings/device-meter:latest

echo "Done!"
echo "Images:"
echo "  - $REGISTRY_ADDR/ithings/modbus-driver:latest"
echo "  - $REGISTRY_ADDR/ithings/device-meter:latest"
