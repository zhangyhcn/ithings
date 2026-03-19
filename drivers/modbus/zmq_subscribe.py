#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
ZeroMQ 数据订阅测试脚本
用于订阅 Modbus 驱动发布的数据并打印

Topic 格式: {base_topic}/{device_name}/{resource_name}
示例: modbus/data/Modbus-Temperature-Sensor/Temperature
"""

import zmq
import json
import argparse
from datetime import datetime

def main():
    parser = argparse.ArgumentParser(description='ZeroMQ 数据订阅测试')
    parser.add_argument('--endpoint', default='tcp://localhost:5555', 
                      help='ZeroMQ 订阅端点 (默认: tcp://localhost:5555)')
    parser.add_argument('--topic', default='', 
                      help='过滤订阅 topic (例如: "modbus/data/Modbus-Temperature-Sensor/")')
    args = parser.parse_args()

    context = zmq.Context()
    socket = context.socket(zmq.SUB)
    
    print(f"连接到 ZeroMQ 端点: {args.endpoint}")
    print(f"订阅 topic: '{args.topic}'")
    
    socket.connect(args.endpoint)
    socket.setsockopt_string(zmq.SUBSCRIBE, args.topic)
    
    print("\n等待接收数据...\n")
    
    count = 0
    try:
        while True:
            topic, message = socket.recv().split(b' ', 1)
            topic_str = topic.decode('utf-8')
            message_str = message.decode('utf-8')
            
            count += 1
            timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            
            try:
                data = json.loads(message_str)
                print(f"[{timestamp}] Topic: {topic_str}")
                name = data.get('name', 'unknown')
                value = data.get('value', None)
                quality = data.get('quality', 'Unknown')
                units = data.get('units', '')
                ts = data.get('timestamp', '')
                
                unit_str = f" {units}" if units else ""
                print(f"  {name}: {value}{unit_str} [{quality}]\n")
            except json.JSONDecodeError as e:
                print(f"[{timestamp}] Topic: {topic_str}")
                print(f"JSON 解析错误: {e}")
                print(f"原始数据: {message_str[:200]}...\n")
            except Exception as e:
                print(f"[{timestamp}] Topic: {topic_str}")
                print(f"处理错误: {e}\n")
                
    except KeyboardInterrupt:
        print("\n\n收到 Ctrl+C，退出...")
    finally:
        socket.close()
        context.term()
    
    print(f"共收到 {count} 条数据")

if __name__ == '__main__':
    main()
