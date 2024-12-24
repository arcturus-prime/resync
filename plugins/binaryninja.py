#       CONFIG
#--------------------
PORT_NUMBER = 12007
#--------------------

import json
import socket

from binaryninja import *

class Client:
    def __init__(self): 
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def connect(self, port):
        self.socket.connect(("127.0.0.1", port))

    def send(self, data):
        binary_data = json.dumps(data) + "\n"

        print(binary_data)
        self.socket.sendall(binary_data.encode('utf-8'))

    def recv(self):
        binary_data = b''

        while b'\n' not in binary_data:
            data = self.socket.recv(1024)

            if data is None:
                return None

            binary_data += data

        return binary_data

client = Client()
client.connect(PORT_NUMBER)

def get_pointer_info(self, type_) -> Tuple[str, int]:
    name, depth = get_pointer_info_subroutine(type_, 0)
    return name, depth

def get_pointer_info_subroutine(self, type_, depth) -> Tuple[str, int]:
    if type_.type_class == TypeClass.PointerTypeClass:
        return get_pointer_info_subroutine(type_.children[0], depth + 1)

    return get_type_name(type_), depth

def lift_binal_function(func):
    arguments = []
    for parameter in func.type.parameters:
        arguments.append({ "name": parameter.name, "arg_type": get_type_name(parameter.type) })

    binal_func = { "location": func.start, "return_type": get_type_name(func.return_type), "arguments": arguments }

    return binal_func

def lift_binal_type(type_):
    binal_type = { "size": type_.width, "alignment": type_.alignment }

    if type_.type_class == TypeClass.PointerTypeClass:
        binal_type["info"] = {}
        binal_type["info"]["kind"] = "pointer"
        binal_type["info"]["to_type"], binal_type["info"]["depth"] = get_pointer_info(type_)

    return binal_type

def lift_binal_global(self, func):
    pass

def push_function(client: Client, func):
    for param in func.type.parameters:
        push_type_subroutine(client, param.type)

    client.send({ "kind": "pushfunction", "name": func.name, "data": lift_binal_function(func) })
    client.send({ "kind": "endtransaction" })

def push_type_subroutine(client: Client, type_: types.Type):
    for subtype in type_.children:
        push_type_subroutine(client, subtype)

    client.send({ "kind": "pushtype", "name": get_type_name(type_), "data": lift_binal_type(type_) })

def push_type(client: Client, type_: types.Type):
    push_type_subroutine(client, type_)
    client.send({ "kind": "endtransaction" })


class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()

        self.lifter = Lifter()

    # -----
    # HOOKS
    # -----

    def function_added(self, view: 'BinaryView', func: 'function.Function') -> None:
        
        self.lifter.push_function(client, func)

    def function_updated(self, view: 'BinaryView', func: 'function.Function') -> None:
        
        self.lifter.push_function(client, func)

    def function_removed(self, view: 'BinaryView', func: 'function.Function') -> None:
        pass

    def type_defined(self, view: 'BinaryView', name: 'types.QualifiedName', type: 'types.Type') -> None:

        self.lifter.push_type(client, type)

    # def symbol_updated(self, view, sym):

bv.register_notification(DecompilerInterface())

# while True:
    # message = client.recv()