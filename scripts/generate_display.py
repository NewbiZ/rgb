#!/usr/bin/env python

import json

opcodes = ''
with open('all_opcodes.json') as f:
  opcodes = f.read()
opcodes = json.loads(opcodes)

prefix             = ['CB']
registers_8        = ['A', 'B', 'C', 'D', 'E', 'H', 'L']
registers_16       = ['BC', 'DE', 'HL', 'SP', 'AF']
deref_registers_8  = ['(C)']
deref_registers_16 = ['(BC)', '(DE)', '(HL)', '(HL+)', '(HL-)']
imm_8              = ['d8', 'r8', 'SP+r8']
imm_16             = ['a16', 'd16']
deref_imm_8        = ['(a8)']
deref_imm_16       = ['(a16)']
bitset             = ['0', '1', '2', '3', '4', '5', '6', '7']
predef_offset      = ['00H', '08H', '10H', '18H', '20H', '28H', '30H', '38H']
flags              = ['Z', 'NZ', 'C', 'NC']

def strarg(arg):
  if arg=='d8':
    return ('    let d8: u8 = self.mmu.read8(self.pc + 1);\n    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());', 1)
  elif arg=='r8':
    return ('    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;\n    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());', 1)
  elif arg=='SP+r8':
    return ('    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;\n    result.push_str(format!("SP+0x{:0>2.2X}", r8).as_slice());', 1)
  elif arg=='a16':
    return ('    let a16: u16 = self.mmu.read16(self.pc + 1);\n    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());', 2)
  elif arg=='d16':
    return ('    let d16: u16 = self.mmu.read16(self.pc + 1);\n    result.push_str(format!("0x{:0>4.4X}", d16).as_slice());', 2)
  elif arg=='(a8)':
    return ('    let pa8: u8 = self.mmu.read8(self.pc + 1);\n    result.push_str(format!("(0x{:0>2.2X})", pa8).as_slice());', 1)
  elif arg=='(a16)':
    return ('    let pa16: u16 = self.mmu.read16(self.pc + 1);\n    result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());', 2)
  else:
    return ('    result.push_str("' + arg + '");', 0)

for opcode in sorted(opcodes.items()):
  size = 1
  if len(opcode[0])==4:
    size = 1
  else:
    size = 2
  print 'fn disp_' + opcode[1]['command'] + '_' + opcode[0] + '(&self) -> (String, u8) {'
  print '    let mut result: String = String::new();'
  print '    result.push_str("' + opcode[1]['command'] + '");'
  if len(opcode[1]['args'])==1:
    print '    result.push_str(" ");'
    arg1 = strarg(opcode[1]['args'][0])
    print arg1[0]
    size += arg1[1]
  if len(opcode[1]['args'])==2:
    arg1 = strarg(opcode[1]['args'][0])
    arg2 = strarg(opcode[1]['args'][1])
    size += arg1[1]
    size += arg2[1]
    print '    result.push_str(" ");'
    print arg1[0]
    print '    result.push_str(", ");'
    print arg2[0]
  print '    (result, ' + str(size) + ')'
  print '}'
  print ''
