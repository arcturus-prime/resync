import json
import socket

from binaryninja import *

file = open("deez.bproj")

bin: BinaryView = bv
project = {
    "functions": [],
    "globals": [],
    "types": [],
}

for binja_function in bin.functions:
    arguments = []
    for parameter in binja_function.type.parameters:
        arguments.append({ "name": parameter.name, "arg_type": parameter.type.get_string() })

    project["functions"].append({ "location": binja_function.start, "return_type": binja_function.return_type.get_string(), "arguments": arguments })

file.write(json.dumps(project))
file.close()