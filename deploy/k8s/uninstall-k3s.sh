#!/bin/bash
# Uninstall k3s script

set -e

echo "Uninstalling k3s..."

# Stop k3s service
sudo systemctl stop k3s || true
sudo systemctl disable k3s || true

# Remove k3s binary
sudo rm -f /usr/local/bin/k3s
sudo rm -f /usr/local/bin/k3s-server
sudo rm -f /usr/local/bin/k3s-agent

# Remove k3s data
sudo rm -rf /var/lib/rancher/k3s
sudo rm -rf /etc/rancher/k3s
sudo rm -rf /var/lib/kubelet
sudo rm -rf /var/lib/cni
sudo rm -rf /etc/cni
sudo rm -rf /run/k3s

# Remove systemd service
sudo rm -f /etc/systemd/system/k3s.service
sudo rm -f /etc/systemd/system/k3s-agent.service
sudo rm -rf /etc/systemd/system/k3s.service.d
sudo rm -rf /etc/systemd/system/k3s-agent.service.d

# Reload systemd
sudo systemctl daemon-reload

echo "k3s uninstall completed!"
echo "You can now reinstall k3s with:"
echo "curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC=\"server --docker-registry-mirror https://docker.m.daocloud.io --docker-registry-mirror https://registry.cyou --docker-registry-mirror https://dockerproxy.link\" sh -"
