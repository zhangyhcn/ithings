#!/bin/bash

cat > /tmp/test-config.json << 'EOF'
{
  "devices": [
    {
      "device_id": "caec2dfb-b25b-40a3-9ccf-8c85bd119b1d",
      "device_name": "一号电表",
      "device_type": "电表",
      "driver": {
        "custom": {
          "address": "192.168.1.100",
          "slave_id": 1
        },
        "driver_name": "modbus-driver",
        "driver_type": "modbus",
        "logging": {
          "format": "json",
          "level": "info"
        },
        "poll_interval_ms": 1000,
        "zmq": {
          "enabled": true,
          "publisher_address": "tcp://*:5555",
          "topic": "modbus/data"
        }
      },
      "poll_interval_ms": 1000,
      "thing_model": {
        "description": "电表产品",
        "device_type": "电表",
        "manufacturer": "ithings",
        "model_id": "f4e6866b-2ec2-4e49-bc3c-96255882451b",
        "model_version": "1.0",
        "properties": []
      }
    },
    {
      "device_id": "21fe6f7d-524b-4e7c-9dc1-c0bd08e7cd35",
      "device_name": "二号电表",
      "device_type": "电表",
      "driver": {
        "custom": {
          "address": "192.168.1.101",
          "slave_id": 2
        },
        "driver_name": "modbus-driver",
        "driver_type": "modbus",
        "logging": {
          "format": "json",
          "level": "info"
        },
        "poll_interval_ms": 1000,
        "zmq": {
          "enabled": true,
          "publisher_address": "tcp://*:5555",
          "topic": "modbus/data"
        }
      },
      "poll_interval_ms": 1000,
      "thing_model": {
        "description": "电表产品",
        "device_type": "电表",
        "manufacturer": "ithings",
        "model_id": "f4e6866b-2ec2-4e49-bc3c-96255882451b",
        "model_version": "1.0",
        "properties": []
      }
    }
  ],
  "namespace_id": "",
  "org_id": "",
  "remote_transport": {
    "broker": null,
    "brokers": null,
    "client_id": null,
    "password": null,
    "type": "mqtt",
    "username": null
  },
  "site_id": "",
  "tenant_id": ""
}
EOF

./target/release/device-meter -c /tmp/test-config.json --group
