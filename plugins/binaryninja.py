import json
import socket

from binaryninja import *

bin: BinaryView = bv

for binja_function in bin.functions:
    binal_function = {}
