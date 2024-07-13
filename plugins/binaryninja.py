import socket
import json

from binaryninja import *
         
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 12007))

class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()

    def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION ADDED")
        message = { "action": "Push", "id": "function_" + func.name, "object": {} }
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)

    def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION REMOVED")
        message = { "action": "Delete", "id": "function_" + func.name }
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)
        
    def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION UPDATED")
        message = { "action": "Push", "id": "function_" + func.name, "object": {}}
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)

    def type_defined(self, view, name, type_):
        print("TYPE DEFINED")
        message = { "action": "Push", "id": "type_" + type_.name, "object": {}}
        data = bytearray(json.dumps(message) + "\n", "utf8")
        s.send(data)

    # def symbol_updated(self, view, sym):


bv.register_notification(DecompilerInterface())