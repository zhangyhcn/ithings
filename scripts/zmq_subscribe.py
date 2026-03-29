#!/usr/bin/env python3
import zmq
import json

def main():
    context = zmq.Context()
    socket = context.socket(zmq.SUB)
    
    zmq_address = f"tcp://localhost:5555"
    print(f"Connecting to {zmq_address}")
    socket.connect(zmq_address)
    
    socket.setsockopt_string(zmq.SUBSCRIBE, "")
    
    print("Waiting for data...")
    print("=" * 50)
    
    count = 0
    while True:
        try:
            message = socket.recv_string()
            count += 1
            parts = message.split(' ', 1)
            if len(parts) == 2:
                topic, payload = parts
                print(f"\n[#{count}] Topic: {topic}")
                try:
                    data = json.loads(payload)
                    print(f"Parsed: {json.dumps(data, indent=2)}")
                except json.JSONDecodeError:
                    print(f"Payload (not JSON): {payload}")
            else:
                print(f"\n[#{count}] Raw: {message}")
                try:
                    data = json.loads(message)
                    print(f"Parsed: {json.dumps(data, indent=2)}")
                except json.JSONDecodeError:
                    print("(Not JSON format)")
        except KeyboardInterrupt:
            print("\n\nStopped by user")
            break
        except Exception as e:
            print(f"Error: {e}")
            break
    
    socket.close()
    context.term()

if __name__ == "__main__":
    main()
