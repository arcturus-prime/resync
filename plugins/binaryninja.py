import requests
import json

from binaryninja import *

host = "http://127.0.0.1:12007"

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
        }

    def get_pointer_info(self, type_) -> Tuple[str, int]:
        name, depth = self.get_pointer_info_subroutine(type_, 0)
        return name, depth

    def get_pointer_info_subroutine(self, type_, depth) -> Tuple[str, int]:
        if type_.type_class == TypeClass.PointerTypeClass:
            return self.get_pointer_info_subroutine(type_.children[0], depth + 1)

        if self.base_type_translation.get(type_.get_string()) is None:
            return type_.get_string(), depth

        return self.base_type_translation[type_.get_string()], depth

    def lift_binal_function(self, func):
        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "type": parameter.type })

        binal_func = { "location": func.start, "return_type": func.return_type.get_string(), "arguments": arguments }

        return binal_func

    def lift_binal_type(self, type_):
        binal_type = { "size": type_.width, "alignment": type_.alignment }

        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = {}
            binal_type["info"]["kind"] = "pointer"
            binal_type["info"]["to"], binal_type["info"]["depth"] = self.get_pointer_info(type_)

        return binal_type

    def lift_binal_global(self, func):
        pass

    def push_function(self, func):
        types = []
        for param in func.type.parameters:
            types.append([ param.type.get_string(), self.lift_binal_type(param.type) ])

        types.append([ func.return_type.get_string(), self.lift_binal_type(func.return_type) ])

        r = requests.put(host + "/type", json=types)
        r = requests.put(host + "/function", json=[[func.name, self.lift_binal_function(func) ]])

    def push_type(self, type_: 'types.Type'):
        types = []
        for subtype in type_.children:
            types.append([ subtype.get_string(), self.lift_binal_type(subtype)])

        r = requests.put(host + "/type", json=types)

    def get_changes(self, timestamp: int):
        type_ = requests.get(host + "/type/" + str(timestamp))
        function = requests.get(host + "/function/" + str(timestamp))
        global_ = requests.get(host + "/global/" + str(timestamp))

        print(type_, function, global_)

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
        print("FUNCTION ADDED")
        
        self.lifter.push_function(func)

    def function_updated(self, view: 'BinaryView', func: 'function.Function') -> None:
        print("FUNCTION UPDATED")
        
        self.lifter.push_function(func)

    def function_removed(self, view: 'BinaryView', func: 'function.Function') -> None:
        print("FUNCTION REMOVED")

    def type_defined(self, view: 'BinaryView', name: 'types.QualifiedName', type: 'types.Type') -> None:
        print("TYPE DEFINED")

        self.lifter.push_type(type)

    # def symbol_updated(self, view, sym):

bv.register_notification(DecompilerInterface())
