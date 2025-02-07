#       CONFIG
#--------------------
PORT_NUMBER = 12007
#--------------------

import json
import socket
import select

from typing import Tuple
from binaryninja import *

class Lifter:
    def __init__(self):
        self.map = {}
        self.current = 0

        pass

    def translate_object(self, object):
        if self.map[object] is None:
            self.map[object] = self.current
            self.current += 1

        return self.map[object]

    def get_pointer_info(self, type_):
        depth = 0
        while type_.children[0].type_class == TypeClass.PointerTypeClass:
            type_ = type_.children[0]
            depth += 1

        return type_, depth

    def lift_function(self, func):
        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "arg_type": self.translate_object(parameter.type) })

        binal_func = { "kind": "function", "name": func.name, "location": func.start, "return_type": self.translate_object(func.return_type), "arguments": arguments }

        return binal_func

    def lift_type(self, type_):
        binal_type = { "size": type_.width, "alignment": type_.alignment }

        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = {}
            binal_type["info"]["kind"] = "pointer"

            ptr_base_type, binal_type["info"]["depth"] = self.get_pointer_info(type_)
            binal_type["info"]["to_type"] = self.translate_object(ptr_base_type)

        return binal_type

    def lift_global(self, func):
        pass

class Connection:
    def __init__(self, socket: socket.socket): 
        self.socket = socket
        self.buffer = b''

    def send(self, data):
        binary_data = json.dumps(data) + "\n"

        self.socket.sendall(binary_data.encode('utf-8'))

    def recv(self):
        data = self.socket.recv(1024)

        self.buffer += data

        if b'\n' in self.buffer:
            data, _, self.buffer = self.buffer.partition(b'\n')
            return json.loads(data.decode('utf-8'))
        
        return None
    
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

    # def symbol_updated(self, view, sym):

class NetworkHandler(BackgroundTaskThread):
    def __init__(self, socket: socket.socket):
        super(NetworkHandler, self).__init__('Handling requests from resync...')

        self.lifter = Lifter()
        self.server = socket
        self.connections = [ socket ]

    def init_sync(self):
        for func in bv.functions: 
            self.lifter.lift_function(func)

    def handle_message(self, message):
        print(message) 

    def run(self):
        while True:
            read, write, error = select.select(self.connections, [], [])

            for connection in read:
                if connection == self.server:
                    s, addr = self.server.accept()

                    new_connection = Connection(s)
                    self.connections.append(new_connection)
                    bv.register_notification(DecompilerHandler(new_connection))
                    
                else:
                    data = connection.recv()
 
                    if data is None:
                        continue
                     
                    self.handle_message(data)


s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind('localhost', PORT_NUMBER)
s.listen(1)

handler = NetworkHandler(s)
handler.start()
