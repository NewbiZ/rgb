#!/usr/bin/env python

import json
import requests
import textwrap

# Load all opcodes in a json dict
opcodes = ''
with open('all_opcodes.json') as f:
  opcodes = f.read()
opcodes = json.loads(opcodes)

def getcmddoc(binary):
  if len(binary)==6:
    with open('opcodes_cb.doc.txt') as f:
      s = f.readlines()
      return s[int(binary[4], 16)*17 + int(binary[5], 16) +1]
  elif len(binary)==4:
    with open('opcodes.doc.txt') as f:
      s = f.readlines()
      return s[int(binary[2], 16)*17 + int(binary[3], 16) +1]
  else:
    raise Exception('Unknown command doc for ' + binary)

def strflag(f):
  if f=='-':
    return 'Preserved'
  elif f=='0':
    return 'Force unset (0)'
  elif f=='1':
    return 'Force set (1)'
  else:
    return 'Set if appropriate'

def strsize(s):
  if s==2:
    return '2 bytes'
  elif s==1:
    return '1 byte'
  else:
    raise Exception('Unknown size')

def strarg(arg):
  prefix = ['CB']
  registers_8 = ['A', 'B', 'C', 'D', 'E', 'H', 'L']
  registers_16 = ['BC', 'DE', 'HL', 'SP', 'AF']
  deref_registers_8 = ['(C)']
  deref_registers_16 = ['(BC)', '(DE)', '(HL)', '(HL+)', '(HL-)']
  imm_8 = ['d8', 'r8', 'SP+r8']
  imm_16 = ['a16', 'd16']
  deref_imm_8 = ['(a8)']
  deref_imm_16 = ['(a16)']
  bitset = ['0', '1', '2', '3', '4', '5', '6', '7']
  predef_offset = ['00H', '08H', '10H', '18H', '20H', '28H', '30H', '38H']
  flags = ['Z', 'NZ', 'C', 'NC']

  if arg in imm_8:
    if arg=='d8':
      return 'data: u8'
    elif arg=='r8' or arg=='SP+r8':
      return 'offset: s8'
    else:
      raise Exception('Unknown immediate 8')

  if arg in imm_16:
    if arg=='a16':
      return 'offset: s16'
    elif arg=='d16':
      return 'data: u16'
    else:
      raise Exception('Unknown immediate 16')

  if arg in deref_imm_8:
    if arg=='(a8)':
      return 'address: u8'
    else:
      raise Exception('Unknown address 8')

  if arg in deref_imm_16:
    if arg=='(a16)':
      return 'address: u16'
    else:
      raise Exception('Unknown address 16')

  if arg in registers_8 or \
    arg in registers_16 or \
    arg in deref_registers_8 or \
    arg in deref_registers_16 or \
    arg in bitset or \
    arg in predef_offset or \
    arg in flags or \
    arg in prefix:
    return None
  else:
    raise Exception('Unknown argument type ' + arg)

def strproto(args):
  s = ''
  for a in args:
    astr = strarg(a)
    if astr!=None:
      if not len(s)==0:
        s += ', '
      s += astr

  return s

s = ''
s += '    use std::collections::BTreeMap;\n'
s += '\n'
s += '    let opcodes = BTreeMap::new();\n'

for op in opcodes.items():
  s += '\n'
  # PROTOTYPE
  s += '    pub fn instr_' + op[1]['command'] + '_' + op[0] + '() {\n'
  # DOCUMENTATION
  s += '        //! Prototype: ' + op[1]['command'] + ' ' + ', '.join(op[1]['args']) + '\n'
  s += '        //! Mnemonic:  ' + op[1]['command'] + '\n'
  s += '        //! Size:      ' + strsize(op[1]['size']) + '\n'
  s += '        //! Binary:    ' + op[0] + '\n'
  s += '        //! Cycles:    '
  s += str(op[1]['cycles'][0]) +' cycles\n' if len(op[1]['cycles'])==1 else str(op[1]['cycles'][0]) + ' cycles (not taken) or ' + str(op[1]['cycles'][1]) + ' cycles (taken)\n'
  s += '        //! Flags:\n'
  s += '        //!   - Z: ' + strflag(op[1]['flags'][0]) + '\n'
  s += '        //!   - N: ' + strflag(op[1]['flags'][1]) + '\n'
  s += '        //!   - H: ' + strflag(op[1]['flags'][2]) + '\n'
  s += '        //!   - C: ' + strflag(op[1]['flags'][3]) + '\n'
  s += '        //! Description:'
  doc = getcmddoc(op[0])
  s += '\n        //!   ' + '\n        //!   '.join(textwrap.wrap(doc, 66))
  # STUB
  s += '\n'
  s += '        unimplemented!();\n'
  s += '    }\n'

s += '\n'
for op in opcodes.items():
  s += '    opcodes.insert(' + op[0] + ', instr_' + op[1]['command'] + '_' + op[0] + ');\n'

print s
