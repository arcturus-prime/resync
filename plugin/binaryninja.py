import json
import socket
import select
from typing import Iterable

from binaryninja import (
    TypeClass,
    BinaryView,
    function,
    types,
    BackgroundTaskThread,
    BinaryDataNotification,
)

#       CONFIG
# --------------------
PORT_NUMBER = 12007
INIT_SYNC_BATCH = 50
# --------------------

def get_pointer_info(type_):
    depth = 0
    while type_.children[0].type_class == TypeClass.PointerTypeClass:
        type_ = type_.children[0]
        depth += 1

    return type_, depth


def lift_function(func):
    arguments = []
    for parameter in func.type.parameters:
        arguments.append(
            {"name": parameter.name, "arg_type": parameter.type.get_string()}
        )
    
    binal_func = {
        "kind": "function",
        "location": func.start,
        "return_type": func.return_type.get_string(),
        "arguments": arguments,
    }

    return binal_func, func.name


def lift_type(type_):
    binal_type = {"kind": "type", "size": type_.width, "alignment": type_.alignment}
    binal_type["info"] = {}

    if type_.type_class == TypeClass.PointerTypeClass:
        binal_type["info"]["kind"] = "pointer"

        ptr_base_type, binal_type["info"]["depth"] = get_pointer_info(type_)
        binal_type["info"]["to_type"] = ptr_base_type.get_string()

    # TODO: Handle other types properly
    binal_type["info"]["kind"] = "uint"

    return binal_type, type_.get_string()


def lift_global(global_):
    pass


def add_and_lower_type(type_):
    kind = type_["info"]["kind"] 

    #TODO: Implement lowering for all types
    if kind == "pointer":
        pass
    if kind == "uint":
        pass
    if kind == "int":
        pass
    if kind == "float":
        pass
    if kind == "struct":
        pass
    if kind == "enum":
        pass
    if kind == "array":
        pass


def add_and_lower_function(function):
    func = bv.create_user_function(function["address"])
    func.name = function.name
    
    # creation of return type
    func.return_type

class Connection:
    def __init__(self, socket: socket.socket):
        self.socket = socket
        self.buffer = b""

    def send(self, data):
        binary_data = json.dumps(data) + "\n"

        try:
            self.socket.sendall(binary_data.encode("utf-8"))
        except ConnectionResetError:
            return None

    def recv(self):
        try:
            data = self.socket.recv(1024)
        except ConnectionResetError:
            return None

        self.buffer += data

        if b"\n" in self.buffer:
            data, _, self.buffer = self.buffer.partition(b"\n")
            return json.loads(data.decode("utf-8"))

        return None

    def close(self):
        self.socket.close()
        self.buffer = b""

    def fileno(self):
        return self.socket.fileno()


class DecompilerHandler(BinaryDataNotification):
    def __init__(self, connection: Connection):
        super().__init__()

        self.connection = connection

    def function_added(self, view: BinaryView, func: function.Function) -> None:
        pass

    def function_updated(self, view: BinaryView, func: function.Function) -> None:
        pass

    def function_removed(self, view: BinaryView, func: function.Function) -> None:
        pass

    def type_defined(
        self, view: BinaryView, name: types.QualifiedName, type: types.Type
    ) -> None:
        pass

    def symbol_updated(self, view, sym):
        pass


# Handles connecting Resync clients, receiving updates from clients, and pushing updates to clients
class NetworkHandler(BackgroundTaskThread):
    def __init__(self, socket: socket.socket):
        super().__init__("Handling requests from resync...", True)

        self.connections = [Connection(socket)]
        self.notifications = {}

    def sync_objects(self, connection: Connection, objects: Iterable):
        objects = []

        for index, pair in enumerate(objects):
            objects.append(pair[0])

            if index != 0 and index % INIT_SYNC_BATCH == 0:
                connection.send({"kind": "push", "objects": objects})
                objects.clear()

        connection.send({"kind": "push", "objects": objects})

    def init_connection(self, connection: Connection):
        self.sync_objects(connection, map(lift_type, bv.types.values()))
        self.sync_objects(connection, map(lift_function, bv.functions))

        notify = DecompilerHandler(connection)

        self.notifications[connection] = notify
        bv.register_notification(notify)

        self.connections.append(connection)

    def handle_message(self, message):
        kind = message["kind"]

        #TODO: Handle each message
        if kind == "push":
            pass
        if kind == "rename":
            pass
        if kind == "delete":
            pass

    def run(self):
        while not self.cancelled:
            read, _, error = select.select(self.connections, [], [], 0)

            for connection in read:
                if connection == self.connections[0]:
                    s, _ = connection.accept()
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
s.bind(("127.0.0.1", PORT_NUMBER))
s.listen(1)

handler = NetworkHandler(s)
handler.start()
