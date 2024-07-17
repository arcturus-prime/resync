import requests
import json

from binaryninja import *

host = "http://127.0.0.1:12007"

class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()

    def create_binal_function(self, func):
        ranges = []
        for r in func.address_ranges:
            ranges.append((r.start, r.end))

        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "type": parameter.type.get_string() })

        binal_func = { "kind": "function", "blocks": ranges, "return_type": func.return_type.get_string(), "arguments": arguments }

        return binal_func

    def create_binal_type(self, type_):
        binal_type = { "kind": "type", "size": type_.width, "alignment": type_.alignment }

        if type_.type_class == TypeClass.PointerTypeClass:
            binal_type["info"] = {}
            binal_type["info"]["kind"] = "pointer"
            binal_type["info"]["to"] = type_.children[0].get_string()

        return binal_type

    def push_function(self, func):
        messages = []
        for param in func.type.parameters:
            messages.append([ param.type.get_string(), self.create_binal_type(param.type) ])

        messages.append([ func.return_type.get_string(), self.create_binal_type(func.return_type) ])
        messages.append([ func.name, self.create_binal_function(func) ])

        r = requests.put(host + "/", json=messages)

    # -----
    # HOOKS
    # -----

    def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION ADDED")
        
        self.push_function(func)

    def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION UPDATED")
        
        self.push_function(func)

    def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION REMOVED")

    def type_defined(self, view, name, type_):
        print("TYPE DEFINED")

    # def symbol_updated(self, view, sym):

bv.register_notification(DecompilerInterface())