#!/bin/bash
# Build script for ithings docker images

# 本地编译
echo "Building project locally..."
cargo build --release

# 构建 docker 镜像
echo "Building modbus-driver image..."
docker build -f docker/modbus-driver.Dockerfile -t ithings/modbus-driver:latest .

echo "Building device-meter image..."
docker build -f docker/device-meter.Dockerfile -t ithings/device-meter:latest .

echo "Done!"
echo "Images:"
echo "  - ithings/modbus-driver:latest"
echo "  - ithings/device-meter:latest"
