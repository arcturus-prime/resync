import json
import socket

from binaryninja import *


s = socket()

bin: BinaryView = bv
project = {
    "functions": {},
    "globals": {},
    "types": {},
}

for binja_function in bin.functions:
    arguments = []
    for parameter in binja_function.type.parameters:
        arguments.append({ "name": parameter.name, "arg_type": parameter.type.get_string() })

    project["functions"][binja_function.name] = { "location": binja_function.start, "return_type": binja_function.return_type.get_string(), "arguments": arguments }

for binja_type in bin.types:
    project["types"][binja_type[1].get_string()] = { "size": binja_type[1].width, "alignment": binja_type[1].alignment, "info": { "kind": "int" } }

for binja_global in bin.data_vars:
    pass

