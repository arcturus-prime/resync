import requests
import json

from binaryninja import *

host = "http://127.0.0.1:12007"

class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()
        self.func_map = {}
        self.type_map = {}
        self.last_id = 0

    def id_binal_type(self, type_):
        binal_id = self.type_map.get(type_.get_string())
        if binal_id is None:
            self.type_map[type_.get_string()] = self.last_id
            binal_id = self.last_id

            self.last_id += 1

        return binal_id

    def id_binal_function(self, func):
        binal_id = self.func_map.get(func.name)
        if binal_id is None:
            self.func_map[func.name] = self.last_id
            binal_id = self.last_id

            self.last_id += 1

        return binal_id

    def create_binal_function(self, func):
        ranges = []
        for r in func.address_ranges:
            ranges.append((r.start, r.end))

        arguments = []
        for parameter in func.type.parameters:
            arguments.append({ "name": parameter.name, "type": self.id_binal_type(parameter.type) })

        binal_func = { "kind": "Function", "name": func.name, "blocks": ranges, "return_type": self.id_binal_type(func.return_type), "arguments": arguments }

        return binal_func

    def create_binal_type(self, type_):
        info = { "kind": "None" }
        if type_.type_class == TypeClass.PointerTypeClass:
            info["kind"] = "Pointer"
            info["to_type"] = self.id_binal_type(type_.children[0])

        binal_type = { "kind": "Type", "name": type_.get_string(), "size": type_.width, "alignment": type_.alignment, "info": info }

        return binal_type

    def push_function(self, func):
        messages = []
        for param in func.type.parameters:
            messages.append([ self.id_binal_type(param.type), self.create_binal_type(param.type) ])

        messages.append([ self.id_binal_type(func.return_type), self.create_binal_type(func.return_type) ])
        messages.append([ self.id_binal_function(func), self.create_binal_function(func) ])

        r = requests.post(host + "/objects", json=messages)

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