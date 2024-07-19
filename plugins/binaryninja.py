import requests
import json

from binaryninja import *

host = "http://127.0.0.1:12007"

class Lifter:
    def create_binal_function(self, func):
        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "type": parameter.type.get_string() })

        binal_func = { "location": func.start, "return_type": func.return_type.get_string(), "arguments": arguments }

        return binal_func

    def create_binal_type(self, type_):
        binal_type = { "size": type_.width, "alignment": type_.alignment }

        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = {}
            binal_type["info"]["kind"] = "pointer"
            binal_type["info"]["to"] = type_.children[0].get_string()

        return binal_type

    def create_binal_global(self, func):
        pass

    def push_function(self, func):
        types = []
        for param in func.type.parameters:
            types.append([ param.type.get_string(), self.create_binal_type(param.type) ])

        types.append([ func.return_type.get_string(), self.create_binal_type(func.return_type) ])

        r = requests.put(host + "/type", json=types)
        r = requests.put(host + "/function", json=[[func.name, self.create_binal_function(func) ]])

    def push_type(self, type_):
        types = []
        for subtype in type_.children:
            types.append([ subtype.get_string(), self.create_binal_type(subtype)])

        types.append([ type_.get_string(), self.create_binal_type(param.type) ])
        r = requests.put(host + "/type", json=types)

    def get_changes(self, timestamp):
        type_ = requests.get(host + "/type/" + timestamp)
        function = requests.get(host + "/function/" + timestamp)
        global_ = requests.get(host + "/global/" + timestamp)

        print(type_, function, global_)

class Compiler:
    def compile_binal_type(self, type_):


    def compile_binal_argument(self, bnf, arg):
        

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

    def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION ADDED")
        
        self.lifter.push_function(func)

    def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION UPDATED")
        
        self.lifter.push_function(func)

    def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION REMOVED")

    def type_defined(self, view, name, type_):
        print("TYPE DEFINED")

        self.lifter.push_type(type_)

    # def symbol_updated(self, view, sym):

bv.register_notification(DecompilerInterface())