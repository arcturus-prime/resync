import socket
import json

from binaryninja import *

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 12007))

class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()

    def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
        message = { "action": "Push", "path": "function/" + func.name, "object": {} }
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)

    def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
        message = { "action": "Delete", "path": "function/" + func.name }
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)
        
    def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
        message = { "action": "Push", "path": "function/" + func.name, "object": {}}
        data = bytearray(json.dumps(message) + "\n", "utf8")
        print(data)
        s.send(data)

    def type_defined(self, view, name, type_):
        message = { "action": "Push", "path": "type/" + type_.name, "object": {}}
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)

    # def symbol_updated(self, view, sym):


bv.register_notification(DecompilerInterface())

while True:
    buf = s.recv(1024)
    print(buf)