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
client.connect(12007)

class Lifter:
    def __init__(self):
        self.base_type_translation = {
            "uint64_t": "u64",
            "uint32_t": "u32",
            "uint16_t": "u16",
            "uint8_t": "u8",
            "int64_t": "i64",
            "int32_t": "i32",
            "int16_t": "i16",
            "int8_t": "i8",
            "float": "f32",
            "double": "f64",
            "char": "i8",
            "short": "i16",
        }

        self.object = {}

    def get_type_name(self, type_):
        if self.base_type_translation.get(type_.get_string()) is None:
            return type_.get_string()

        return self.base_type_translation[type_.get_string()]

    def get_pointer_info(self, type_) -> Tuple[str, int]:
        name, depth = self.get_pointer_info_subroutine(type_, 0)
        return name, depth

    def get_pointer_info_subroutine(self, type_, depth) -> Tuple[str, int]:
        if type_.type_class == TypeClass.PointerTypeClass:
            return self.get_pointer_info_subroutine(type_.children[0], depth + 1)

        return self.get_type_name(type_), depth

    def lift_binal_function(self, func):
        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "arg_type": self.get_type_name(parameter.type) })

        binal_func = { "location": func.start, "return_type": self.get_type_name(func.return_type), "arguments": arguments }

        return binal_func

    def lift_binal_type(self, type_):
        binal_type = { "size": type_.width, "alignment": type_.alignment }

        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = {}
            binal_type["info"]["kind"] = "pointer"
            binal_type["info"]["to_type"], binal_type["info"]["depth"] = self.get_pointer_info(type_)

        return binal_type

    def lift_binal_global(self, func):
        pass

    def push_function(self, client: Client, func):
        for param in func.type.parameters:
            self.push_type_subroutine(client, param.type)

        client.send({ "kind": "pushfunction", "name": func.name, "data": self.lift_binal_function(func) })
        client.send({ "kind": "endtransaction" })

    def push_type_subroutine(self, client: Client, type_: types.Type):
        for subtype in type_.children:
            self.push_type_subroutine(client, subtype)

        client.send({ "kind": "pushtype", "name": self.get_type_name(type_), "data": self.lift_binal_type(type_) })

    def push_type(self, client: Client, type_: types.Type):
        self.push_type_subroutine(client, type_)
        client.send({ "kind": "endtransaction" })


class Compiler:
    def compile_binal_type(self, type_: 'types.Type'):
        pass

    def compile_binal_argument(self, bnf: 'function.Function', arg):
        pass

    def compile_binal_function(self, func):
        bnf = bv.create_user_function(func.location)

        ret = self.compile_binal_type(func.return_type)
        bnf.set_auto_return_type(ret)

        bnf.set_auto_parameter_vars()


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