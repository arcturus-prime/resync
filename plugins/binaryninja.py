import socket

from binaryninja import *

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("127.0.0.1", 12007))

while True:
	s.send(b'{"action": "Push", "path": "global/deez", "object": { "bruh": 123 } }\n')
	buf = s.recv(1024)
	print(buf)

# class DecompilerInterface(binaryninja.BinaryDataNotification):
#     def __init__(self):
#         super(ProjectInterface, self).__init__()
#     def function_added(self, view: 'BinaryView', func: '_function.Function') -> None:
#         log_info(func)
#     def function_removed(self, view: 'BinaryView', func: '_function.Function') -> None:
#         log_info(func)
#     def function_updated(self, view: 'BinaryView', func: '_function.Function') -> None:
#         log_info(func)
# 	def type_defined(self, view, name, type_):

#     def symbol_updated(self, view, sym):

# bv.register_notification(DecompilerInterface())