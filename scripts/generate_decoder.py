#!/usr/bin/env python

import json

opcodes = ''
with open('all_opcodes.json') as f:
  opcodes = f.read()
opcodes = json.loads(opcodes)

for opcode in sorted(opcodes.items()):
  print 'decoder.insert(' + opcode[0] + ' as u16, ' + 'Cpu::instr_' + opcode[1]['command'] + '_' + opcode[0] + ');'
