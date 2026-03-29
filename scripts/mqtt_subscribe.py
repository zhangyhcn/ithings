#!/usr/bin/env python3
import paho.mqtt.client as mqtt
import time

def on_connect(client, userdata, flags, reason_code, properties):
    print(f"Connected with result code {reason_code}")
    client.subscribe("/#")

def on_message(client, userdata, msg):
    print(f"Topic: {msg.topic}")
    print(f"Payload: {msg.payload.decode()}")
    print("-" * 50)

client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
client.on_connect = on_connect
client.on_message = on_message

print("Connecting to 172.25.219.101:1883...")
client.connect("172.25.219.101", 1883, 60)
client.loop_start()

time.sleep(10)
client.loop_stop()
print("Done")
