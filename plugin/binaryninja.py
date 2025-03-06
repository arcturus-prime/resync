#       CONFIG
#--------------------
PORT_NUMBER = 12007
#--------------------

import json
import socket
import select

from typing import Tuple
from binaryninja import *

def get_pointer_info(type_):
    depth = 0
    while type_.children[0].type_class == TypeClass.PointerTypeClass:
        type_ = type_.children[0]
        depth += 1

    return type_, depth

def lift_function(func):
    arguments = []
    for parameter in func.type.parameters:
        arguments.append({ "name": parameter.name, "arg_type": parameter.type.get_string() })

    resync_func = { "kind": "function", "location": func.start, "return_type": func.return_type.get_string(), "arguments": arguments }

    return resync_func, func.name

def lift_type(type_):
    resync_type = { "kind": "type", "size": type_.width, "alignment": type_.alignment }
    resync_type["info"] = {}
   
    if type_.type_class == TypeClass.PointerTypeClass:
        resync_type["info"]["kind"] = "pointer"

        ptr_base_type, resync_type["info"]["depth"] = get_pointer_info(type_)
        resync_type["info"]["to_type"] = ptr_base_type.get_string()

    resync_type["info"]["kind"] = "uint"

    return resync_type, type_.get_string()

def lift_global(global_):
    pass

class Connection:
    def __init__(self, socket: socket.socket): 
        self.socket = socket
        self.buffer = b''

    def send(self, data):
        binary_data = json.dumps(data) + "\n"

        try:
            self.socket.sendall(binary_data.encode('utf-8'))
        except ConnectionResetError:
            return None

    def recv(self):
        try:
            data = self.socket.recv(1024)
        except ConnectionResetError:
            return None

        self.buffer += data

        if b'\n' in self.buffer:
            data, _, self.buffer = self.buffer.partition(b'\n')
            return json.loads(data.decode('utf-8'))
        
        return None
    
    def init_sync(self):
        objects = []
        names = []

        for type_ in bv.types.values():
            resync_type, name = lift_type(type_)
            objects.append(resync_type)
            names.append(name)

        for func in bv.functions: 
            resync_func, name = lift_function(func)
            objects.append(resync_func)
            names.append(name)

        self.send({ "kind": "push", "objects": objects, "names": names })

    def fileno(self):
        return self.socket.fileno()


class DecompilerHandler(BinaryDataNotification):
    def __init__(self, connection: Connection):
        super(DecompilerHandler, self).__init__()

        self.connection = connection
    
    # -----
    # HOOKS
    # -----

    def function_added(self, view: 'BinaryView', func: 'function.Function') -> None:
        pass

    def function_updated(self, view: 'BinaryView', func: 'function.Function') -> None:
        pass

    def function_removed(self, view: 'BinaryView', func: 'function.Function') -> None:
        pass

    def type_defined(self, view: 'BinaryView', name: 'types.QualifiedName', type: 'types.Type') -> None:
        pass

    def symbol_updated(self, view, sym):
        pass

class NetworkHandler(BackgroundTaskThread):
    def __init__(self, socket: socket.socket):
        super(NetworkHandler, self).__init__('Handling requests from resync...', True)

        self.server = socket
        self.connections = [ socket ]

    def handle_message(self, message):
        print(message) 

    def cancel(self):
        for connection in self.connections:
            connections.close()

    def run(self):
        while True:
            read, write, error = select.select(self.connections, [], [])

            for connection in read:
                if connection == self.server:
                    s, addr = self.server.accept()

                    new_connection = Connection(s)
                    
                    self.connections.append(new_connection)
                    bv.register_notification(DecompilerHandler(new_connection))
                    new_connection.init_sync()
                    
                else:
                    data = connection.recv()
                    if data is None:
                        continue
                     
                    self.handle_message(data)


s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(('127.0.0.1', PORT_NUMBER))
s.listen(1)

handler = NetworkHandler(s)
handler.start()
