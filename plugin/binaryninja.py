#       CONFIG
#--------------------
PORT_NUMBER = 12007
INIT_SYNC_BATCH = 50
#--------------------

import json
import socket
import select

from typing import Tuple
from binaryninja import *

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
   
    def close():
        self.socket.close()
        self.buffer = b''

    def fileno(self):
        return self.socket.fileno()

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

class DecompilerHandler(BinaryDataNotification):
    def __init__(self, connection: Connection):
        super().__init__()

        self.connection = connection
    
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
        super().__init__('Handling requests from resync...', True)

        self.connections = [ socket ]
        self.notifications = {}

    #TODO: style improvements 
    def init_connection(self, connection: Connection):
        objects = []
        names = []

        for index, type_ in enumerate(bv.types.values()):
            resync_type, name = lift_type(type_)
            objects.append(resync_type)
            names.append(name)

            if index != 0 and index % INIT_SYNC_BATCH == 0:
                connection.send({ "kind": "push", "objects": objects, "names": names })
                objects.clear()
                names.clear()
        
        for index, function in enumerate(bv.functions):
            resync_function, name = lift_function(function)
            objects.append(resync_function)
            names.append(name)

            if index != 0 and index % INIT_SYNC_BATCH == 0:
                connection.send({ "kind": "push", "objects": objects, "names": names })
                objects.clear()
                names.clear()

        connection.send({ "kind": "push", "objects": objects, "names": names })
       
        notify = DecompilerHandler(connection)
   
        self.notifications[connection] = notify
        bv.register_notification(notify)

        self.connections.append(connection)
    
    def handle_message(self, message):
        print(message)

    def run(self):
        while not self.cancelled:
            read, write, error = select.select(self.connections, [], [], 0)

            for connection in read:
                if connection == self.connections[0]:
                    s, addr = connection.accept()
                    self.init_connection(Connection(s))
                else:
                    data = connection.recv()
                    
                    if data is None:
                        continue
                     
                    self.handle_message(data)
            
            for connection in error:
                connection.close()

                bv.unregister_notification(self.notifications[connection])
                self.connections.remove(connection)
        
        for connection in self.connections:
            if connection != self.connections[0]:
                bv.unregister_notification(self.notifications[connection])
        
            connection.close()

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(('127.0.0.1', PORT_NUMBER))
s.listen(1)

handler = NetworkHandler(s)
handler.start()
