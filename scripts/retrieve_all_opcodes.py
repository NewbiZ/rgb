#!/usr/bin/env python

from collections import OrderedDict
import json

opcodes = ''
opcodes_cb = ''
with open('opcodes.json') as f:
  opcodes = f.read()
with open('opcodes_cb.json') as f:
  opcodes_cb = f.read()

opcodes = json.loads(opcodes)
opcodes_cb = json.loads(opcodes_cb)

for cmd in opcodes.items():
  cmd[1]['size'] = 1

for cmd in opcodes_cb.items():
  k = '0xCB' + cmd[0].split('0x')[1]
  opcodes[k] = cmd[1]
  opcodes[k]['size'] = 2

json.dump(OrderedDict(opcodes), open('all_opcodes.json', 'w'), indent=2)


