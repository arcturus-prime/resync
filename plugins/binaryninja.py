import requests
import json

from binaryninja import *

host = "http://127.0.0.1:12007"

# OBJECT ID MANGLING
def mangle_binal_type(type_):
    return "type_" + type_.altname

def mangle_binal_function(func):
    return "function_" + func.name

# OBJECT LIFTING
def create_binal_function(func):
    ranges = []
    for r in func.address_ranges:
        ranges.append((r.start, r.end))

    arguments = []
    for parameter in func.type.parameters:
        arguments.append({ "name": parameter.name, "type": mangle_binal_type(parameter.type) })

    binal_func = { "blocks": ranges, "return_type": mangle_binal_type(func.return_type), "arguments": arguments }

    return binal_func

def create_binal_type(type_):
    info = {}
    if type_.type_class == TypeClass.PointerTypeClass:
        info["kind"] = "pointer"
        info["to"] = mangle_binal_type(type_.children[0])

    binal_type = { "size": type_.width, "alignment": type_.alignment, "info": info }

    return binal_type

class DecompilerInterface(binaryninja.BinaryDataNotification):
    def __init__(self):
        super(DecompilerInterface, self).__init__()

    def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION ADDED")
        
        messages = []
        for param in func.type.parameters:
            messages.append({ "id": mangle_binal_type(param.type), "object": create_binal_type(param.type) })

        messages.append({ "id": mangle_binal_type(func.return_type), "object": create_binal_type(func.return_type) })
        messages.append({ "id": mangle_binal_function(func), "object": create_binal_function(func) })

        requests.post(host + "/objects", json=messages)

    def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION UPDATED")
        
        messages = []
        for param in func.type.parameters:
            messages.append({ "id": mangle_binal_type(param.type), "object": create_binal_type(param.type) })

        messages.append({ "id": mangle_binal_type(func.return_type), "object": create_binal_type(func.return_type) })
        messages.append({ "id": mangle_binal_function(func), "object": create_binal_function(func) })

        requests.post(host + "/objects", json=messages)

    def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
        print("FUNCTION REMOVED")

    def type_defined(self, view, name, type_):
        print("TYPE DEFINED")

    # def symbol_updated(self, view, sym):

bv.register_notification(DecompilerInterface())