import socket
import json

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 12007))

message = { "action": "Push", "id": "function_" + "deeznuts", "object": {} }
data = bytearray(json.dumps(message) + "\n", "utf8")

s.send(data)

while True:
	buf = s.recv(1024)
	print(buf)