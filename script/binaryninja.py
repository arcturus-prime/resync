import json
import socket
import select
import traceback

from binaryninja import (
    TypeClass,
    Type,
    BinaryView,
    Function,
    QualifiedName,
    BackgroundTaskThread,
    BinaryDataNotification,
    FunctionBuilder,
    StructureBuilder
)

#       CONFIG
# --------------------
PORT_NUMBER = 12007
INIT_SYNC_BATCH = 50
# --------------------


def lift_function(func):
    binal_objects = {}

    arguments = []
    for parameter in func.type.parameters:
        arguments.append(
            {"name": parameter.name, "arg_type": parameter.type.get_string()}
        )

        binal_objects.update(lift_type(parameter.type))
    
    binal_func = {
        "kind": "function",
        "location": func.start,
        "return_type": func.return_type.get_string(),
        "arguments": arguments,
    }

    binal_objects[func.name] = binal_func

    return binal_objects

def get_pointer_info(type_):
    depth = 0
    while type_.type_class == TypeClass.PointerTypeClass:
        type_ = type_.target
        depth += 1

    return type_, depth

def lift_type(type_: Type):
    binal_types = {}

    # array where we can push dependent types that we encounter
    to_parse = [type_]
    parsed_types = set({})

    while to_parse:
        type_ = to_parse.pop()

        # need to do this to prevent circular type traversal
        if type_ in parsed_types:
            continue

        parsed_types.add(type_)

        binal_type = {"kind": "type", "size": type_.width, "alignment": type_.alignment}
        
        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = { "kind": "pointer" }

            ptr_base_type, binal_type["info"]["depth"] = get_pointer_info(type_)
            binal_type["info"]["to_type"] = ptr_base_type.get_string()

            to_parse.append(ptr_base_type)
        elif type_.type_class == TypeClass.IntegerTypeClass:
            binal_type["info"] = { "kind": "int" if type_.signed else "uint" } 
        elif type_.type_class == TypeClass.BoolTypeClass:
            binal_type["info"] = { "kind": "bool" }
        elif type_.type_class == TypeClass.FloatTypeClass:
            binal_type["info"] = { "kind": "float" }
        elif type_.type_class == TypeClass.EnumerationTypeClass:
            binal_type["info"] = { "kind": "enum", "values": [] }
            
            for member in type_.members:
                binal_type["info"]["values"].append({
                    "name": member.name,
                    "value": member.value
                })

        elif type_.type_class == TypeClass.StructureTypeClass:
            binal_type["info"] = { "kind": "struct", "fields": [] }
       
            for field in type_.members:
                to_parse.append(field.type)
                binal_type["info"]["fields"].append({
                    "name": field.name,
                    "offset": field.offset,
                    "field_type": field.type.get_string()
                })

        elif type_.type_class == TypeClass.VoidTypeClass:
            binal_type["info"] = { "kind": "uint" }
        elif type_.type_class == TypeClass.FunctionTypeClass:
            binal_type["info"] = { "kind": "function", "return_type": type_.return_value.get_string(), "arg_types": []}

            for argument in type_.parameters:
                binal_type["info"]["arg_types"].append(argument.type.get_string())

        elif type_.type_class == TypeClass.ArrayTypeClass:
            to_parse.append(type_.element_type)
            binal_type["info"] = { "kind": "array", "item_type": type_.element_type.get_string(), "size": type_.count }
        elif type_.type_class == TypeClass.NamedTypeReferenceClass and type_.target(bv):
            to_parse.append(type_.target(bv))

            # don't want named types in the list to be sent
            continue
        else:
            # any other types shouldn't be sent either
            continue

        binal_types[type_.get_string()] = binal_type

    return binal_types

def lift_global(global_):
    binal_objects = lift_type(global_.type)

    binal_global = { "kind": "global", "location": global_.address, "global_type": global_.type.get_string() }
    binal_objects[global_.name] = binal_global

    return binal_objects

def lower_and_add_types(objects: dict):
    lowered_types = {}

    stacks: list[list] = [[(name, obj) for name, obj in objects if obj["kind"] == "type"], []]
    while stacks[0] or stacks[1]:
        if not stacks[0]:
            stacks[0], stacks[1] = stacks[1], stacks[0]

        pair = stacks[0].pop()
        type_ = pair[1]        
        name = pair[0]

        kind = type_["info"]["kind"]
        size = type_["size"]

        print(name)

        if kind == "uint" and size == 0:
            lowered_types[name] = Type.void()
        elif kind == "uint":
            lowered_types[name] = Type.int(size, False)
        elif kind == "int":
            lowered_types[name] = Type.int(size)
        elif kind == "float":
            lowered_types[name] = Type.float(size)
        elif kind == "bool":
            lowered_types[name] = Type.bool() 
        elif kind == "pointer":
            target_type = lowered_types.get(type_["info"]["to_type"])
            
            if not target_type:
                stacks[1].append(type_)
                continue

            binja_type = Type.pointer(bv.arch, type=target_type, width=size)
        
            for _ in range(type_["info"]["depth"]):
                binja_type = Type.pointer(bv.arch, type=binja_type, width=size)

            lowered_types[name] = binja_type
        elif kind == "function":
            binja_type = FunctionBuilder.create()
           
            # python moment
            failed = False
            for parameter in type_["info"]["arg_types"]:
                param_type = lowered_types.get(parameter)
                
                if not param_type:
                    stacks[1].append((name, type_))
                    failed = True
                    break

                binja_type.append(param_type)

            if failed:
                continue

            return_type = lowered_types.get(type_["info"]["return_type"])
            
            if not return_type:
                stacks[1].append((name, type_))
                continue

            binja_type.return_value = return_type
            lowered_types[name] = binja_type
        elif kind == "struct":
            binja_type = StructureBuilder.create()

            failed = False
            for field in type_["info"]["fields"]:
                field_type = lowered_types.get(field["field_type"])

                if not field_type:
                    stacks[1].append((name, type_))
                   
                    # this eliminates circular dependency between two or more structures
                    bv.define_user_type(name, Type.structure())
                    lowered_types[name] = Type.named_type_from_registered_type(bv, name)
                    
                    failed = True
                    break

                binja_type.add_member_at_offset(field["name"], field_type, field["offset"])

            if failed:
                continue

            bv.define_user_type(name, binja_type)
            lowered_types[name] = Type.named_type_from_registered_type(bv, name)
        elif kind == "enum":
            enum_values = [ [ value["name"], value["value"] ] for value in type_["info"]["values"] ]
            
            bv.define_user_type(name, Type.enumeration(members=enum_values, width=size))
            lowered_types[name] = Type.named_type_from_registered_type(bv, name)
        elif kind == "array":
            lowered_types[name] = Type.array(Type.int(4), type_["info"]["count"])

def lower_and_add_functions(objects: dict):
    for name, obj in objects:
        if obj["kind"] != "function":
            continue

        func = bv.create_user_function(obj["location"])
        func.name = name
        
        function_type = Type.function()

        for param in obj["arguments"]:
            var_type = bv.types[param["arg_type"]]
            function_type.append(var_type)

        function_type.return_value = bv.types[obj["return_type"]]
        func.type = function_type

        for i, param in enumerate(obj["arguments"]):
            func.set_parameter_name(i, param["name"])

def lower_and_add_globals(objects):
    for name, obj in objects:
        if obj["kind"] != "global":
            continue

        bv.create_data_var(obj["location"], bv.types[obj["global_type"]], name)

def lower_and_add_objects(objects: dict):
    lower_and_add_types(objects)
    lower_and_add_functions(objects)
    lower_and_add_globals(objects)

def remove_object(name: str):
    funcs = bv.get_function_by_name(name)
    
    if funcs:
        bv.remove_user_function(funcs[0])
    
    bv.undefine_user_type(name)
    
    for var in bv.data_vars:
        if var.name == name:
            bv.remove_user_data_var(var.address)

class Connection:
    def __init__(self, socket: socket.socket):
        self.socket = socket
        self.buffer = b""

    def send(self, data):
        binary_data = json.dumps(data) + "\n"
        self.socket.sendall(binary_data.encode("utf-8"))

    def recv(self):
        data = self.socket.recv(1024)

        if not data:
            raise ConnectionResetError

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

    def function_added(self, view: BinaryView, func: Function) -> None:
        binal_objects = lift_function(func)
        self.connection.send({ "kind": "push", "objects": binal_objects})

    def function_updated(self, view: BinaryView, func: Function) -> None:
        binal_objects = lift_function(func)
        self.connection.send({ "kind": "push", "objects": binal_objects})

    def function_removed(self, view: BinaryView, func: Function) -> None:
        self.connection.send({ "kind": "delete", "name": func.name })

    def type_defined(
        self, view: BinaryView, name: QualifiedName, type: Type
    ) -> None:
        binal_types = lift_type(type)
        self.connection.send({ "kind": "push", "objects": binal_types})

    def type_undefined(self, view: BinaryView, name: QualifiedName, type: Type) -> None:
        self.connection.send({ "kind": "delete", "name": type.get_string() })

    def data_var_updated(self, view, var):
        pass
    
    def data_var_added(self, view, var):
        pass

    def data_var_removed(self, view, var):
        pass

class NetworkHandler(BackgroundTaskThread):
    def __init__(self, socket: socket.socket):
        super().__init__("Handling requests from Binal...", True)

        self.connections = [socket]
        self.notifications = {}

    def sync_objects(self, connection: Connection, object_iter: Iterable):
        objects = {}

        for index, (name, obj) in enumerate(object_iter):
            objects[name] = obj
            if index != 0 and index % INIT_SYNC_BATCH == 0:
                connection.send({"kind": "push", "objects": objects})
                objects.clear()

        connection.send({"kind": "push", "objects": objects})

    def init_connection(self, connection: Connection):
        self.sync_objects(connection, ((name, obj) for d in map(lift_type, bv.types.values()) for name, obj in d.items()))
        self.sync_objects(connection, ((name, obj) for d in map(lift_function, bv.functions) for name, obj in d.items()))
        self.sync_objects(connection, ((name, obj) for d in map(lift_global, bv.data_vars.values()) for name, obj in d.items()))
        
        notify = DecompilerHandler(connection)

        self.notifications[connection] = notify
        bv.register_notification(notify)

        self.connections.append(connection)

    def handle_message(self, message):
        kind = message["kind"]
        
        #TODO: Handle each message
        if kind == "push":
            lower_and_add_objects(message["objects"])
        if kind == "delete":
            remove_object(message["name"])

    def close(self, connection: Connection):
        if connection != self.connections[0]:
            bv.unregister_notification(self.notifications[connection])

        connection.close()
        self.connections.remove(connection)

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
                        self.close(connection)
                else:
                    try:
                        data = connection.recv()
                    except ConnectionResetError:
                        self.connections.remove(connection)
                        continue

                    if not data:
                        continue

                    try:
                        self.handle_message(data)
                    except:
                        traceback.print_exc()
                        self.close(connection)

            for connection in error:
                self.close(connection)

        for connection in self.connections:
            if connection != self.connections[0]:
                bv.unregister_notification(self.notifications[connection])
            
            connection.close()

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("127.0.0.1", PORT_NUMBER))
s.listen(1)

handler = NetworkHandler(s)
handler.start()
