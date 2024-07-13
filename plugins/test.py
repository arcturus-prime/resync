import socket
import json
import time

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 12007))

while True:
	time.sleep(1)
	message = { "action": "Push", "id": "function_" + "deeznuts", "object": {} }
	data = bytearray(json.dumps(message) + "\n", "utf8")

	s.send(data)
	buf = s.recv(1024)

	print(buf)