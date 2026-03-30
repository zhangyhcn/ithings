#!/bin/bash
# Push script for ithings docker images
# Pushes already built images to registry

set -e

REGISTRY_ADDR=172.17.0.1:30500

# 推送到本地 registry
echo "Pushing images to registry..."
docker push $REGISTRY_ADDR/driver/modbus-driver:latest
docker push $REGISTRY_ADDR/driver/bacnet-driver:latest
docker push $REGISTRY_ADDR/device/device-meter:latest
docker push $REGISTRY_ADDR/infra/zmq-router:latest

echo "Done!"
echo "Images pushed:"
echo "  - $REGISTRY_ADDR/driver/modbus-driver:latest"
echo "  - $REGISTRY_ADDR/driver/bacnet-driver:latest"
echo "  - $REGISTRY_ADDR/device/device-meter:latest"
echo "  - $REGISTRY_ADDR/infra/zmq-router:latest"
