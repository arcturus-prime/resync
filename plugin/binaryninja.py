import json
import socket
import select
import traceback

from binaryninja import (
    TypeClass,
    Type,
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
    while type_.type_class == TypeClass.PointerTypeClass:
        type_ = type_.target
        depth += 1

    return type_, depth


def lift_function(func):
    binal_objects = []

    arguments = []
    for parameter in func.type.parameters:
        arguments.append(
            {"name": parameter.name, "arg_type": parameter.type.altname}
        )

        binal_objects.extend(lift_type(parameter.type))
    
    binal_func = {
        "kind": "function",
        "name": func.name,
        "location": func.start,
        "return_type": func.return_type.altname,
        "arguments": arguments,
    }

    binal_objects.append(binal_func)

    return binal_objects


def lift_type(type_: Type):
    binal_types = [{"kind": "type", "name": type_.altname or type_.get_string(), "size": type_.width, "alignment": type_.alignment}]

    if type_.type_class == TypeClass.PointerTypeClass:
        binal_types[0]["info"] = { "kind": "pointer" }

        ptr_base_type, binal_types[0]["info"]["depth"] = get_pointer_info(type_)

        binal_types[0]["info"]["to_type"] = ptr_base_type.altname
        binal_types.extend(lift_type(ptr_base_type))

    elif type_.type_class == TypeClass.IntegerTypeClass:
        binal_types[0]["info"] = { "kind": "int" if type_.signed else "uint" } 
    elif type_.type_class == TypeClass.BoolTypeClass:
        binal_types[0]["info"] = { "kind": "bool" }
    elif type_.type_class == TypeClass.FloatTypeClass:
        binal_types[0]["info"] = { "kind": "float" }
    elif type_.type_class == TypeClass.EnumerationTypeClass:
        binal_types[0]["info"] = { "kind": "enum", "values": [] }
        
        for member in type_.members:
            binal_types[0]["info"]["values"].append({
                "name": member.name,
                "value": member.value
            })

    elif type_.type_class == TypeClass.StructureTypeClass:
        binal_types[0]["info"] = { "kind": "struct", "fields": [] }
   
        for field in type_.members:
            binal_types.extend(lift_type(field.type))
            binal_types[0]["info"]["fields"].append({
                "name": field.name,
                "offset": field.offset,
                "field_type": field.type.altname
            })

    elif type_.type_class == TypeClass.VoidTypeClass:
        binal_types[0]["info"] = { "kind": "void" }
    elif type_.type_class == TypeClass.ArrayTypeClass:
        binal_types.extend(lift_type(type_.element_type))
        binal_types[0]["info"] = { "kind": "array", "item_type": type_.element_type.altname, "size": type_.count }
    else:
        binal_types[0]["info"] = { "kind": "void" }

    return binal_types

def lift_global(global_):
    pass

def lower_and_add_types(types: List[dict]):
    # register types (and stubs for compound types)
    for type_ in types:
        kind = type_["info"]["kind"]
        
        if kind == "uint":
            binja_type = Type.int(type_["size"], False)
        elif kind == "int":
            binja_type = Type.int(type_["size"])
        elif kind == "float":
            binja_type = Type.float(type_["size"])
        elif kind == "pointer":
            binja_type = Type.pointer(type=Type.void(), width=type_["size"])

            for _ in range(type_["info"]["depth"]):
                binja_type = Type.pointer(type=pointer_type, width=type_["size"])

        elif kind == "struct":
            binja_type = Type.structure()
        elif kind == "enum":
            enum_values = [ [ value["name"], value["value"] ] for value in type_["info"]["values"] ]

            binja_type = Type.enumeration(members=enum_values, width=type_["size"])
        elif kind == "array":
            binja_type = Type.array(Type.int(4), binal_type["info"]["count"]) 

        bv.define_user_type(type_["name"], binja_type)

    # update compound types
    for type_ in types:
        binja_type = bv.types[type_["name"]].mutable_copy()
        kind = type_["info"]["kind"]
        
        if kind == "struct":
            for field in type_["info"]["fields"]:
                type_name = field["field_type"]
                binja_type.add_member_at_offset(field["name"], bv.types[type_name], field["offset"])

        elif kind == "array":
            type_name = type_["info"]["item_type"]
            binja_type.element_type = lower_primitive_type(type_name) or bv.types[type_name]

        elif kind == "pointer":
            type_name = type_["info"]["to_type"]
            binja_type.target = bv.types[type_name]

        bv.define_user_type(type_["name"], binja_type)

def lower_and_add_function(function):
    func = bv.create_user_function(function["address"])
    func.name = function.name
   
    # creation of return type
    func.return_type

def lower_and_add_globals(global_):
    pass

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
        super().__init__("Handling requests from Binal...", True)

        self.connections = [socket]
        self.notifications = {}

    def sync_objects(self, connection: Connection, object_iter: Iterable):
        objects = []

        for index, obj in enumerate(object_iter):
            objects.append(obj)

            if index != 0 and index % INIT_SYNC_BATCH == 0:
                connection.send({"kind": "push", "objects": objects})
                objects.clear()

        connection.send({"kind": "push", "objects": objects})

    def init_connection(self, connection: Connection):
        self.sync_objects(connection, (type_ for type_group in map(lift_type, bv.types.values()) for type_ in type_group))
        self.sync_objects(connection, (obj for obj_group in map(lift_function, bv.functions) for obj in obj_group))

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

    def close(self):
        for connection in self.connections:
            if connection != self.connections[0]:
                bv.unregister_notification(self.notifications[connection])

            connection.close()

    def run(self):
        while not self.cancelled:
            read, _, error = select.select(self.connections, [], [], 0)

            for connection in read:
                if connection == self.connections[0]:
                    s, _ = connection.accept()
                    
                    try:
                        self.init_connection(Connection(s))
                    except:
                        traceback.print_exc()
                        self.close()
                        return
                else:
                    data = connection.recv()

                    if data is None:
                        continue

                    try:
                        self.handle_message(data)
                    except:
                        traceback.print_exc()
                        self.close()
                        return
    
            for connection in error:
                connection.close()

                bv.unregister_notification(self.notifications[connection])
                self.connections.remove(connection)

        self.close()

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("127.0.0.1", PORT_NUMBER))
s.listen(1)

handler = NetworkHandler(s)
handler.start()
