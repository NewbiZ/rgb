#![allow(dead_code)]
#![allow(unstable)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]

use std::u8;
use std::u16;
use std::fmt;
use std::collections::BTreeMap;
use super::mmu::Mmu;

#[cfg(test)]
mod tests;

/// All different values of flags that could
/// end up in `Cpu` register `Cpu::f`.
pub enum Flag {
    /// Previous operation resulted in zero
    Zero      = 0x80,
    /// Previous operation was a subtraction
    Operation = 0x40,
    /// Previous operation resulted in a 4 bit overflow
    HalfCarry = 0x20,
    /// Previous operation resulted in a 8 bit overflow
    Carry     = 0x10,
    /// Previous operation triggered no flag
    None      = 0x00,
}

/// This struct models a GameBoy Z80 processor.
pub struct Cpu {
    /// Accumulator register
    a:  u8,
    /// General purpose register
    b:  u8,
    /// General purpose register
    c:  u8,
    /// General purpose register
    d:  u8,
    /// General purpose register
    e:  u8,
    /// General purpose register
    h:  u8,
    /// General purpose register
    l:  u8,
    /// Flag register
    f:  u8,
    /// Program counter register
    pub pc: u16,
    /// Stack pointer register
    sp: u16,
    /// Clock
    m: u8,
    /// Clock
    t: u8,
    /// Memory management unit
    pub mmu: Mmu,
    /// Boolean indicating whether processor is stopped
    pub stop: bool,
}

// ==============================================
// Implementation
// ==============================================

impl Cpu {
    pub fn new() -> Cpu {
        //! Create a new `Cpu`. All registers and clocks should be set to 0.
        Cpu {
            a:    0,
            b:    0,
            c:    0,
            d:    0,
            e:    0,
            h:    0,
            l:    0,
            f:    Flag::None as u8,
            pc:   0,
            sp:   0,
            m:    0,
            t:    0,
            mmu:  Mmu::new(),
            stop: true,
        }
    }

    pub fn reset(&mut self) {
        //! Reset the `Cpu` to its pristine state. All registers are set to 0.
        self.a =    0;
        self.b =    0;
        self.c =    0;
        self.d =    0;
        self.e =    0;
        self.h =    0;
        self.l =    0;
        self.f =    Flag::None as u8;
        self.pc =   0;
        self.sp =   0;
        self.m =    0;
        self.t =    0;
        self.stop = true;
    }

    pub fn stopped(&self) -> bool {
        self.stop
    }

    pub fn step(&mut self) {
        let mut opcode: u16 = self.mmu.read8(self.pc) as u16;
        if opcode==0xCB {
            opcode = (opcode << 8) + self.mmu.read8(self.pc+1) as u16;
        }
        match DECODER.get(&opcode) {
            Some(&(instr, _)) => instr(self),
            _ => panic!("error: unknown opcode {0:X}", self.pc),
        }
    }

    pub fn state(&self) -> (String, u8) {
        let mut opcode: u16 = self.mmu.read8(self.pc) as u16;
        if opcode==0xCB {
            opcode = (opcode << 8) + self.mmu.read8(self.pc+1) as u16;
        }
        match DECODER.get(&opcode) {
            Some(&(_, disp)) => disp(self),
            _ => panic!("error: unknown opcode {0:X}", self.pc),
        }
    }

    pub fn run(&mut self) {
        self.stop = false;
        while !self.stop {
            self.step();
        }
    }

    pub fn instr_ADD_0x85(&mut self) {
        //! - Prototype: `ADD A, L`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x85`
        //! - Cycles:    4 cycles
        //! - Flags:
        //!   - `Z`: Set if appropriate
        //!   - `N`: Force unset (0)
        //!   - `H`: Set if appropriate
        //!   - `C`: Set if appropriate
        //! - Description:
        //!   Adds l to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.l) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.l;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xE7(&mut self) {
        //! - Prototype: `RST 20H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xE7`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 20h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x20;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_AND_0xE6(&mut self) {
        //! - Prototype: `AND d8`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xE6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with *.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.a &= d8;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_JR_0x28(&mut self) {
        //! - Prototype: `JR Z, r8`
        //! - Mnemonic:  `JR`
        //! - Size:      2 bytes
        //! - Binary:    `0x28`
        //! - Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        if self.f & (Flag::Zero as u8) != 0 {
            // Update clocks
            self.m += 3;
            self.t += 12;
            // Update program counter
            if r8 < 0 {
                self.pc -= -r8 as u16;
            } else {
                self.pc += r8 as u16;
            }
        } else {
            // Update clocks
            self.m += 2;
            self.t += 8;
            // Update program counter
            self.pc += 2;
        }
    }

    pub fn instr_ADD_0x29(&mut self) {
        //! - Prototype: `ADD HL, HL`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x29`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The value of hl is added to hl.

        // Update registers h and l
        let lhs: u16 = ((self.h as u16) << 8) + self.l as u16;
        let rhs: u16 = lhs;
        let res: u16 = rhs + lhs;
        self.h = (res >> 8) as u8;
        self.l = res as u8;

        // Update flags
        self.f &= !(Flag::Operation as u8);
        if lhs > (u16::MAX - rhs) {
            self.f |= Flag::Carry as u8;
        }

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x22(&mut self) {
        //! - Prototype: `LD (HL+), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x22`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Store a in (hl), then increment hl

        // Store a in (hl)
        let mut hl: u16 = ((self.h as u16) << 8) + self.l as u16;
        self.mmu.write8(hl, self.a);
        // Increment hl
        hl += 1;
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x23(&mut self) {
        //! - Prototype: `INC HL`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x23`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x20(&mut self) {
        //! - Prototype: `JR NZ, r8`
        //! - Mnemonic:  `JR`
        //! - Size:      2 bytes
        //! - Binary:    `0x20`
        //! - Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        if self.f & (Flag::Zero as u8) == 0 {
            // Update clocks
            self.m += 3;
            self.t += 12;
            // Update program counter
            if r8 < 0 {
                self.pc -= -r8 as u16;
            } else {
                self.pc += r8 as u16;
            }
        } else {
            // Update clocks
            self.m += 2;
            self.t += 8;
            // Update program counter
            self.pc += 2;
        }
    }

    pub fn instr_LD_0x21(&mut self) {
        //! - Prototype: `LD HL, d16`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0x21`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads ** into hl.

        let d16: u16 = self.mmu.read16(self.pc + 1);
        self.h = (d16 >> 8) as u8;
        self.l = d16 as u8;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_LD_0x26(&mut self) {
        //! - Prototype: `LD H, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x26`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into h.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.h = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DAA_0x27(&mut self) {
        //! - Prototype: `DAA `
        //! - Mnemonic:  `DAA`
        //! - Size:      1 byte
        //! - Binary:    `0x27`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Preserved
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adjusts a for BCD addition and subtraction operations.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x24(&mut self) {
        //! - Prototype: `INC H`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x24`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x25(&mut self) {
        //! - Prototype: `DEC H`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x25`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from h.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xE9(&mut self) {
        //! - Prototype: `JP (HL)`
        //! - Mnemonic:  `JP`
        //! - Size:      1 byte
        //! - Binary:    `0xE9`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value of hl into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0xE8(&mut self) {
        //! - Prototype: `ADD SP, r8`
        //! - Mnemonic:  `ADD`
        //! - Size:      2 bytes
        //! - Binary:    `0xE8`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Add signed offset r8 to sp.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        if self.sp > (u16::MAX - r8 as u16) {
            self.f |= Flag::Carry as u8;
        }

        // Update register sp
        self.sp += r8 as u16;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB18(&mut self) {
        //! - Prototype: `RR B`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB18`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DEC_0x2B(&mut self) {
        //! - Prototype: `DEC HL`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x2B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x2C(&mut self) {
        //! - Prototype: `INC L`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x2C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x2A(&mut self) {
        //! - Prototype: `LD A, (HL+)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x2A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value pointed to by hl into a, then increment hl.

        // Load (hl) in a
        let mut hl: u16 = ((self.h as u16) << 8) + self.l as u16;
        self.a = self.mmu.read8(hl);
        // Increment hl
        hl += 1;
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CPL_0x2F(&mut self) {
        //! - Prototype: `CPL `
        //! - Mnemonic:  `CPL`
        //! - Size:      1 byte
        //! - Binary:    `0x2F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force set (1)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are inverted (one's complement).

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x2D(&mut self) {
        //! - Prototype: `DEC L`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x2D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from l.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x2E(&mut self) {
        //! - Prototype: `LD L, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x2E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into l.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.l = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB19(&mut self) {
        //! - Prototype: `RR C`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB19`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2F(&mut self) {
        //! - Prototype: `SRA A`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of a are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2E(&mut self) {
        //! - Prototype: `SRA (HL)`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of (hl) are shifted right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and the previous
        //!   contents of bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2D(&mut self) {
        //! - Prototype: `SRA L`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of l are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2C(&mut self) {
        //! - Prototype: `SRA H`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of h are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2B(&mut self) {
        //! - Prototype: `SRA E`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of e are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2A(&mut self) {
        //! - Prototype: `SRA D`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB2A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of d are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB27(&mut self) {
        //! - Prototype: `SLA A`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB27`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB26(&mut self) {
        //! - Prototype: `SLA (HL)`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB26`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are shifted left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and a zero is put
        //!   into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB25(&mut self) {
        //! - Prototype: `SLA L`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB25`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB24(&mut self) {
        //! - Prototype: `SLA H`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB24`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB23(&mut self) {
        //! - Prototype: `SLA E`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB23`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB22(&mut self) {
        //! - Prototype: `SLA D`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB22`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB21(&mut self) {
        //! - Prototype: `SLA C`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB21`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB20(&mut self) {
        //! - Prototype: `SLA B`
        //! - Mnemonic:  `SLA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB20`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB29(&mut self) {
        //! - Prototype: `SRA C`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB29`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of c are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB28(&mut self) {
        //! - Prototype: `SRA B`
        //! - Mnemonic:  `SRA`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB28`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of b are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x68(&mut self) {
        //! - Prototype: `LD L, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x68`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into l.

        self.l = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x69(&mut self) {
        //! - Prototype: `LD L, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x69`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into l.

        self.l = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x66(&mut self) {
        //! - Prototype: `LD H, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x66`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into h.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.h = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x67(&mut self) {
        //! - Prototype: `LD H, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x67`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into h.

        self.h = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x64(&mut self) {
        //! - Prototype: `LD H, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x64`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into h.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x65(&mut self) {
        //! - Prototype: `LD H, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x65`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into h.

        self.h = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x62(&mut self) {
        //! - Prototype: `LD H, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x62`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into h.

        self.h = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x63(&mut self) {
        //! - Prototype: `LD H, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x63`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into h.

        self.h = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x60(&mut self) {
        //! - Prototype: `LD H, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x60`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into h.

        self.h = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x61(&mut self) {
        //! - Prototype: `LD H, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x61`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into h.

        self.h = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6F(&mut self) {
        //! - Prototype: `LD L, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into l.

        self.l = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6D(&mut self) {
        //! - Prototype: `LD L, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into l.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6E(&mut self) {
        //! - Prototype: `LD L, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into l.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.l = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6B(&mut self) {
        //! - Prototype: `LD L, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into l.

        self.l = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6C(&mut self) {
        //! - Prototype: `LD L, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into l.

        self.l = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6A(&mut self) {
        //! - Prototype: `LD L, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x6A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into l.

        self.l = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB6C(&mut self) {
        //! - Prototype: `BIT 5, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6B(&mut self) {
        //! - Prototype: `BIT 5, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6A(&mut self) {
        //! - Prototype: `BIT 5, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6F(&mut self) {
        //! - Prototype: `BIT 5, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6E(&mut self) {
        //! - Prototype: `BIT 5, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6D(&mut self) {
        //! - Prototype: `BIT 5, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB6D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDA(&mut self) {
        //! - Prototype: `SET 3, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of d.

        self.d |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDC(&mut self) {
        //! - Prototype: `SET 3, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of h.

        self.h |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDB(&mut self) {
        //! - Prototype: `SET 3, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of e.

        self.e |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDE(&mut self) {
        //! - Prototype: `SET 3, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00001000;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDD(&mut self) {
        //! - Prototype: `SET 3, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDD`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of l.

        self.l |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDF(&mut self) {
        //! - Prototype: `SET 3, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBDF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of a.

        self.a |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADD_0x84(&mut self) {
        //! - Prototype: `ADD A, H`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x84`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds h to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.h) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.h;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xDA(&mut self) {
        //! - Prototype: `JP C, a16`
        //! - Mnemonic:  `JP`
        //! - Size:      3 bytes
        //! - Binary:    `0xDA`
        //! - Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_ADD_0x86(&mut self) {
        //! - Prototype: `ADD A, (HL)`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x86`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds (hl) to a.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);

        // Check if there will be an overflow
        if self.a > (u8::MAX - hl) {
            self.f |= Flag::Carry as u8;
        }

        self.a += hl;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xDC(&mut self) {
        //! - Prototype: `CALL C, a16`
        //! - Mnemonic:  `CALL`
        //! - Size:      3 bytes
        //! - Binary:    `0xDC`
        //! - Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_ADD_0x80(&mut self) {
        //! - Prototype: `ADD A, B`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x80`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds b to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.b) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.b;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x81(&mut self) {
        //! - Prototype: `ADD A, C`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x81`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds c to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.c) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.c;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x82(&mut self) {
        //! - Prototype: `ADD A, D`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x82`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds d to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.d) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.d;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x83(&mut self) {
        //! - Prototype: `ADD A, E`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x83`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds e to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > (u8::MAX - self.e) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.e;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RR_0xCB1F(&mut self) {
        //! - Prototype: `RR A`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1D(&mut self) {
        //! - Prototype: `RR L`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1E(&mut self) {
        //! - Prototype: `RR (HL)`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are rotated right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and the previous
        //!   contents of the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1B(&mut self) {
        //! - Prototype: `RR E`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1C(&mut self) {
        //! - Prototype: `RR H`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1A(&mut self) {
        //! - Prototype: `RR D`
        //! - Mnemonic:  `RR`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB1A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RET_0xD0(&mut self) {
        //! - Prototype: `RET NC`
        //! - Mnemonic:  `RET`
        //! - Size:      1 byte
        //! - Binary:    `0xD0`
        //! - Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8E(&mut self) {
        //! - Prototype: `ADC A, (HL)`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds (hl) and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8F(&mut self) {
        //! - Prototype: `ADC A, A`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds a and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xD4(&mut self) {
        //! - Prototype: `CALL NC, a16`
        //! - Mnemonic:  `CALL`
        //! - Size:      3 bytes
        //! - Binary:    `0xD4`
        //! - Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_ADC_0x8A(&mut self) {
        //! - Prototype: `ADC A, D`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds d and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8B(&mut self) {
        //! - Prototype: `ADC A, E`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds e and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8C(&mut self) {
        //! - Prototype: `ADC A, H`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds h and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RL_0xCB16(&mut self) {
        //! - Prototype: `RL (HL)`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB16`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are rotated left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and the previous
        //!   contents of the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB17(&mut self) {
        //! - Prototype: `RL A`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB17`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB14(&mut self) {
        //! - Prototype: `RL H`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB14`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB15(&mut self) {
        //! - Prototype: `RL L`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB15`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB12(&mut self) {
        //! - Prototype: `RL D`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB12`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB13(&mut self) {
        //! - Prototype: `RL E`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB13`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB10(&mut self) {
        //! - Prototype: `RL B`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB10`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB11(&mut self) {
        //! - Prototype: `RL C`
        //! - Mnemonic:  `RL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB11`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD1(&mut self) {
        //! - Prototype: `SET 2, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of c.

        self.c |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD0(&mut self) {
        //! - Prototype: `SET 2, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of b.

        self.b |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD3(&mut self) {
        //! - Prototype: `SET 2, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of e.

        self.e |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD2(&mut self) {
        //! - Prototype: `SET 2, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of d.

        self.d |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD5(&mut self) {
        //! - Prototype: `SET 2, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of l.

        self.l |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD4(&mut self) {
        //! - Prototype: `SET 2, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of h.

        self.h |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD7(&mut self) {
        //! - Prototype: `SET 2, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of a.

        self.a |= 0b00000100;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD6(&mut self) {
        //! - Prototype: `SET 2, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 2 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00000100;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD9(&mut self) {
        //! - Prototype: `SET 3, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of c.

        self.c |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD8(&mut self) {
        //! - Prototype: `SET 3, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBD8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 3 of b.

        self.b |= 0b00001000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB69(&mut self) {
        //! - Prototype: `BIT 5, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB69`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB68(&mut self) {
        //! - Prototype: `BIT 5, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB68`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 5 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB63(&mut self) {
        //! - Prototype: `BIT 4, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB63`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB62(&mut self) {
        //! - Prototype: `BIT 4, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB62`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB61(&mut self) {
        //! - Prototype: `BIT 4, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB61`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB60(&mut self) {
        //! - Prototype: `BIT 4, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB60`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB67(&mut self) {
        //! - Prototype: `BIT 4, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB67`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB66(&mut self) {
        //! - Prototype: `BIT 4, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB66`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB65(&mut self) {
        //! - Prototype: `BIT 4, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB65`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB64(&mut self) {
        //! - Prototype: `BIT 4, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB64`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 4 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB85(&mut self) {
        //! - Prototype: `RES 0, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB85`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB84(&mut self) {
        //! - Prototype: `RES 0, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB84`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB87(&mut self) {
        //! - Prototype: `RES 0, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB87`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB86(&mut self) {
        //! - Prototype: `RES 0, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB86`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB81(&mut self) {
        //! - Prototype: `RES 0, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB81`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB80(&mut self) {
        //! - Prototype: `RES 0, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB80`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB83(&mut self) {
        //! - Prototype: `RES 0, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB83`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB82(&mut self) {
        //! - Prototype: `RES 0, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB82`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 0 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB89(&mut self) {
        //! - Prototype: `RES 1, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB89`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB88(&mut self) {
        //! - Prototype: `RES 1, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB88`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x3E(&mut self) {
        //! - Prototype: `LD A, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x3E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into a.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.a = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DEC_0x3D(&mut self) {
        //! - Prototype: `DEC A`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x3D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CCF_0x3F(&mut self) {
        //! - Prototype: `CCF `
        //! - Mnemonic:  `CCF`
        //! - Size:      1 byte
        //! - Binary:    `0x3F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Inverts the carry flag.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x3A(&mut self) {
        //! - Prototype: `LD A, (HL-)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x3A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value pointed to by (hl) into a. Then decrement hl.

        // Load (hl) in a
        let mut hl: u16 = ((self.h as u16) << 8) + self.l as u16;
        self.a = self.mmu.read8(hl);
        // Decrement hl
        hl -= 1;
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x3C(&mut self) {
        //! - Prototype: `INC A`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x3C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x3B(&mut self) {
        //! - Prototype: `DEC SP`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x3B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x87(&mut self) {
        //! - Prototype: `ADD A, A`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x87`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds a to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Check if there will be an overflow
        if self.a > u8::MAX/2 {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += self.a;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0xDE(&mut self) {
        //! - Prototype: `SBC A, d8`
        //! - Mnemonic:  `SBC`
        //! - Size:      2 bytes
        //! - Binary:    `0xDE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts * and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RST_0xDF(&mut self) {
        //! - Prototype: `RST 18H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xDF`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 18h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x18;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_DEC_0x35(&mut self) {
        //! - Prototype: `DEC (HL)`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x35`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from (hl).

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x34(&mut self) {
        //! - Prototype: `INC (HL)`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x34`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SCF_0x37(&mut self) {
        //! - Prototype: `SCF `
        //! - Mnemonic:  `SCF`
        //! - Size:      1 byte
        //! - Binary:    `0x37`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force set (1)
        //! - Description
        //!   Sets the carry flag.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f |= Flag::Carry as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x36(&mut self) {
        //! - Prototype: `LD (HL), d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x36`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into (hl).

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, d8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x31(&mut self) {
        //! - Prototype: `LD SP, d16`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0x31`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads ** into sp.

        let d16: u16 = self.mmu.read16(self.pc + 1);
        self.sp = d16;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_JR_0x30(&mut self) {
        //! - Prototype: `JR NC, r8`
        //! - Mnemonic:  `JR`
        //! - Size:      2 bytes
        //! - Binary:    `0x30`
        //! - Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        if self.f & (Flag::Carry as u8) == 0 {
            // Update clocks
            self.m += 3;
            self.t += 12;
            // Update program counter
            if r8 < 0 {
                self.pc -= -r8 as u16;
            } else {
                self.pc += r8 as u16;
            }
        } else {
            // Update clocks
            self.m += 2;
            self.t += 8;
            // Update program counter
            self.pc += 2;
        }
    }

    pub fn instr_INC_0x33(&mut self) {
        //! - Prototype: `INC SP`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x33`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x32(&mut self) {
        //! - Prototype: `LD (HL-), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x32`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Stores a into (hl), then decrement hl.

        // Store a in (hl)
        let mut hl: u16 = ((self.h as u16) << 8) + self.l as u16;
        self.mmu.write8(hl, self.a);
        // Decrement hl
        hl += 1;
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x39(&mut self) {
        //! - Prototype: `ADD HL, SP`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x39`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The value of sp is added to hl.

        // Update registers h and l
        let lhs: u16 = ((self.h as u16) << 8) + self.l as u16;
        let rhs: u16 = lhs;
        let res: u16 = rhs + lhs;
        self.h = (res >> 8) as u8;
        self.l = res as u8;

        // Update flags
        if res==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        if lhs > (u16::MAX - rhs) {
            self.f |= Flag::Carry as u8;
        }

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x38(&mut self) {
        //! - Prototype: `JR C, r8`
        //! - Mnemonic:  `JR`
        //! - Size:      2 bytes
        //! - Binary:    `0x38`
        //! - Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        if self.f & (Flag::Carry as u8) != 0 {
            // Update clocks
            self.m += 3;
            self.t += 12;
            // Update program counter
            if r8 < 0 {
                self.pc -= -r8 as u16;
            } else {
                self.pc += r8 as u16;
            }
        } else {
            // Update clocks
            self.m += 2;
            self.t += 8;
            // Update program counter
            self.pc += 2;
        }
    }

    pub fn instr_RES_0xCB8E(&mut self) {
        //! - Prototype: `RES 1, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8D(&mut self) {
        //! - Prototype: `RES 1, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8F(&mut self) {
        //! - Prototype: `RES 1, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8A(&mut self) {
        //! - Prototype: `RES 1, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8C(&mut self) {
        //! - Prototype: `RES 1, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8B(&mut self) {
        //! - Prototype: `RES 1, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB8B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 1 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADC_0x88(&mut self) {
        //! - Prototype: `ADC A, B`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x88`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds b and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x89(&mut self) {
        //! - Prototype: `ADC A, C`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x89`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds c and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8D(&mut self) {
        //! - Prototype: `ADC A, L`
        //! - Mnemonic:  `ADC`
        //! - Size:      1 byte
        //! - Binary:    `0x8D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds l and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_POP_0xD1(&mut self) {
        //! - Prototype: `POP DE`
        //! - Mnemonic:  `POP`
        //! - Size:      1 byte
        //! - Binary:    `0xD1`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The memory location pointed to by sp is stored into e and sp is
        //!   incremented. The memory location pointed to by sp is stored into d
        //!   and sp is incremented again.

        // Pop DE from stack
        self.e = self.mmu.read8(self.sp);
        self.sp += 1;
        self.d = self.mmu.read8(self.sp);
        self.sp += 1;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xD2(&mut self) {
        //! - Prototype: `JP NC, a16`
        //! - Mnemonic:  `JP`
        //! - Size:      3 bytes
        //! - Binary:    `0xD2`
        //! - Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_PUSH_0xD5(&mut self) {
        //! - Prototype: `PUSH DE`
        //! - Mnemonic:  `PUSH`
        //! - Size:      1 byte
        //! - Binary:    `0xD5`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   sp is decremented and d is stored into the memory location pointed
        //!   to by sp. sp is decremented again and e is stored into the memory
        //!   location pointed to by sp.

        // Push DE on the stack
        self.sp -= 1;
        self.mmu.write8(self.sp, self.d);
        self.sp -= 1;
        self.mmu.write8(self.sp, self.e);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0xD6(&mut self) {
        //! - Prototype: `SUB d8`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0xD6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts * from a.

        let d8: u8 = self.mmu.read8(self.pc + 1);

        // Update flags
        if self.a==d8 {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<d8 {
            self.f |= Flag::Carry as u8;
        }

        self.a -= d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RST_0xD7(&mut self) {
        //! - Prototype: `RST 10H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xD7`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 10h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x10;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_RET_0xD8(&mut self) {
        //! - Prototype: `RET C`
        //! - Mnemonic:  `RET`
        //! - Size:      1 byte
        //! - Binary:    `0xD8`
        //! - Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RETI_0xD9(&mut self) {
        //! - Prototype: `RETI `
        //! - Mnemonic:  `RETI`
        //! - Size:      1 byte
        //! - Binary:    `0xD9`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Exchanges the 16-bit contents of bc, de, and hl with bc', de', and
        //!   hl'.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xA9(&mut self) {
        //! - Prototype: `XOR C`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xA9`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with c.

        self.a ^= self.c;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xA8(&mut self) {
        //! - Prototype: `XOR B`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xA8`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with b.

        self.a ^= self.b;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA7(&mut self) {
        //! - Prototype: `AND A`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA7`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with a.

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA6(&mut self) {
        //! - Prototype: `AND (HL)`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with (hl).

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.a &= hl;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA5(&mut self) {
        //! - Prototype: `AND L`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA5`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with l.

        self.a &= self.l;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA4(&mut self) {
        //! - Prototype: `AND H`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA4`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with h.

        self.a &= self.h;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA3(&mut self) {
        //! - Prototype: `AND E`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA3`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with e.

        self.a &= self.e;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA2(&mut self) {
        //! - Prototype: `AND D`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA2`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with d.

        self.a &= self.d;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA1(&mut self) {
        //! - Prototype: `AND C`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA1`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with c.

        self.a &= self.c;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA0(&mut self) {
        //! - Prototype: `AND B`
        //! - Mnemonic:  `AND`
        //! - Size:      1 byte
        //! - Binary:    `0xA0`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise AND on a with b.

        self.a &= self.b;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBC8(&mut self) {
        //! - Prototype: `SET 1, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of b.

        self.b |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC9(&mut self) {
        //! - Prototype: `SET 1, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of c.

        self.c |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x7A(&mut self) {
        //! - Prototype: `LD A, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into a.

        self.a = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7C(&mut self) {
        //! - Prototype: `LD A, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into a.

        self.a = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7B(&mut self) {
        //! - Prototype: `LD A, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into a.

        self.a = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7E(&mut self) {
        //! - Prototype: `LD A, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into a.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.a = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7D(&mut self) {
        //! - Prototype: `LD A, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into a.

        self.a = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7F(&mut self) {
        //! - Prototype: `LD A, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x7F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into a.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB52(&mut self) {
        //! - Prototype: `BIT 2, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB52`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB53(&mut self) {
        //! - Prototype: `BIT 2, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB53`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB50(&mut self) {
        //! - Prototype: `BIT 2, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB50`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB51(&mut self) {
        //! - Prototype: `BIT 2, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB51`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB56(&mut self) {
        //! - Prototype: `BIT 2, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB56`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB57(&mut self) {
        //! - Prototype: `BIT 2, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB57`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB54(&mut self) {
        //! - Prototype: `BIT 2, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB54`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB55(&mut self) {
        //! - Prototype: `BIT 2, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB55`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 2 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC0(&mut self) {
        //! - Prototype: `SET 0, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of b.

        self.b |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC1(&mut self) {
        //! - Prototype: `SET 0, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of c.

        self.c |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB58(&mut self) {
        //! - Prototype: `BIT 3, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB58`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC3(&mut self) {
        //! - Prototype: `SET 0, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of e.

        self.e |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC4(&mut self) {
        //! - Prototype: `SET 0, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of h.

        self.h |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC5(&mut self) {
        //! - Prototype: `SET 0, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of l.

        self.l |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC6(&mut self) {
        //! - Prototype: `SET 0, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00000001;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC7(&mut self) {
        //! - Prototype: `SET 0, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of a.

        self.a |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5B(&mut self) {
        //! - Prototype: `BIT 3, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5C(&mut self) {
        //! - Prototype: `BIT 3, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5A(&mut self) {
        //! - Prototype: `BIT 3, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5F(&mut self) {
        //! - Prototype: `BIT 3, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5D(&mut self) {
        //! - Prototype: `BIT 3, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5E(&mut self) {
        //! - Prototype: `BIT 3, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB5E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCA(&mut self) {
        //! - Prototype: `SET 1, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of d.

        self.d |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCB(&mut self) {
        //! - Prototype: `SET 1, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of e.

        self.e |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCC(&mut self) {
        //! - Prototype: `SET 1, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of h.

        self.h |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCD(&mut self) {
        //! - Prototype: `SET 1, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCD`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of l.

        self.l |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCE(&mut self) {
        //! - Prototype: `SET 1, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00000010;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCF(&mut self) {
        //! - Prototype: `SET 1, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBCF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 1 of a.

        self.a |= 0b00000010;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x79(&mut self) {
        //! - Prototype: `LD A, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x79`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into a.

        self.a = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x78(&mut self) {
        //! - Prototype: `LD A, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x78`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into a.

        self.a = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x71(&mut self) {
        //! - Prototype: `LD (HL), C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x71`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.c);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x70(&mut self) {
        //! - Prototype: `LD (HL), B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x70`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.b);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x73(&mut self) {
        //! - Prototype: `LD (HL), E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x73`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.e);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x72(&mut self) {
        //! - Prototype: `LD (HL), D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x72`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.d);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x75(&mut self) {
        //! - Prototype: `LD (HL), L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x75`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.l);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x74(&mut self) {
        //! - Prototype: `LD (HL), H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x74`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.h);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x77(&mut self) {
        //! - Prototype: `LD (HL), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x77`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into (hl).

        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, self.a);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_HALT_0x76(&mut self) {
        //! - Prototype: `HALT `
        //! - Mnemonic:  `HALT`
        //! - Size:      1 byte
        //! - Binary:    `0x76`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Suspends CPU operation until an interrupt or reset occurs.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB59(&mut self) {
        //! - Prototype: `BIT 3, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB59`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 3 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_XOR_0xAF(&mut self) {
        //! - Prototype: `XOR A`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAF`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with a.

        self.a = 0;

        // Update flags
        self.f |= Flag::Zero as u8;
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAE(&mut self) {
        //! - Prototype: `XOR (HL)`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with (hl).

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.a ^= hl;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAD(&mut self) {
        //! - Prototype: `XOR L`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAD`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with l.

        self.a ^= self.l;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAC(&mut self) {
        //! - Prototype: `XOR H`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAC`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with h.

        self.a ^= self.h;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAB(&mut self) {
        //! - Prototype: `XOR E`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAB`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with e.

        self.a ^= self.e;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAA(&mut self) {
        //! - Prototype: `XOR D`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xAA`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with d.

        self.a ^= self.d;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9F(&mut self) {
        //! - Prototype: `SBC A, A`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts a and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9E(&mut self) {
        //! - Prototype: `SBC A, (HL)`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts (hl) and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9D(&mut self) {
        //! - Prototype: `SBC A, L`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts l and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9C(&mut self) {
        //! - Prototype: `SBC A, H`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts h and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9B(&mut self) {
        //! - Prototype: `SBC A, E`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts e and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9A(&mut self) {
        //! - Prototype: `SBC A, D`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x9A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts d and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x08(&mut self) {
        //! - Prototype: `LD (a16), SP`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0x08`
        //! - Cycles:    20 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Exchanges the 16-bit contents of af and af'.

        let a16: u16 = self.mmu.read16(self.pc + 1);
        self.mmu.write16(a16, self.sp);

        // Update clocks
        self.m += 5;
        self.t += 20;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_ADD_0x09(&mut self) {
        //! - Prototype: `ADD HL, BC`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x09`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The value of bc is added to hl.

        // Update registers h and l
        let lhs: u16 = ((self.h as u16) << 8) + self.l as u16;
        let rhs: u16 = ((self.b as u16) << 8) + self.c as u16;
        let res: u16 = rhs + lhs;
        self.h = (res >> 8) as u8;
        self.l = res as u8;

        // Update flags
        if res==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        if lhs > (u16::MAX - rhs) {
            self.f |= Flag::Carry as u8;
        }

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x04(&mut self) {
        //! - Prototype: `INC B`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x04`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x05(&mut self) {
        //! - Prototype: `DEC B`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x05`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from b.

        self.b -= 1;

        // Update flags
        if self.b==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x06(&mut self) {
        //! - Prototype: `LD B, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x06`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into b.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.b = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLCA_0x07(&mut self) {
        //! - Prototype: `RLCA `
        //! - Mnemonic:  `RLCA`
        //! - Size:      1 byte
        //! - Binary:    `0x07`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_NOP_0x00(&mut self) {
        //! - Prototype: `NOP `
        //! - Mnemonic:  `NOP`
        //! - Size:      1 byte
        //! - Binary:    `0x00`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   No operation is performed.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x01(&mut self) {
        //! - Prototype: `LD BC, d16`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0x01`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads ** into bc.

        let d16: u16 = self.mmu.read16(self.pc + 1);
        self.b = (d16 >> 8) as u8;
        self.c = d16 as u8;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_LD_0x02(&mut self) {
        //! - Prototype: `LD (BC), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x02`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Stores a into the memory location pointed to by bc.

        let bc: u16 = ((self.b as u16) << 8) + self.c as u16;
        self.mmu.write8(bc, self.a);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x03(&mut self) {
        //! - Prototype: `INC BC`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x03`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x0D(&mut self) {
        //! - Prototype: `DEC C`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x0D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from c.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x0E(&mut self) {
        //! - Prototype: `LD C, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x0E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into c.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.c = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRCA_0x0F(&mut self) {
        //! - Prototype: `RRCA `
        //! - Mnemonic:  `RRCA`
        //! - Size:      1 byte
        //! - Binary:    `0x0F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x0A(&mut self) {
        //! - Prototype: `LD A, (BC)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x0A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value pointed to by bc into a.

        let bc: u16 = ((self.b as u16) << 8) + self.c as u16;
        self.a = self.mmu.read8(bc);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x0B(&mut self) {
        //! - Prototype: `DEC BC`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x0B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x0C(&mut self) {
        //! - Prototype: `INC C`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x0C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x97(&mut self) {
        //! - Prototype: `SUB A`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x97`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts a from a.

        self.a = 0;

        // Update flags
        self.f |= Flag::Zero as u8;
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x96(&mut self) {
        //! - Prototype: `SUB (HL)`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x96`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts (hl) from a.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);

        // Update flags
        if self.a==hl {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<hl {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x95(&mut self) {
        //! - Prototype: `SUB L`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x95`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts l from a.

        // Update flags
        if self.a==self.l {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.l {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x94(&mut self) {
        //! - Prototype: `SUB H`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x94`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts h from a.

        // Update flags
        if self.a==self.h {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.h {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x93(&mut self) {
        //! - Prototype: `SUB E`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x93`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts e from a.

        // Update flags
        if self.a==self.e {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.e {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x92(&mut self) {
        //! - Prototype: `SUB D`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x92`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts d from a.

        // Update flags
        if self.a==self.d {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.d {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x91(&mut self) {
        //! - Prototype: `SUB C`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x91`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts c from a.

        // Update flags
        if self.a==self.c {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.c {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x90(&mut self) {
        //! - Prototype: `SUB B`
        //! - Mnemonic:  `SUB`
        //! - Size:      1 byte
        //! - Binary:    `0x90`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts b from a.

        // Update flags
        if self.a==self.b {
            self.f |= Flag::Zero as u8;
        }
        self.f |= Flag::Operation as u8;
        if self.a<self.b {
            self.f |= Flag::Carry as u8;
        }

        // Update register a
        self.a -= self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x99(&mut self) {
        //! - Prototype: `SBC A, C`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x99`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts c and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x98(&mut self) {
        //! - Prototype: `SBC A, B`
        //! - Mnemonic:  `SBC`
        //! - Size:      1 byte
        //! - Binary:    `0x98`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts b and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0xE2(&mut self) {
        //! - Prototype: `LD (C), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0xE2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        self.mmu.write8(self.c as u16 + 0xFF00u16, self.a);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_POP_0xE1(&mut self) {
        //! - Prototype: `POP HL`
        //! - Mnemonic:  `POP`
        //! - Size:      1 byte
        //! - Binary:    `0xE1`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The memory location pointed to by sp is stored into l and sp is
        //!   incremented. The memory location pointed to by sp is stored into h
        //!   and sp is incremented again.

        // Pop HL from stack
        self.l = self.mmu.read8(self.sp);
        self.sp += 1;
        self.h = self.mmu.read8(self.sp);
        self.sp += 1;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LDH_0xE0(&mut self) {
        //! - Prototype: `LDH (a8), A`
        //! - Mnemonic:  `LDH`
        //! - Size:      2 bytes
        //! - Binary:    `0xE0`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        let a8: u16 = self.mmu.read8(self.pc + 1) as u16 + 0xFF00u16;
        self.mmu.write8(a8, self.a);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB09(&mut self) {
        //! - Prototype: `RRC C`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB09`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB08(&mut self) {
        //! - Prototype: `RRC B`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB08`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xE5(&mut self) {
        //! - Prototype: `PUSH HL`
        //! - Mnemonic:  `PUSH`
        //! - Size:      1 byte
        //! - Binary:    `0xE5`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   sp is decremented and h is stored into the memory location pointed
        //!   to by sp. sp is decremented again and l is stored into the memory
        //!   location pointed to by sp.

        // Push HL on the stack
        self.sp -= 1;
        self.mmu.write8(self.sp, self.h);
        self.sp -= 1;
        self.mmu.write8(self.sp, self.l);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RLC_0xCB05(&mut self) {
        //! - Prototype: `RLC L`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB05`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB04(&mut self) {
        //! - Prototype: `RLC H`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB04`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB07(&mut self) {
        //! - Prototype: `RLC A`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB07`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB06(&mut self) {
        //! - Prototype: `RLC (HL)`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB06`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are rotated left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB01(&mut self) {
        //! - Prototype: `RLC C`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB01`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB00(&mut self) {
        //! - Prototype: `RLC B`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB00`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB03(&mut self) {
        //! - Prototype: `RLC E`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB03`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB02(&mut self) {
        //! - Prototype: `RLC D`
        //! - Mnemonic:  `RLC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB02`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE5(&mut self) {
        //! - Prototype: `SET 4, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of l.

        self.l |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xEA(&mut self) {
        //! - Prototype: `LD (a16), A`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0xEA`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        let a16: u16 = self.mmu.read16(self.pc + 1);
        self.mmu.write8(a16, self.a);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_RST_0xEF(&mut self) {
        //! - Prototype: `RST 28H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xEF`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 28h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x28;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_XOR_0xEE(&mut self) {
        //! - Prototype: `XOR d8`
        //! - Mnemonic:  `XOR`
        //! - Size:      1 byte
        //! - Binary:    `0xEE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise XOR on a with *.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.a ^= d8;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0E(&mut self) {
        //! - Prototype: `RRC (HL)`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are rotated right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0D(&mut self) {
        //! - Prototype: `RRC L`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0F(&mut self) {
        //! - Prototype: `RRC A`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0A(&mut self) {
        //! - Prototype: `RRC D`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0C(&mut self) {
        //! - Prototype: `RRC H`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0B(&mut self) {
        //! - Prototype: `RRC E`
        //! - Mnemonic:  `RRC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB0B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x40(&mut self) {
        //! - Prototype: `LD B, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x40`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into b.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x41(&mut self) {
        //! - Prototype: `LD B, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x41`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into b.

        self.b = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x42(&mut self) {
        //! - Prototype: `LD B, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x42`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into b.

        self.b = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x43(&mut self) {
        //! - Prototype: `LD B, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x43`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into b.

        self.b = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x44(&mut self) {
        //! - Prototype: `LD B, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x44`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into b.

        self.b = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x45(&mut self) {
        //! - Prototype: `LD B, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x45`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into b.

        self.b = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x46(&mut self) {
        //! - Prototype: `LD B, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x46`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into b.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.b = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x47(&mut self) {
        //! - Prototype: `LD B, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x47`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into b.

        self.b = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x48(&mut self) {
        //! - Prototype: `LD C, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x48`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into c.

        self.c = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x49(&mut self) {
        //! - Prototype: `LD C, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x49`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into c.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4A(&mut self) {
        //! - Prototype: `LD C, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into c.

        self.c = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4B(&mut self) {
        //! - Prototype: `LD C, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into c.

        self.c = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4C(&mut self) {
        //! - Prototype: `LD C, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into c.

        self.c = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4D(&mut self) {
        //! - Prototype: `LD C, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into c.

        self.c = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4E(&mut self) {
        //! - Prototype: `LD C, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into c.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.c = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4F(&mut self) {
        //! - Prototype: `LD C, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x4F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into c.

        self.c = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB4A(&mut self) {
        //! - Prototype: `BIT 1, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4C(&mut self) {
        //! - Prototype: `BIT 1, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4B(&mut self) {
        //! - Prototype: `BIT 1, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4E(&mut self) {
        //! - Prototype: `BIT 1, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4D(&mut self) {
        //! - Prototype: `BIT 1, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4F(&mut self) {
        //! - Prototype: `BIT 1, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB4F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBF(&mut self) {
        //! - Prototype: `RES 7, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBE(&mut self) {
        //! - Prototype: `RES 7, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBD(&mut self) {
        //! - Prototype: `RES 7, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBD`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBC(&mut self) {
        //! - Prototype: `RES 7, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBB(&mut self) {
        //! - Prototype: `RES 7, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBA(&mut self) {
        //! - Prototype: `RES 7, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBBA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_CP_0xFE(&mut self) {
        //! - Prototype: `CP d8`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xFE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts * from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBB(&mut self) {
        //! - Prototype: `CP E`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBB`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts e from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBC(&mut self) {
        //! - Prototype: `CP H`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBC`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts h from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBA(&mut self) {
        //! - Prototype: `CP D`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBA`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts d from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBF(&mut self) {
        //! - Prototype: `CP A`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBF`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts a from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBD(&mut self) {
        //! - Prototype: `CP L`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBD`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts l from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBE(&mut self) {
        //! - Prototype: `CP (HL)`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xBE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts (hl) from a and affects flags according to the result. a
        //!   is not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xB8(&mut self) {
        //! - Prototype: `CP B`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xB8`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts b from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xB9(&mut self) {
        //! - Prototype: `CP C`
        //! - Mnemonic:  `CP`
        //! - Size:      1 byte
        //! - Binary:    `0xB9`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Subtracts c from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB2(&mut self) {
        //! - Prototype: `OR D`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB2`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with d.

        self.a |= self.d;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB3(&mut self) {
        //! - Prototype: `OR E`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB3`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with e.

        self.a |= self.e;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB0(&mut self) {
        //! - Prototype: `OR B`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB0`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with b.

        self.a |= self.b;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB1(&mut self) {
        //! - Prototype: `OR C`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB1`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with c.

        self.a |= self.c;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB6(&mut self) {
        //! - Prototype: `OR (HL)`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with (hl).

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.a |= hl;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB7(&mut self) {
        //! - Prototype: `OR A`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB7`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with a.

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB4(&mut self) {
        //! - Prototype: `OR H`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB4`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with h.

        self.a |= self.h;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB5(&mut self) {
        //! - Prototype: `OR L`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xB5`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with l.

        self.a |= self.l;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBB7(&mut self) {
        //! - Prototype: `RES 6, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB6(&mut self) {
        //! - Prototype: `RES 6, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB5(&mut self) {
        //! - Prototype: `RES 6, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB4(&mut self) {
        //! - Prototype: `RES 6, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB3(&mut self) {
        //! - Prototype: `RES 6, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB2(&mut self) {
        //! - Prototype: `RES 6, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB1(&mut self) {
        //! - Prototype: `RES 6, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB0(&mut self) {
        //! - Prototype: `RES 6, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 6 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_EI_0xFB(&mut self) {
        //! - Prototype: `EI `
        //! - Mnemonic:  `EI`
        //! - Size:      1 byte
        //! - Binary:    `0xFB`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets both interrupt flip-flops, thus allowing maskable interrupts
        //!   to occur. An interrupt will not occur until after the immediatedly
        //!   following instruction.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBB9(&mut self) {
        //! - Prototype: `RES 7, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB8(&mut self) {
        //! - Prototype: `RES 7, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBB8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 7 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB41(&mut self) {
        //! - Prototype: `BIT 0, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB41`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB40(&mut self) {
        //! - Prototype: `BIT 0, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB40`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB43(&mut self) {
        //! - Prototype: `BIT 0, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB43`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB42(&mut self) {
        //! - Prototype: `BIT 0, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB42`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB45(&mut self) {
        //! - Prototype: `BIT 0, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB45`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB44(&mut self) {
        //! - Prototype: `BIT 0, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB44`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB47(&mut self) {
        //! - Prototype: `BIT 0, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB47`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB46(&mut self) {
        //! - Prototype: `BIT 0, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB46`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 0 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB49(&mut self) {
        //! - Prototype: `BIT 1, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB49`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB48(&mut self) {
        //! - Prototype: `BIT 1, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB48`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 1 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRA_0x1F(&mut self) {
        //! - Prototype: `RRA `
        //! - Mnemonic:  `RRA`
        //! - Size:      1 byte
        //! - Binary:    `0x1F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x1E(&mut self) {
        //! - Prototype: `LD E, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x1E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into e.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.e = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DEC_0x1D(&mut self) {
        //! - Prototype: `DEC E`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x1D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from e.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x1C(&mut self) {
        //! - Prototype: `INC E`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x1C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x1B(&mut self) {
        //! - Prototype: `DEC DE`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x1B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x1A(&mut self) {
        //! - Prototype: `LD A, (DE)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x1A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value pointed to by de into a.

        let de: u16 = ((self.d as u16) << 8) + self.e as u16;
        self.a = self.mmu.read8(de);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xF6(&mut self) {
        //! - Prototype: `OR d8`
        //! - Mnemonic:  `OR`
        //! - Size:      1 byte
        //! - Binary:    `0xF6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   Bitwise OR on a with *.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.a |= d8;

        // Update flags
        if self.a==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RST_0xF7(&mut self) {
        //! - Prototype: `RST 30H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xF7`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 30h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x30;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_SET_0xCBE2(&mut self) {
        //! - Prototype: `SET 4, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of d.

        self.d |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xF5(&mut self) {
        //! - Prototype: `PUSH AF`
        //! - Mnemonic:  `PUSH`
        //! - Size:      1 byte
        //! - Binary:    `0xF5`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   sp is decremented and a is stored into the memory location pointed
        //!   to by sp. sp is decremented again and f is stored into the memory
        //!   location pointed to by sp.

        // Push AF on the stack
        self.sp -= 1;
        self.mmu.write8(self.sp, self.a);
        self.sp -= 1;
        self.mmu.write8(self.sp, self.f);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB34(&mut self) {
        //! - Prototype: `SWAP H`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB34`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of h are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE0(&mut self) {
        //! - Prototype: `SET 4, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of b.

        self.b |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DI_0xF3(&mut self) {
        //! - Prototype: `DI `
        //! - Mnemonic:  `DI`
        //! - Size:      1 byte
        //! - Binary:    `0xF3`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets both interrupt flip-flops, thus prenting maskable
        //!   interrupts from triggering.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE1(&mut self) {
        //! - Prototype: `SET 4, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of c.

        self.c |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LDH_0xF0(&mut self) {
        //! - Prototype: `LDH A, (a8)`
        //! - Mnemonic:  `LDH`
        //! - Size:      2 bytes
        //! - Binary:    `0xF0`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        let a8: u16 = self.mmu.read8(self.pc + 1) as u16 + 0xFF00u16;
        self.a = self.mmu.read8(a8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE6(&mut self) {
        //! - Prototype: `SET 4, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00010000;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_POP_0xF1(&mut self) {
        //! - Prototype: `POP AF`
        //! - Mnemonic:  `POP`
        //! - Size:      1 byte
        //! - Binary:    `0xF1`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Set if appropriate
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The memory location pointed to by sp is stored into f and sp is
        //!   incremented. The memory location pointed to by sp is stored into a
        //!   and sp is incremented again.

        // Pop AF from stack
        self.f = self.mmu.read8(self.sp);
        self.sp += 1;
        self.a = self.mmu.read8(self.sp);
        self.sp += 1;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE7(&mut self) {
        //! - Prototype: `SET 4, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of a.

        self.a |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADD_0x19(&mut self) {
        //! - Prototype: `ADD HL, DE`
        //! - Mnemonic:  `ADD`
        //! - Size:      1 byte
        //! - Binary:    `0x19`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The value of de is added to hl.

        // Update registers h and l
        let lhs: u16 = ((self.h as u16) << 8) + self.l as u16;
        let rhs: u16 = ((self.d as u16) << 8) + self.e as u16;
        let res: u16 = rhs + lhs;
        self.h = (res >> 8) as u8;
        self.l = res as u8;

        // Update flags
        if res==0 {
            self.f |= Flag::Zero as u8;
        }
        self.f &= !(Flag::Operation as u8);
        if lhs > (u16::MAX - rhs) {
            self.f |= Flag::Carry as u8;
        }

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x18(&mut self) {
        //! - Prototype: `JR r8`
        //! - Mnemonic:  `JR`
        //! - Size:      2 bytes
        //! - Binary:    `0x18`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The signed value * is added to pc. The jump is measured from the
        //!   start of the instruction opcode.

        // Update program counter
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        if r8 < 0 {
            self.pc -= -r8 as u16;
        } else {
            self.pc += r8 as u16;
        }

        // Update clocks
        self.m += 3;
        self.t += 12;
    }

    pub fn instr_RLA_0x17(&mut self) {
        //! - Prototype: `RLA `
        //! - Mnemonic:  `RLA`
        //! - Size:      1 byte
        //! - Binary:    `0x17`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x16(&mut self) {
        //! - Prototype: `LD D, d8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0x16`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads * into d.

        let d8: u8 = self.mmu.read8(self.pc + 1);
        self.d = d8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DEC_0x15(&mut self) {
        //! - Prototype: `DEC D`
        //! - Mnemonic:  `DEC`
        //! - Size:      1 byte
        //! - Binary:    `0x15`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force set (1)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Subtracts one from d.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x14(&mut self) {
        //! - Prototype: `INC D`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x14`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x13(&mut self) {
        //! - Prototype: `INC DE`
        //! - Mnemonic:  `INC`
        //! - Size:      1 byte
        //! - Binary:    `0x13`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Adds one to de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x12(&mut self) {
        //! - Prototype: `LD (DE), A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x12`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Stores a into the memory location pointed to by de.

        let de: u16 = ((self.d as u16) << 8) + self.e as u16;
        self.mmu.write8(de, self.a);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x11(&mut self) {
        //! - Prototype: `LD DE, d16`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0x11`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads ** into de.

        let d16: u16 = self.mmu.read16(self.pc + 1);
        self.d = (d16 >> 8) as u8;
        self.e = d16 as u8;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_STOP_0x10(&mut self) {
        //! - Prototype: `STOP 0`
        //! - Mnemonic:  `STOP`
        //! - Size:      1 byte
        //! - Binary:    `0x10`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   This is NOT DJNZ as on other Z80. This is equivalent to
        //!   HALT not waiting for interrupts.

        self.stop = true;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBFC(&mut self) {
        //! - Prototype: `SET 7, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of h.

        self.h |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFB(&mut self) {
        //! - Prototype: `SET 7, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of e.

        self.e |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFA(&mut self) {
        //! - Prototype: `SET 7, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of d.

        self.d |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFF(&mut self) {
        //! - Prototype: `SET 7, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of a.

        self.a |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFE(&mut self) {
        //! - Prototype: `SET 7, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b10000000;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFD(&mut self) {
        //! - Prototype: `SET 7, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBFD`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of l.

        self.l |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RST_0xFF(&mut self) {
        //! - Prototype: `RST 38H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xFF`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 38h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x38;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_SRL_0xCB3A(&mut self) {
        //! - Prototype: `SRL D`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of d are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3B(&mut self) {
        //! - Prototype: `SRL E`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of e are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3C(&mut self) {
        //! - Prototype: `SRL H`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of h are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3D(&mut self) {
        //! - Prototype: `SRL L`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of l are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3E(&mut self) {
        //! - Prototype: `SRL (HL)`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of (hl) are shifted right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and a zero is put
        //!   into bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3F(&mut self) {
        //! - Prototype: `SRL A`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB3F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of a are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xFA(&mut self) {
        //! - Prototype: `LD A, (a16)`
        //! - Mnemonic:  `LD`
        //! - Size:      3 bytes
        //! - Binary:    `0xFA`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        let a16: u16 = self.mmu.read16(self.pc + 1);
        self.a = self.mmu.read8(a16);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_SRL_0xCB38(&mut self) {
        //! - Prototype: `SRL B`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB38`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of b are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB39(&mut self) {
        //! - Prototype: `SRL C`
        //! - Mnemonic:  `SRL`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB39`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   The contents of c are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xF8(&mut self) {
        //! - Prototype: `LD HL, SP+r8`
        //! - Mnemonic:  `LD`
        //! - Size:      2 bytes
        //! - Binary:    `0xF8`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Force unset (0)
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        let mut spr8: u16 = self.sp;
        if r8 < 0 {
            spr8 -= -r8 as u16;
        } else {
            spr8 += r8 as u16;
        }
        self.h = (spr8 >> 8) as u8;
        self.l = spr8 as u8;

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xF9(&mut self) {
        //! - Prototype: `LD SP, HL`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0xF9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Loads the value of hl into sp.

        self.sp = ((self.h as u16) << 8) + self.l as u16;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB30(&mut self) {
        //! - Prototype: `SWAP B`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB30`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of b are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB31(&mut self) {
        //! - Prototype: `SWAP C`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB31`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of c are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB32(&mut self) {
        //! - Prototype: `SWAP D`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB32`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of d are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB33(&mut self) {
        //! - Prototype: `SWAP E`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB33`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of e are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xF2(&mut self) {
        //! - Prototype: `LD A, (C)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0xF2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        self.a = self.mmu.read8(self.c as u16 + 0xFF00u16);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB35(&mut self) {
        //! - Prototype: `SWAP L`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB35`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of l are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB36(&mut self) {
        //! - Prototype: `SWAP (HL)`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB36`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of (hl) are shifted left one bit position. The
        //!   contents of bit 7 are put into the carry flag and a one is put
        //!   into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB37(&mut self) {
        //! - Prototype: `SWAP A`
        //! - Mnemonic:  `SWAP`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB37`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force unset (0)
        //!   - `C`:  Force unset (0)
        //! - Description
        //!   The contents of a are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF9(&mut self) {
        //! - Prototype: `SET 7, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of c.

        self.c |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF8(&mut self) {
        //! - Prototype: `SET 7, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 7 of b.

        self.b |= 0b10000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF3(&mut self) {
        //! - Prototype: `SET 6, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of e.

        self.e |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF2(&mut self) {
        //! - Prototype: `SET 6, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of d.

        self.d |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF1(&mut self) {
        //! - Prototype: `SET 6, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of c.

        self.c |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF0(&mut self) {
        //! - Prototype: `SET 6, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of b.

        self.b |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF7(&mut self) {
        //! - Prototype: `SET 6, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of a.

        self.a |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF6(&mut self) {
        //! - Prototype: `SET 6, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b01000000;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF5(&mut self) {
        //! - Prototype: `SET 6, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of l.

        self.l |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF4(&mut self) {
        //! - Prototype: `SET 6, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBF4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 6 of h.

        self.h |= 0b01000000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x5C(&mut self) {
        //! - Prototype: `LD E, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5C`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into e.

        self.e = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5B(&mut self) {
        //! - Prototype: `LD E, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5B`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into e.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5A(&mut self) {
        //! - Prototype: `LD E, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5A`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into e.

        self.e = self.d;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5F(&mut self) {
        //! - Prototype: `LD E, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5F`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into e.

        self.e = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5E(&mut self) {
        //! - Prototype: `LD E, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5E`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into e.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.e = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5D(&mut self) {
        //! - Prototype: `LD E, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x5D`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into e.

        self.e = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBA8(&mut self) {
        //! - Prototype: `RES 5, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA9(&mut self) {
        //! - Prototype: `RES 5, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA6(&mut self) {
        //! - Prototype: `RES 4, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA6`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA7(&mut self) {
        //! - Prototype: `RES 4, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA7`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA4(&mut self) {
        //! - Prototype: `RES 4, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA5(&mut self) {
        //! - Prototype: `RES 4, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA5`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA2(&mut self) {
        //! - Prototype: `RES 4, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA3(&mut self) {
        //! - Prototype: `RES 4, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA0(&mut self) {
        //! - Prototype: `RES 4, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA0`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA1(&mut self) {
        //! - Prototype: `RES 4, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBA1`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 4 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAF(&mut self) {
        //! - Prototype: `RES 5, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAD(&mut self) {
        //! - Prototype: `RES 5, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAD`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAE(&mut self) {
        //! - Prototype: `RES 5, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAB(&mut self) {
        //! - Prototype: `RES 5, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAC(&mut self) {
        //! - Prototype: `RES 5, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAA(&mut self) {
        //! - Prototype: `RES 5, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBAA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 5 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x53(&mut self) {
        //! - Prototype: `LD D, E`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x53`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of e are loaded into d.

        self.d = self.e;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x52(&mut self) {
        //! - Prototype: `LD D, D`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x52`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of d are loaded into d.

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x51(&mut self) {
        //! - Prototype: `LD D, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x51`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into d.

        self.d = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x50(&mut self) {
        //! - Prototype: `LD D, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x50`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into d.

        self.d = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x57(&mut self) {
        //! - Prototype: `LD D, A`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x57`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of a are loaded into d.

        self.d = self.a;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x56(&mut self) {
        //! - Prototype: `LD D, (HL)`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x56`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of (hl) are loaded into d.

        let hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        self.d = hl;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x55(&mut self) {
        //! - Prototype: `LD D, L`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x55`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of l are loaded into d.

        self.d = self.l;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x54(&mut self) {
        //! - Prototype: `LD D, H`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x54`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of h are loaded into d.

        self.d = self.h;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x59(&mut self) {
        //! - Prototype: `LD E, C`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x59`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of c are loaded into e.

        self.e = self.c;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x58(&mut self) {
        //! - Prototype: `LD E, B`
        //! - Mnemonic:  `LD`
        //! - Size:      1 byte
        //! - Binary:    `0x58`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The contents of b are loaded into e.

        self.e = self.b;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBEB(&mut self) {
        //! - Prototype: `SET 5, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBEB`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of e.

        self.e |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEC(&mut self) {
        //! - Prototype: `SET 5, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBEC`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of h.

        self.h |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEA(&mut self) {
        //! - Prototype: `SET 5, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBEA`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of d.

        self.d |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEF(&mut self) {
        //! - Prototype: `SET 5, A`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBEF`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of a.

        self.a |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBED(&mut self) {
        //! - Prototype: `SET 5, L`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBED`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of l.

        self.l |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEE(&mut self) {
        //! - Prototype: `SET 5, (HL)`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBEE`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of (hl).

        let mut hl: u8 = self.mmu.read8(((self.h as u16) << 8) + self.l as u16);
        hl |= 0b00100000;
        self.mmu.write8(((self.h as u16) << 8) + self.l as u16, hl);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xC5(&mut self) {
        //! - Prototype: `PUSH BC`
        //! - Mnemonic:  `PUSH`
        //! - Size:      1 byte
        //! - Binary:    `0xC5`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   sp is decremented and b is stored into the memory location pointed
        //!   to by sp. sp is decremented again and c is stored into the memory
        //!   location pointed to by sp.

        // Push BC on the stack
        self.sp -= 1;
        self.mmu.write8(self.sp, self.b);
        self.sp -= 1;
        self.mmu.write8(self.sp, self.c);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xC4(&mut self) {
        //! - Prototype: `CALL NZ, a16`
        //! - Mnemonic:  `CALL`
        //! - Size:      3 bytes
        //! - Binary:    `0xC4`
        //! - Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_RST_0xC7(&mut self) {
        //! - Prototype: `RST 00H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xC7`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 00h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x00;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_ADD_0xC6(&mut self) {
        //! - Prototype: `ADD A, d8`
        //! - Mnemonic:  `ADD`
        //! - Size:      2 bytes
        //! - Binary:    `0xC6`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds * to a.

        // Reset flags
        self.f = Flag::None as u8;

        // Retrieve immediate value
        let d8: u8 = self.mmu.read8(self.pc + 1);

        // Check if there will be an overflow
        if self.a > (u8::MAX - d8) {
            self.f |= Flag::Carry as u8;
        }

        // Perform the ADD
        self.a += d8;

        // Check for zero
        if self.a == 0 {
            self.f |= Flag::Zero as u8;
        }

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_POP_0xC1(&mut self) {
        //! - Prototype: `POP BC`
        //! - Mnemonic:  `POP`
        //! - Size:      1 byte
        //! - Binary:    `0xC1`
        //! - Cycles:    12 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The memory location pointed to by sp is stored into c and sp is
        //!   incremented. The memory location pointed to by sp is stored into b
        //!   and sp is incremented again.

        // Pop BC from stack
        self.c = self.mmu.read8(self.sp);
        self.sp += 1;
        self.b = self.mmu.read8(self.sp);
        self.sp += 1;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xC0(&mut self) {
        //! - Prototype: `RET NZ`
        //! - Mnemonic:  `RET`
        //! - Size:      1 byte
        //! - Binary:    `0xC0`
        //! - Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xC3(&mut self) {
        //! - Prototype: `JP a16`
        //! - Mnemonic:  `JP`
        //! - Size:      3 bytes
        //! - Binary:    `0xC3`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   ** is copied to pc.

        unimplemented!();

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_JP_0xC2(&mut self) {
        //! - Prototype: `JP NZ, a16`
        //! - Mnemonic:  `JP`
        //! - Size:      3 bytes
        //! - Binary:    `0xC2`
        //! - Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_RET_0xC9(&mut self) {
        //! - Prototype: `RET `
        //! - Mnemonic:  `RET`
        //! - Size:      1 byte
        //! - Binary:    `0xC9`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xC8(&mut self) {
        //! - Prototype: `RET Z`
        //! - Mnemonic:  `RET`
        //! - Size:      1 byte
        //! - Binary:    `0xC8`
        //! - Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCB9F(&mut self) {
        //! - Prototype: `RES 3, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE3(&mut self) {
        //! - Prototype: `SET 4, E`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE3`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of e.

        self.e |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9D(&mut self) {
        //! - Prototype: `RES 3, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9E(&mut self) {
        //! - Prototype: `RES 3, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9B(&mut self) {
        //! - Prototype: `RES 3, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9C(&mut self) {
        //! - Prototype: `RES 3, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE4(&mut self) {
        //! - Prototype: `SET 4, H`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE4`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 4 of h.

        self.h |= 0b00010000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9A(&mut self) {
        //! - Prototype: `RES 3, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB9A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE8(&mut self) {
        //! - Prototype: `SET 5, B`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE8`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of b.

        self.b |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE9(&mut self) {
        //! - Prototype: `SET 5, C`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBE9`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 5 of c.

        self.c |= 0b00100000;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC2(&mut self) {
        //! - Prototype: `SET 0, D`
        //! - Mnemonic:  `SET`
        //! - Size:      2 bytes
        //! - Binary:    `0xCBC2`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Sets bit 0 of d.

        self.d |= 0b00000001;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB78(&mut self) {
        //! - Prototype: `BIT 7, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB78`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB79(&mut self) {
        //! - Prototype: `BIT 7, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB79`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB74(&mut self) {
        //! - Prototype: `BIT 6, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB74`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB75(&mut self) {
        //! - Prototype: `BIT 6, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB75`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB76(&mut self) {
        //! - Prototype: `BIT 6, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB76`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB77(&mut self) {
        //! - Prototype: `BIT 6, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB77`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB70(&mut self) {
        //! - Prototype: `BIT 6, B`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB70`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB71(&mut self) {
        //! - Prototype: `BIT 6, C`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB71`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB72(&mut self) {
        //! - Prototype: `BIT 6, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB72`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB73(&mut self) {
        //! - Prototype: `BIT 6, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB73`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 6 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7D(&mut self) {
        //! - Prototype: `BIT 7, L`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7D`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7E(&mut self) {
        //! - Prototype: `BIT 7, (HL)`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7E`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7F(&mut self) {
        //! - Prototype: `BIT 7, A`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7F`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7A(&mut self) {
        //! - Prototype: `BIT 7, D`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7A`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7B(&mut self) {
        //! - Prototype: `BIT 7, E`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7B`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7C(&mut self) {
        //! - Prototype: `BIT 7, H`
        //! - Mnemonic:  `BIT`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB7C`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Force set (1)
        //!   - `C`:  Preserved
        //! - Description
        //!   Tests bit 7 of h.

        // Update flags
        if (self.h & 0b10000000u8)==0 {
            self.f |= Flag::Zero as u8;
        } else {
            self.f &= !(Flag::Zero as u8);
        }
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB96(&mut self) {
        //! - Prototype: `RES 2, (HL)`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB96`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB97(&mut self) {
        //! - Prototype: `RES 2, A`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB97`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB94(&mut self) {
        //! - Prototype: `RES 2, H`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB94`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB95(&mut self) {
        //! - Prototype: `RES 2, L`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB95`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB92(&mut self) {
        //! - Prototype: `RES 2, D`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB92`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB93(&mut self) {
        //! - Prototype: `RES 2, E`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB93`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB90(&mut self) {
        //! - Prototype: `RES 2, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB90`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB91(&mut self) {
        //! - Prototype: `RES 2, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB91`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 2 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB98(&mut self) {
        //! - Prototype: `RES 3, B`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB98`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB99(&mut self) {
        //! - Prototype: `RES 3, C`
        //! - Mnemonic:  `RES`
        //! - Size:      2 bytes
        //! - Binary:    `0xCB99`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   Resets bit 3 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADC_0xCE(&mut self) {
        //! - Prototype: `ADC A, d8`
        //! - Mnemonic:  `ADC`
        //! - Size:      2 bytes
        //! - Binary:    `0xCE`
        //! - Cycles:    8 cycles
        //! - Flags
        //!   - `Z`:  Set if appropriate
        //!   - `N`:  Force unset (0)
        //!   - `H`:  Set if appropriate
        //!   - `C`:  Set if appropriate
        //! - Description
        //!   Adds * and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_CALL_0xCD(&mut self) {
        //! - Prototype: `CALL a16`
        //! - Mnemonic:  `CALL`
        //! - Size:      3 bytes
        //! - Binary:    `0xCD`
        //! - Cycles:    24 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus three is pushed onto the stack, then is
        //!   loaded with **.

        unimplemented!();

        // Update clocks
        self.m += 6;
        self.t += 24;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_RST_0xCF(&mut self) {
        //! - Prototype: `RST 08H`
        //! - Mnemonic:  `RST`
        //! - Size:      1 byte
        //! - Binary:    `0xCF`
        //! - Cycles:    16 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 08h.

        // Push next instruction address on the stack
        self.sp -= 2;
        self.mmu.write16(self.sp, self.pc + 1);

        // Update program counter
        self.pc = 0x08;

        // Update clocks
        self.m += 4;
        self.t += 16;
    }

    pub fn instr_JP_0xCA(&mut self) {
        //! - Prototype: `JP Z, a16`
        //! - Mnemonic:  `JP`
        //! - Size:      3 bytes
        //! - Binary:    `0xCA`
        //! - Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_CALL_0xCC(&mut self) {
        //! - Prototype: `CALL Z, a16`
        //! - Mnemonic:  `CALL`
        //! - Size:      3 bytes
        //! - Binary:    `0xCC`
        //! - Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 3;
    }

    pub fn instr_PREFIX_0xCB(&mut self) {
        //! - Prototype: `PREFIX CB`
        //! - Mnemonic:  `PREFIX`
        //! - Size:      1 byte
        //! - Binary:    `0xCB`
        //! - Cycles:    4 cycles
        //! - Flags
        //!   - `Z`:  Preserved
        //!   - `N`:  Preserved
        //!   - `H`:  Preserved
        //!   - `C`:  Preserved
        //! - Description
        //!   BITS instruction prefix

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    fn disp_NOP_0x00(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("NOP");
        (result, 1)
    }

    fn disp_LD_0x01(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("BC");
        result.push_str(", ");
        let d16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
        (result, 3)
    }

    fn disp_LD_0x02(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(BC)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_INC_0x03(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("BC");
        (result, 1)
    }

    fn disp_INC_0x04(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_DEC_0x05(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x06(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RLCA_0x07(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLCA");
        (result, 1)
    }

    fn disp_LD_0x08(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        let pa16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
        result.push_str(", ");
        result.push_str("SP");
        (result, 3)
    }

    fn disp_ADD_0x09(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        result.push_str("BC");
        (result, 1)
    }

    fn disp_LD_0x0A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(BC)");
        (result, 1)
    }

    fn disp_DEC_0x0B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("BC");
        (result, 1)
    }

    fn disp_INC_0x0C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_DEC_0x0D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x0E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RRCA_0x0F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRCA");
        (result, 1)
    }

    fn disp_STOP_0x10(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("STOP");
        result.push_str(" ");
        result.push_str("0");
        (result, 1)
    }

    fn disp_LD_0x11(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("DE");
        result.push_str(", ");
        let d16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
        (result, 3)
    }

    fn disp_LD_0x12(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(DE)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_INC_0x13(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("DE");
        (result, 1)
    }

    fn disp_INC_0x14(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_DEC_0x15(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x16(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RLA_0x17(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLA");
        (result, 1)
    }

    fn disp_JR_0x18(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JR");
        result.push_str(" ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_ADD_0x19(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        result.push_str("DE");
        (result, 1)
    }

    fn disp_LD_0x1A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(DE)");
        (result, 1)
    }

    fn disp_DEC_0x1B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("DE");
        (result, 1)
    }

    fn disp_INC_0x1C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_DEC_0x1D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x1E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RRA_0x1F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRA");
        (result, 1)
    }

    fn disp_JR_0x20(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JR");
        result.push_str(" ");
        result.push_str("NZ");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_LD_0x21(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        let d16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
        (result, 3)
    }

    fn disp_LD_0x22(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL+)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_INC_0x23(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_INC_0x24(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_DEC_0x25(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x26(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_DAA_0x27(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DAA");
        (result, 1)
    }

    fn disp_JR_0x28(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JR");
        result.push_str(" ");
        result.push_str("Z");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_ADD_0x29(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_LD_0x2A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL+)");
        (result, 1)
    }

    fn disp_DEC_0x2B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_INC_0x2C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_DEC_0x2D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x2E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_CPL_0x2F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CPL");
        (result, 1)
    }

    fn disp_JR_0x30(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JR");
        result.push_str(" ");
        result.push_str("NC");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_LD_0x31(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("SP");
        result.push_str(", ");
        let d16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
        (result, 3)
    }

    fn disp_LD_0x32(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL-)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_INC_0x33(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("SP");
        (result, 1)
    }

    fn disp_INC_0x34(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_DEC_0x35(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x36(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_SCF_0x37(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SCF");
        (result, 1)
    }

    fn disp_JR_0x38(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JR");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_ADD_0x39(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        result.push_str("SP");
        (result, 1)
    }

    fn disp_LD_0x3A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL-)");
        (result, 1)
    }

    fn disp_DEC_0x3B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("SP");
        (result, 1)
    }

    fn disp_INC_0x3C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("INC");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_DEC_0x3D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DEC");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x3E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_CCF_0x3F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CCF");
        (result, 1)
    }

    fn disp_LD_0x40(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x41(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x42(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x43(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x44(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x45(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x46(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x47(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("B");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x48(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x49(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x4A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x4B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x4C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x4D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x4E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x4F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x50(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x51(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x52(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x53(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x54(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x55(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x56(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x57(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("D");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x58(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x59(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x5A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x5B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x5C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x5D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x5E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x5F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("E");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x60(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x61(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x62(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x63(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x64(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x65(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x66(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x67(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("H");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x68(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x69(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x6A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x6B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x6C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x6D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x6E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x6F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("L");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x70(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x71(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x72(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x73(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x74(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x75(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_HALT_0x76(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("HALT");
        (result, 1)
    }

    fn disp_LD_0x77(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(HL)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_LD_0x78(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_LD_0x79(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_LD_0x7A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_LD_0x7B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_LD_0x7C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_LD_0x7D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_LD_0x7E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0x7F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_ADD_0x80(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_ADD_0x81(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_ADD_0x82(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_ADD_0x83(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_ADD_0x84(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_ADD_0x85(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_ADD_0x86(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_ADD_0x87(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_ADC_0x88(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_ADC_0x89(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_ADC_0x8A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_ADC_0x8B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_ADC_0x8C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_ADC_0x8D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_ADC_0x8E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_ADC_0x8F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_SUB_0x90(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_SUB_0x91(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_SUB_0x92(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_SUB_0x93(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_SUB_0x94(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_SUB_0x95(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_SUB_0x96(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_SUB_0x97(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_SBC_0x98(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_SBC_0x99(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_SBC_0x9A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_SBC_0x9B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_SBC_0x9C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_SBC_0x9D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_SBC_0x9E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_SBC_0x9F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_AND_0xA0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_AND_0xA1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_AND_0xA2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_AND_0xA3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_AND_0xA4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_AND_0xA5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_AND_0xA6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_AND_0xA7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_XOR_0xA8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_XOR_0xA9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_XOR_0xAA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_XOR_0xAB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_XOR_0xAC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_XOR_0xAD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_XOR_0xAE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_XOR_0xAF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_OR_0xB0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_OR_0xB1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_OR_0xB2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_OR_0xB3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_OR_0xB4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_OR_0xB5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_OR_0xB6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_OR_0xB7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_CP_0xB8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("B");
        (result, 1)
    }

    fn disp_CP_0xB9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_CP_0xBA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("D");
        (result, 1)
    }

    fn disp_CP_0xBB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("E");
        (result, 1)
    }

    fn disp_CP_0xBC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("H");
        (result, 1)
    }

    fn disp_CP_0xBD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("L");
        (result, 1)
    }

    fn disp_CP_0xBE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_CP_0xBF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_RET_0xC0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RET");
        result.push_str(" ");
        result.push_str("NZ");
        (result, 1)
    }

    fn disp_POP_0xC1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("POP");
        result.push_str(" ");
        result.push_str("BC");
        (result, 1)
    }

    fn disp_JP_0xC2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        result.push_str("NZ");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_JP_0xC3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_CALL_0xC4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CALL");
        result.push_str(" ");
        result.push_str("NZ");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_PUSH_0xC5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("PUSH");
        result.push_str(" ");
        result.push_str("BC");
        (result, 1)
    }

    fn disp_ADD_0xC6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xC7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("00H");
        (result, 1)
    }

    fn disp_RET_0xC8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RET");
        result.push_str(" ");
        result.push_str("Z");
        (result, 1)
    }

    fn disp_RET_0xC9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RET");
        (result, 1)
    }

    fn disp_JP_0xCA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        result.push_str("Z");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_PREFIX_0xCB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("PREFIX");
        result.push_str(" ");
        result.push_str("CB");
        (result, 1)
    }

    fn disp_RLC_0xCB00(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RLC_0xCB01(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RLC_0xCB02(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RLC_0xCB03(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RLC_0xCB04(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RLC_0xCB05(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RLC_0xCB06(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RLC_0xCB07(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RLC");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RRC_0xCB08(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RRC_0xCB09(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RRC_0xCB0A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RRC_0xCB0B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RRC_0xCB0C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RRC_0xCB0D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RRC_0xCB0E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RRC_0xCB0F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RRC");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RL_0xCB10(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RL_0xCB11(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RL_0xCB12(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RL_0xCB13(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RL_0xCB14(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RL_0xCB15(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RL_0xCB16(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RL_0xCB17(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RL");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RR_0xCB18(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RR_0xCB19(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RR_0xCB1A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RR_0xCB1B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RR_0xCB1C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RR_0xCB1D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RR_0xCB1E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RR_0xCB1F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RR");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SLA_0xCB20(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SLA_0xCB21(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SLA_0xCB22(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SLA_0xCB23(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SLA_0xCB24(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SLA_0xCB25(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SLA_0xCB26(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SLA_0xCB27(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SLA");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SRA_0xCB28(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SRA_0xCB29(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SRA_0xCB2A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SRA_0xCB2B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SRA_0xCB2C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SRA_0xCB2D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SRA_0xCB2E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SRA_0xCB2F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRA");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SWAP_0xCB30(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SWAP_0xCB31(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SWAP_0xCB32(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SWAP_0xCB33(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SWAP_0xCB34(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SWAP_0xCB35(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SWAP_0xCB36(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SWAP_0xCB37(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SWAP");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SRL_0xCB38(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SRL_0xCB39(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SRL_0xCB3A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SRL_0xCB3B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SRL_0xCB3C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SRL_0xCB3D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SRL_0xCB3E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SRL_0xCB3F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SRL");
        result.push_str(" ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB40(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB41(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB42(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB43(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB44(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB45(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB46(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB47(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB48(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB49(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB4A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB4B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB4C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB4D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB4E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB4F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB50(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB51(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB52(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB53(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB54(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB55(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB56(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB57(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB58(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB59(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB5A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB5B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB5C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB5D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB5E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB5F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB60(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB61(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB62(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB63(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB64(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB65(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB66(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB67(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB68(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB69(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB6A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB6B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB6C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB6D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB6E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB6F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB70(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB71(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB72(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB73(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB74(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB75(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB76(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB77(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_BIT_0xCB78(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_BIT_0xCB79(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_BIT_0xCB7A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_BIT_0xCB7B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_BIT_0xCB7C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_BIT_0xCB7D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_BIT_0xCB7E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_BIT_0xCB7F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("BIT");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCB80(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCB81(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCB82(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCB83(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCB84(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCB85(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCB86(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCB87(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCB88(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCB89(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCB8A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCB8B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCB8C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCB8D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCB8E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCB8F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCB90(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCB91(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCB92(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCB93(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCB94(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCB95(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCB96(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCB97(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCB98(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCB99(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCB9A(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCB9B(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCB9C(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCB9D(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCB9E(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCB9F(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCBA0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCBA1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCBA2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCBA3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCBA4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCBA5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCBA6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCBA7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCBA8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCBA9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCBAA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCBAB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCBAC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCBAD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCBAE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCBAF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCBB0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCBB1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCBB2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCBB3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCBB4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCBB5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCBB6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCBB7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_RES_0xCBB8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_RES_0xCBB9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_RES_0xCBBA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_RES_0xCBBB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_RES_0xCBBC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_RES_0xCBBD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_RES_0xCBBE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_RES_0xCBBF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RES");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBC0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBC1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBC2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBC3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBC4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBC5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBC6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBC7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("0");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBC8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBC9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBCA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBCB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBCC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBCD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBCE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBCF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("1");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBD0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBD1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBD2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBD3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBD4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBD5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBD6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBD7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("2");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBD8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBD9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBDA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBDB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBDC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBDD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBDE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBDF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("3");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBE0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBE1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBE2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBE3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBE4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBE5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBE6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBE7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("4");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBE8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBE9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBEA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBEB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBEC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBED(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBEE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBEF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("5");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBF0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBF1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBF2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBF3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBF4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBF5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBF6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBF7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("6");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_SET_0xCBF8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("B");
        (result, 2)
    }

    fn disp_SET_0xCBF9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("C");
        (result, 2)
    }

    fn disp_SET_0xCBFA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("D");
        (result, 2)
    }

    fn disp_SET_0xCBFB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("E");
        (result, 2)
    }

    fn disp_SET_0xCBFC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("H");
        (result, 2)
    }

    fn disp_SET_0xCBFD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("L");
        (result, 2)
    }

    fn disp_SET_0xCBFE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("(HL)");
        (result, 2)
    }

    fn disp_SET_0xCBFF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SET");
        result.push_str(" ");
        result.push_str("7");
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_CALL_0xCC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CALL");
        result.push_str(" ");
        result.push_str("Z");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_CALL_0xCD(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CALL");
        result.push_str(" ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_ADC_0xCE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xCF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("08H");
        (result, 1)
    }

    fn disp_RET_0xD0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RET");
        result.push_str(" ");
        result.push_str("NC");
        (result, 1)
    }

    fn disp_POP_0xD1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("POP");
        result.push_str(" ");
        result.push_str("DE");
        (result, 1)
    }

    fn disp_JP_0xD2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        result.push_str("NC");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_CALL_0xD4(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CALL");
        result.push_str(" ");
        result.push_str("NC");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_PUSH_0xD5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("PUSH");
        result.push_str(" ");
        result.push_str("DE");
        (result, 1)
    }

    fn disp_SUB_0xD6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SUB");
        result.push_str(" ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xD7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("10H");
        (result, 1)
    }

    fn disp_RET_0xD8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RET");
        result.push_str(" ");
        result.push_str("C");
        (result, 1)
    }

    fn disp_RETI_0xD9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RETI");
        (result, 1)
    }

    fn disp_JP_0xDA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_CALL_0xDC(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CALL");
        result.push_str(" ");
        result.push_str("C");
        result.push_str(", ");
        let a16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
        (result, 3)
    }

    fn disp_SBC_0xDE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("SBC");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xDF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("18H");
        (result, 1)
    }

    fn disp_LDH_0xE0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LDH");
        result.push_str(" ");
        let pa8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("(0x{:0>2.2X})", pa8).as_slice());
        result.push_str(", ");
        result.push_str("A");
        (result, 2)
    }

    fn disp_POP_0xE1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("POP");
        result.push_str(" ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_LD_0xE2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("(C)");
        result.push_str(", ");
        result.push_str("A");
        (result, 1)
    }

    fn disp_PUSH_0xE5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("PUSH");
        result.push_str(" ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_AND_0xE6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("AND");
        result.push_str(" ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xE7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("20H");
        (result, 1)
    }

    fn disp_ADD_0xE8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("ADD");
        result.push_str(" ");
        result.push_str("SP");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_JP_0xE9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("JP");
        result.push_str(" ");
        result.push_str("(HL)");
        (result, 1)
    }

    fn disp_LD_0xEA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        let pa16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
        result.push_str(", ");
        result.push_str("A");
        (result, 3)
    }

    fn disp_XOR_0xEE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("XOR");
        result.push_str(" ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xEF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("28H");
        (result, 1)
    }

    fn disp_LDH_0xF0(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LDH");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let pa8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("(0x{:0>2.2X})", pa8).as_slice());
        (result, 2)
    }

    fn disp_POP_0xF1(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("POP");
        result.push_str(" ");
        result.push_str("AF");
        (result, 1)
    }

    fn disp_LD_0xF2(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        result.push_str("(C)");
        (result, 1)
    }

    fn disp_DI_0xF3(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("DI");
        (result, 1)
    }

    fn disp_PUSH_0xF5(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("PUSH");
        result.push_str(" ");
        result.push_str("AF");
        (result, 1)
    }

    fn disp_OR_0xF6(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("OR");
        result.push_str(" ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xF7(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("30H");
        (result, 1)
    }

    fn disp_LD_0xF8(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("HL");
        result.push_str(", ");
        let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
        result.push_str(format!("SP+0x{:0>2.2X}", r8).as_slice());
        (result, 2)
    }

    fn disp_LD_0xF9(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("SP");
        result.push_str(", ");
        result.push_str("HL");
        (result, 1)
    }

    fn disp_LD_0xFA(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("LD");
        result.push_str(" ");
        result.push_str("A");
        result.push_str(", ");
        let pa16: u16 = self.mmu.read16(self.pc + 1);
        result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
        (result, 3)
    }

    fn disp_EI_0xFB(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("EI");
        (result, 1)
    }

    fn disp_CP_0xFE(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("CP");
        result.push_str(" ");
        let d8: u8 = self.mmu.read8(self.pc + 1);
        result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
        (result, 2)
    }

    fn disp_RST_0xFF(&self) -> (String, u8) {
        let mut result: String = String::new();
        result.push_str("RST");
        result.push_str(" ");
        result.push_str("38H");
        (result, 1)
    }
}

// ==============================================
// Traits
// ==============================================
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let af: u16 = ((self.a as u16) << 8) + self.f as u16;
        let bc: u16 = ((self.b as u16) << 8) + self.c as u16;
        let de: u16 = ((self.d as u16) << 8) + self.e as u16;
        let hl: u16 = ((self.h as u16) << 8) + self.l as u16;

        f.write_fmt(format_args!("|  A   F  \
                                  |  B   C  \
                                  |  D   E  \
                                  |  H   L  \
                                  |     PC \
                                  |     SP \
                                  | ZNHC---- \
                                  |  M  |  T  |\n"));
        f.write_fmt(format_args!("| x{:0>2.2X} x{:0>2.2X} \
                                  | x{:0>2.2X} x{:0>2.2X} \
                                  | x{:0>2.2X} x{:0>2.2X} \
                                  | x{:0>2.2X} x{:0>2.2X} \
                                  | @x{:0>4.4X} \
                                  | @x{:0>4.4X} \
                                  | {:0>8.8b} \
                                  | {: >3.3} | {: >3.3} |\n",
                                self.a, self.f,
                                self.b, self.c,
                                self.d, self.e,
                                self.h, self.l,
                                self.pc,
                                self.sp,
                                self.f,
                                self.m, self.t));
        f.write_fmt(format_args!("| {: >3.3} {: >3.3} \
                                  | {: >3.3} {: >3.3} \
                                  | {: >3.3} {: >3.3} \
                                  | {: >3.3} {: >3.3} \
                                  |      - |      - |\n",
                                self.a, self.f,
                                self.b, self.c,
                                self.d, self.e,
                                self.h, self.l));
        f.write_fmt(format_args!("|   {: >5.5} \
                                  |   {: >5.5} \
                                  |   {: >5.5} \
                                  |   {: >5.5} \
                                  | @{: >5.5} \
                                  | @{: >5.5} |\n",
                                af,
                                bc,
                                de,
                                hl,
                                self.pc,
                                self.sp))
    }
}

// ==============================================
// Statics
// ==============================================

lazy_static! {
    static ref DECODER: BTreeMap<u16, (fn(&mut Cpu), fn(&Cpu) -> (String, u8))> = {
        let mut decoder: BTreeMap<u16, (fn(&mut Cpu), fn(&Cpu) -> (String, u8))> = BTreeMap::new();
        decoder.insert(0x00 as u16, (Cpu::instr_NOP_0x00, Cpu::disp_NOP_0x00));
        decoder.insert(0x01 as u16, (Cpu::instr_LD_0x01, Cpu::disp_LD_0x01));
        decoder.insert(0x02 as u16, (Cpu::instr_LD_0x02, Cpu::disp_LD_0x02));
        decoder.insert(0x03 as u16, (Cpu::instr_INC_0x03, Cpu::disp_INC_0x03));
        decoder.insert(0x04 as u16, (Cpu::instr_INC_0x04, Cpu::disp_INC_0x04));
        decoder.insert(0x05 as u16, (Cpu::instr_DEC_0x05, Cpu::disp_DEC_0x05));
        decoder.insert(0x06 as u16, (Cpu::instr_LD_0x06, Cpu::disp_LD_0x06));
        decoder.insert(0x07 as u16, (Cpu::instr_RLCA_0x07, Cpu::disp_RLCA_0x07));
        decoder.insert(0x08 as u16, (Cpu::instr_LD_0x08, Cpu::disp_LD_0x08));
        decoder.insert(0x09 as u16, (Cpu::instr_ADD_0x09, Cpu::disp_ADD_0x09));
        decoder.insert(0x0A as u16, (Cpu::instr_LD_0x0A, Cpu::disp_LD_0x0A));
        decoder.insert(0x0B as u16, (Cpu::instr_DEC_0x0B, Cpu::disp_DEC_0x0B));
        decoder.insert(0x0C as u16, (Cpu::instr_INC_0x0C, Cpu::disp_INC_0x0C));
        decoder.insert(0x0D as u16, (Cpu::instr_DEC_0x0D, Cpu::disp_DEC_0x0D));
        decoder.insert(0x0E as u16, (Cpu::instr_LD_0x0E, Cpu::disp_LD_0x0E));
        decoder.insert(0x0F as u16, (Cpu::instr_RRCA_0x0F, Cpu::disp_RRCA_0x0F));
        decoder.insert(0x10 as u16, (Cpu::instr_STOP_0x10, Cpu::disp_STOP_0x10));
        decoder.insert(0x11 as u16, (Cpu::instr_LD_0x11, Cpu::disp_LD_0x11));
        decoder.insert(0x12 as u16, (Cpu::instr_LD_0x12, Cpu::disp_LD_0x12));
        decoder.insert(0x13 as u16, (Cpu::instr_INC_0x13, Cpu::disp_INC_0x13));
        decoder.insert(0x14 as u16, (Cpu::instr_INC_0x14, Cpu::disp_INC_0x14));
        decoder.insert(0x15 as u16, (Cpu::instr_DEC_0x15, Cpu::disp_DEC_0x15));
        decoder.insert(0x16 as u16, (Cpu::instr_LD_0x16, Cpu::disp_LD_0x16));
        decoder.insert(0x17 as u16, (Cpu::instr_RLA_0x17, Cpu::disp_RLA_0x17));
        decoder.insert(0x18 as u16, (Cpu::instr_JR_0x18, Cpu::disp_JR_0x18));
        decoder.insert(0x19 as u16, (Cpu::instr_ADD_0x19, Cpu::disp_ADD_0x19));
        decoder.insert(0x1A as u16, (Cpu::instr_LD_0x1A, Cpu::disp_LD_0x1A));
        decoder.insert(0x1B as u16, (Cpu::instr_DEC_0x1B, Cpu::disp_DEC_0x1B));
        decoder.insert(0x1C as u16, (Cpu::instr_INC_0x1C, Cpu::disp_INC_0x1C));
        decoder.insert(0x1D as u16, (Cpu::instr_DEC_0x1D, Cpu::disp_DEC_0x1D));
        decoder.insert(0x1E as u16, (Cpu::instr_LD_0x1E, Cpu::disp_LD_0x1E));
        decoder.insert(0x1F as u16, (Cpu::instr_RRA_0x1F, Cpu::disp_RRA_0x1F));
        decoder.insert(0x20 as u16, (Cpu::instr_JR_0x20, Cpu::disp_JR_0x20));
        decoder.insert(0x21 as u16, (Cpu::instr_LD_0x21, Cpu::disp_LD_0x21));
        decoder.insert(0x22 as u16, (Cpu::instr_LD_0x22, Cpu::disp_LD_0x22));
        decoder.insert(0x23 as u16, (Cpu::instr_INC_0x23, Cpu::disp_INC_0x23));
        decoder.insert(0x24 as u16, (Cpu::instr_INC_0x24, Cpu::disp_INC_0x24));
        decoder.insert(0x25 as u16, (Cpu::instr_DEC_0x25, Cpu::disp_DEC_0x25));
        decoder.insert(0x26 as u16, (Cpu::instr_LD_0x26, Cpu::disp_LD_0x26));
        decoder.insert(0x27 as u16, (Cpu::instr_DAA_0x27, Cpu::disp_DAA_0x27));
        decoder.insert(0x28 as u16, (Cpu::instr_JR_0x28, Cpu::disp_JR_0x28));
        decoder.insert(0x29 as u16, (Cpu::instr_ADD_0x29, Cpu::disp_ADD_0x29));
        decoder.insert(0x2A as u16, (Cpu::instr_LD_0x2A, Cpu::disp_LD_0x2A));
        decoder.insert(0x2B as u16, (Cpu::instr_DEC_0x2B, Cpu::disp_DEC_0x2B));
        decoder.insert(0x2C as u16, (Cpu::instr_INC_0x2C, Cpu::disp_INC_0x2C));
        decoder.insert(0x2D as u16, (Cpu::instr_DEC_0x2D, Cpu::disp_DEC_0x2D));
        decoder.insert(0x2E as u16, (Cpu::instr_LD_0x2E, Cpu::disp_LD_0x2E));
        decoder.insert(0x2F as u16, (Cpu::instr_CPL_0x2F, Cpu::disp_CPL_0x2F));
        decoder.insert(0x30 as u16, (Cpu::instr_JR_0x30, Cpu::disp_JR_0x30));
        decoder.insert(0x31 as u16, (Cpu::instr_LD_0x31, Cpu::disp_LD_0x31));
        decoder.insert(0x32 as u16, (Cpu::instr_LD_0x32, Cpu::disp_LD_0x32));
        decoder.insert(0x33 as u16, (Cpu::instr_INC_0x33, Cpu::disp_INC_0x33));
        decoder.insert(0x34 as u16, (Cpu::instr_INC_0x34, Cpu::disp_INC_0x34));
        decoder.insert(0x35 as u16, (Cpu::instr_DEC_0x35, Cpu::disp_DEC_0x35));
        decoder.insert(0x36 as u16, (Cpu::instr_LD_0x36, Cpu::disp_LD_0x36));
        decoder.insert(0x37 as u16, (Cpu::instr_SCF_0x37, Cpu::disp_SCF_0x37));
        decoder.insert(0x38 as u16, (Cpu::instr_JR_0x38, Cpu::disp_JR_0x38));
        decoder.insert(0x39 as u16, (Cpu::instr_ADD_0x39, Cpu::disp_ADD_0x39));
        decoder.insert(0x3A as u16, (Cpu::instr_LD_0x3A, Cpu::disp_LD_0x3A));
        decoder.insert(0x3B as u16, (Cpu::instr_DEC_0x3B, Cpu::disp_DEC_0x3B));
        decoder.insert(0x3C as u16, (Cpu::instr_INC_0x3C, Cpu::disp_INC_0x3C));
        decoder.insert(0x3D as u16, (Cpu::instr_DEC_0x3D, Cpu::disp_DEC_0x3D));
        decoder.insert(0x3E as u16, (Cpu::instr_LD_0x3E, Cpu::disp_LD_0x3E));
        decoder.insert(0x3F as u16, (Cpu::instr_CCF_0x3F, Cpu::disp_CCF_0x3F));
        decoder.insert(0x40 as u16, (Cpu::instr_LD_0x40, Cpu::disp_LD_0x40));
        decoder.insert(0x41 as u16, (Cpu::instr_LD_0x41, Cpu::disp_LD_0x41));
        decoder.insert(0x42 as u16, (Cpu::instr_LD_0x42, Cpu::disp_LD_0x42));
        decoder.insert(0x43 as u16, (Cpu::instr_LD_0x43, Cpu::disp_LD_0x43));
        decoder.insert(0x44 as u16, (Cpu::instr_LD_0x44, Cpu::disp_LD_0x44));
        decoder.insert(0x45 as u16, (Cpu::instr_LD_0x45, Cpu::disp_LD_0x45));
        decoder.insert(0x46 as u16, (Cpu::instr_LD_0x46, Cpu::disp_LD_0x46));
        decoder.insert(0x47 as u16, (Cpu::instr_LD_0x47, Cpu::disp_LD_0x47));
        decoder.insert(0x48 as u16, (Cpu::instr_LD_0x48, Cpu::disp_LD_0x48));
        decoder.insert(0x49 as u16, (Cpu::instr_LD_0x49, Cpu::disp_LD_0x49));
        decoder.insert(0x4A as u16, (Cpu::instr_LD_0x4A, Cpu::disp_LD_0x4A));
        decoder.insert(0x4B as u16, (Cpu::instr_LD_0x4B, Cpu::disp_LD_0x4B));
        decoder.insert(0x4C as u16, (Cpu::instr_LD_0x4C, Cpu::disp_LD_0x4C));
        decoder.insert(0x4D as u16, (Cpu::instr_LD_0x4D, Cpu::disp_LD_0x4D));
        decoder.insert(0x4E as u16, (Cpu::instr_LD_0x4E, Cpu::disp_LD_0x4E));
        decoder.insert(0x4F as u16, (Cpu::instr_LD_0x4F, Cpu::disp_LD_0x4F));
        decoder.insert(0x50 as u16, (Cpu::instr_LD_0x50, Cpu::disp_LD_0x50));
        decoder.insert(0x51 as u16, (Cpu::instr_LD_0x51, Cpu::disp_LD_0x51));
        decoder.insert(0x52 as u16, (Cpu::instr_LD_0x52, Cpu::disp_LD_0x52));
        decoder.insert(0x53 as u16, (Cpu::instr_LD_0x53, Cpu::disp_LD_0x53));
        decoder.insert(0x54 as u16, (Cpu::instr_LD_0x54, Cpu::disp_LD_0x54));
        decoder.insert(0x55 as u16, (Cpu::instr_LD_0x55, Cpu::disp_LD_0x55));
        decoder.insert(0x56 as u16, (Cpu::instr_LD_0x56, Cpu::disp_LD_0x56));
        decoder.insert(0x57 as u16, (Cpu::instr_LD_0x57, Cpu::disp_LD_0x57));
        decoder.insert(0x58 as u16, (Cpu::instr_LD_0x58, Cpu::disp_LD_0x58));
        decoder.insert(0x59 as u16, (Cpu::instr_LD_0x59, Cpu::disp_LD_0x59));
        decoder.insert(0x5A as u16, (Cpu::instr_LD_0x5A, Cpu::disp_LD_0x5A));
        decoder.insert(0x5B as u16, (Cpu::instr_LD_0x5B, Cpu::disp_LD_0x5B));
        decoder.insert(0x5C as u16, (Cpu::instr_LD_0x5C, Cpu::disp_LD_0x5C));
        decoder.insert(0x5D as u16, (Cpu::instr_LD_0x5D, Cpu::disp_LD_0x5D));
        decoder.insert(0x5E as u16, (Cpu::instr_LD_0x5E, Cpu::disp_LD_0x5E));
        decoder.insert(0x5F as u16, (Cpu::instr_LD_0x5F, Cpu::disp_LD_0x5F));
        decoder.insert(0x60 as u16, (Cpu::instr_LD_0x60, Cpu::disp_LD_0x60));
        decoder.insert(0x61 as u16, (Cpu::instr_LD_0x61, Cpu::disp_LD_0x61));
        decoder.insert(0x62 as u16, (Cpu::instr_LD_0x62, Cpu::disp_LD_0x62));
        decoder.insert(0x63 as u16, (Cpu::instr_LD_0x63, Cpu::disp_LD_0x63));
        decoder.insert(0x64 as u16, (Cpu::instr_LD_0x64, Cpu::disp_LD_0x64));
        decoder.insert(0x65 as u16, (Cpu::instr_LD_0x65, Cpu::disp_LD_0x65));
        decoder.insert(0x66 as u16, (Cpu::instr_LD_0x66, Cpu::disp_LD_0x66));
        decoder.insert(0x67 as u16, (Cpu::instr_LD_0x67, Cpu::disp_LD_0x67));
        decoder.insert(0x68 as u16, (Cpu::instr_LD_0x68, Cpu::disp_LD_0x68));
        decoder.insert(0x69 as u16, (Cpu::instr_LD_0x69, Cpu::disp_LD_0x69));
        decoder.insert(0x6A as u16, (Cpu::instr_LD_0x6A, Cpu::disp_LD_0x6A));
        decoder.insert(0x6B as u16, (Cpu::instr_LD_0x6B, Cpu::disp_LD_0x6B));
        decoder.insert(0x6C as u16, (Cpu::instr_LD_0x6C, Cpu::disp_LD_0x6C));
        decoder.insert(0x6D as u16, (Cpu::instr_LD_0x6D, Cpu::disp_LD_0x6D));
        decoder.insert(0x6E as u16, (Cpu::instr_LD_0x6E, Cpu::disp_LD_0x6E));
        decoder.insert(0x6F as u16, (Cpu::instr_LD_0x6F, Cpu::disp_LD_0x6F));
        decoder.insert(0x70 as u16, (Cpu::instr_LD_0x70, Cpu::disp_LD_0x70));
        decoder.insert(0x71 as u16, (Cpu::instr_LD_0x71, Cpu::disp_LD_0x71));
        decoder.insert(0x72 as u16, (Cpu::instr_LD_0x72, Cpu::disp_LD_0x72));
        decoder.insert(0x73 as u16, (Cpu::instr_LD_0x73, Cpu::disp_LD_0x73));
        decoder.insert(0x74 as u16, (Cpu::instr_LD_0x74, Cpu::disp_LD_0x74));
        decoder.insert(0x75 as u16, (Cpu::instr_LD_0x75, Cpu::disp_LD_0x75));
        decoder.insert(0x76 as u16, (Cpu::instr_HALT_0x76, Cpu::disp_HALT_0x76));
        decoder.insert(0x77 as u16, (Cpu::instr_LD_0x77, Cpu::disp_LD_0x77));
        decoder.insert(0x78 as u16, (Cpu::instr_LD_0x78, Cpu::disp_LD_0x78));
        decoder.insert(0x79 as u16, (Cpu::instr_LD_0x79, Cpu::disp_LD_0x79));
        decoder.insert(0x7A as u16, (Cpu::instr_LD_0x7A, Cpu::disp_LD_0x7A));
        decoder.insert(0x7B as u16, (Cpu::instr_LD_0x7B, Cpu::disp_LD_0x7B));
        decoder.insert(0x7C as u16, (Cpu::instr_LD_0x7C, Cpu::disp_LD_0x7C));
        decoder.insert(0x7D as u16, (Cpu::instr_LD_0x7D, Cpu::disp_LD_0x7D));
        decoder.insert(0x7E as u16, (Cpu::instr_LD_0x7E, Cpu::disp_LD_0x7E));
        decoder.insert(0x7F as u16, (Cpu::instr_LD_0x7F, Cpu::disp_LD_0x7F));
        decoder.insert(0x80 as u16, (Cpu::instr_ADD_0x80, Cpu::disp_ADD_0x80));
        decoder.insert(0x81 as u16, (Cpu::instr_ADD_0x81, Cpu::disp_ADD_0x81));
        decoder.insert(0x82 as u16, (Cpu::instr_ADD_0x82, Cpu::disp_ADD_0x82));
        decoder.insert(0x83 as u16, (Cpu::instr_ADD_0x83, Cpu::disp_ADD_0x83));
        decoder.insert(0x84 as u16, (Cpu::instr_ADD_0x84, Cpu::disp_ADD_0x84));
        decoder.insert(0x85 as u16, (Cpu::instr_ADD_0x85, Cpu::disp_ADD_0x85));
        decoder.insert(0x86 as u16, (Cpu::instr_ADD_0x86, Cpu::disp_ADD_0x86));
        decoder.insert(0x87 as u16, (Cpu::instr_ADD_0x87, Cpu::disp_ADD_0x87));
        decoder.insert(0x88 as u16, (Cpu::instr_ADC_0x88, Cpu::disp_ADC_0x88));
        decoder.insert(0x89 as u16, (Cpu::instr_ADC_0x89, Cpu::disp_ADC_0x89));
        decoder.insert(0x8A as u16, (Cpu::instr_ADC_0x8A, Cpu::disp_ADC_0x8A));
        decoder.insert(0x8B as u16, (Cpu::instr_ADC_0x8B, Cpu::disp_ADC_0x8B));
        decoder.insert(0x8C as u16, (Cpu::instr_ADC_0x8C, Cpu::disp_ADC_0x8C));
        decoder.insert(0x8D as u16, (Cpu::instr_ADC_0x8D, Cpu::disp_ADC_0x8D));
        decoder.insert(0x8E as u16, (Cpu::instr_ADC_0x8E, Cpu::disp_ADC_0x8E));
        decoder.insert(0x8F as u16, (Cpu::instr_ADC_0x8F, Cpu::disp_ADC_0x8F));
        decoder.insert(0x90 as u16, (Cpu::instr_SUB_0x90, Cpu::disp_SUB_0x90));
        decoder.insert(0x91 as u16, (Cpu::instr_SUB_0x91, Cpu::disp_SUB_0x91));
        decoder.insert(0x92 as u16, (Cpu::instr_SUB_0x92, Cpu::disp_SUB_0x92));
        decoder.insert(0x93 as u16, (Cpu::instr_SUB_0x93, Cpu::disp_SUB_0x93));
        decoder.insert(0x94 as u16, (Cpu::instr_SUB_0x94, Cpu::disp_SUB_0x94));
        decoder.insert(0x95 as u16, (Cpu::instr_SUB_0x95, Cpu::disp_SUB_0x95));
        decoder.insert(0x96 as u16, (Cpu::instr_SUB_0x96, Cpu::disp_SUB_0x96));
        decoder.insert(0x97 as u16, (Cpu::instr_SUB_0x97, Cpu::disp_SUB_0x97));
        decoder.insert(0x98 as u16, (Cpu::instr_SBC_0x98, Cpu::disp_SBC_0x98));
        decoder.insert(0x99 as u16, (Cpu::instr_SBC_0x99, Cpu::disp_SBC_0x99));
        decoder.insert(0x9A as u16, (Cpu::instr_SBC_0x9A, Cpu::disp_SBC_0x9A));
        decoder.insert(0x9B as u16, (Cpu::instr_SBC_0x9B, Cpu::disp_SBC_0x9B));
        decoder.insert(0x9C as u16, (Cpu::instr_SBC_0x9C, Cpu::disp_SBC_0x9C));
        decoder.insert(0x9D as u16, (Cpu::instr_SBC_0x9D, Cpu::disp_SBC_0x9D));
        decoder.insert(0x9E as u16, (Cpu::instr_SBC_0x9E, Cpu::disp_SBC_0x9E));
        decoder.insert(0x9F as u16, (Cpu::instr_SBC_0x9F, Cpu::disp_SBC_0x9F));
        decoder.insert(0xA0 as u16, (Cpu::instr_AND_0xA0, Cpu::disp_AND_0xA0));
        decoder.insert(0xA1 as u16, (Cpu::instr_AND_0xA1, Cpu::disp_AND_0xA1));
        decoder.insert(0xA2 as u16, (Cpu::instr_AND_0xA2, Cpu::disp_AND_0xA2));
        decoder.insert(0xA3 as u16, (Cpu::instr_AND_0xA3, Cpu::disp_AND_0xA3));
        decoder.insert(0xA4 as u16, (Cpu::instr_AND_0xA4, Cpu::disp_AND_0xA4));
        decoder.insert(0xA5 as u16, (Cpu::instr_AND_0xA5, Cpu::disp_AND_0xA5));
        decoder.insert(0xA6 as u16, (Cpu::instr_AND_0xA6, Cpu::disp_AND_0xA6));
        decoder.insert(0xA7 as u16, (Cpu::instr_AND_0xA7, Cpu::disp_AND_0xA7));
        decoder.insert(0xA8 as u16, (Cpu::instr_XOR_0xA8, Cpu::disp_XOR_0xA8));
        decoder.insert(0xA9 as u16, (Cpu::instr_XOR_0xA9, Cpu::disp_XOR_0xA9));
        decoder.insert(0xAA as u16, (Cpu::instr_XOR_0xAA, Cpu::disp_XOR_0xAA));
        decoder.insert(0xAB as u16, (Cpu::instr_XOR_0xAB, Cpu::disp_XOR_0xAB));
        decoder.insert(0xAC as u16, (Cpu::instr_XOR_0xAC, Cpu::disp_XOR_0xAC));
        decoder.insert(0xAD as u16, (Cpu::instr_XOR_0xAD, Cpu::disp_XOR_0xAD));
        decoder.insert(0xAE as u16, (Cpu::instr_XOR_0xAE, Cpu::disp_XOR_0xAE));
        decoder.insert(0xAF as u16, (Cpu::instr_XOR_0xAF, Cpu::disp_XOR_0xAF));
        decoder.insert(0xB0 as u16, (Cpu::instr_OR_0xB0, Cpu::disp_OR_0xB0));
        decoder.insert(0xB1 as u16, (Cpu::instr_OR_0xB1, Cpu::disp_OR_0xB1));
        decoder.insert(0xB2 as u16, (Cpu::instr_OR_0xB2, Cpu::disp_OR_0xB2));
        decoder.insert(0xB3 as u16, (Cpu::instr_OR_0xB3, Cpu::disp_OR_0xB3));
        decoder.insert(0xB4 as u16, (Cpu::instr_OR_0xB4, Cpu::disp_OR_0xB4));
        decoder.insert(0xB5 as u16, (Cpu::instr_OR_0xB5, Cpu::disp_OR_0xB5));
        decoder.insert(0xB6 as u16, (Cpu::instr_OR_0xB6, Cpu::disp_OR_0xB6));
        decoder.insert(0xB7 as u16, (Cpu::instr_OR_0xB7, Cpu::disp_OR_0xB7));
        decoder.insert(0xB8 as u16, (Cpu::instr_CP_0xB8, Cpu::disp_CP_0xB8));
        decoder.insert(0xB9 as u16, (Cpu::instr_CP_0xB9, Cpu::disp_CP_0xB9));
        decoder.insert(0xBA as u16, (Cpu::instr_CP_0xBA, Cpu::disp_CP_0xBA));
        decoder.insert(0xBB as u16, (Cpu::instr_CP_0xBB, Cpu::disp_CP_0xBB));
        decoder.insert(0xBC as u16, (Cpu::instr_CP_0xBC, Cpu::disp_CP_0xBC));
        decoder.insert(0xBD as u16, (Cpu::instr_CP_0xBD, Cpu::disp_CP_0xBD));
        decoder.insert(0xBE as u16, (Cpu::instr_CP_0xBE, Cpu::disp_CP_0xBE));
        decoder.insert(0xBF as u16, (Cpu::instr_CP_0xBF, Cpu::disp_CP_0xBF));
        decoder.insert(0xC0 as u16, (Cpu::instr_RET_0xC0, Cpu::disp_RET_0xC0));
        decoder.insert(0xC1 as u16, (Cpu::instr_POP_0xC1, Cpu::disp_POP_0xC1));
        decoder.insert(0xC2 as u16, (Cpu::instr_JP_0xC2, Cpu::disp_JP_0xC2));
        decoder.insert(0xC3 as u16, (Cpu::instr_JP_0xC3, Cpu::disp_JP_0xC3));
        decoder.insert(0xC4 as u16, (Cpu::instr_CALL_0xC4, Cpu::disp_CALL_0xC4));
        decoder.insert(0xC5 as u16, (Cpu::instr_PUSH_0xC5, Cpu::disp_PUSH_0xC5));
        decoder.insert(0xC6 as u16, (Cpu::instr_ADD_0xC6, Cpu::disp_ADD_0xC6));
        decoder.insert(0xC7 as u16, (Cpu::instr_RST_0xC7, Cpu::disp_RST_0xC7));
        decoder.insert(0xC8 as u16, (Cpu::instr_RET_0xC8, Cpu::disp_RET_0xC8));
        decoder.insert(0xC9 as u16, (Cpu::instr_RET_0xC9, Cpu::disp_RET_0xC9));
        decoder.insert(0xCA as u16, (Cpu::instr_JP_0xCA, Cpu::disp_JP_0xCA));
        decoder.insert(0xCB as u16, (Cpu::instr_PREFIX_0xCB, Cpu::disp_PREFIX_0xCB));
        decoder.insert(0xCC as u16, (Cpu::instr_CALL_0xCC, Cpu::disp_CALL_0xCC));
        decoder.insert(0xCD as u16, (Cpu::instr_CALL_0xCD, Cpu::disp_CALL_0xCD));
        decoder.insert(0xCE as u16, (Cpu::instr_ADC_0xCE, Cpu::disp_ADC_0xCE));
        decoder.insert(0xCF as u16, (Cpu::instr_RST_0xCF, Cpu::disp_RST_0xCF));
        decoder.insert(0xD0 as u16, (Cpu::instr_RET_0xD0, Cpu::disp_RET_0xD0));
        decoder.insert(0xD1 as u16, (Cpu::instr_POP_0xD1, Cpu::disp_POP_0xD1));
        decoder.insert(0xD2 as u16, (Cpu::instr_JP_0xD2, Cpu::disp_JP_0xD2));
        decoder.insert(0xD4 as u16, (Cpu::instr_CALL_0xD4, Cpu::disp_CALL_0xD4));
        decoder.insert(0xD5 as u16, (Cpu::instr_PUSH_0xD5, Cpu::disp_PUSH_0xD5));
        decoder.insert(0xD6 as u16, (Cpu::instr_SUB_0xD6, Cpu::disp_SUB_0xD6));
        decoder.insert(0xD7 as u16, (Cpu::instr_RST_0xD7, Cpu::disp_RST_0xD7));
        decoder.insert(0xD8 as u16, (Cpu::instr_RET_0xD8, Cpu::disp_RET_0xD8));
        decoder.insert(0xD9 as u16, (Cpu::instr_RETI_0xD9, Cpu::disp_RETI_0xD9));
        decoder.insert(0xDA as u16, (Cpu::instr_JP_0xDA, Cpu::disp_JP_0xDA));
        decoder.insert(0xDC as u16, (Cpu::instr_CALL_0xDC, Cpu::disp_CALL_0xDC));
        decoder.insert(0xDE as u16, (Cpu::instr_SBC_0xDE, Cpu::disp_SBC_0xDE));
        decoder.insert(0xDF as u16, (Cpu::instr_RST_0xDF, Cpu::disp_RST_0xDF));
        decoder.insert(0xE0 as u16, (Cpu::instr_LDH_0xE0, Cpu::disp_LDH_0xE0));
        decoder.insert(0xE1 as u16, (Cpu::instr_POP_0xE1, Cpu::disp_POP_0xE1));
        decoder.insert(0xE2 as u16, (Cpu::instr_LD_0xE2, Cpu::disp_LD_0xE2));
        decoder.insert(0xE5 as u16, (Cpu::instr_PUSH_0xE5, Cpu::disp_PUSH_0xE5));
        decoder.insert(0xE6 as u16, (Cpu::instr_AND_0xE6, Cpu::disp_AND_0xE6));
        decoder.insert(0xE7 as u16, (Cpu::instr_RST_0xE7, Cpu::disp_RST_0xE7));
        decoder.insert(0xE8 as u16, (Cpu::instr_ADD_0xE8, Cpu::disp_ADD_0xE8));
        decoder.insert(0xE9 as u16, (Cpu::instr_JP_0xE9, Cpu::disp_JP_0xE9));
        decoder.insert(0xEA as u16, (Cpu::instr_LD_0xEA, Cpu::disp_LD_0xEA));
        decoder.insert(0xEE as u16, (Cpu::instr_XOR_0xEE, Cpu::disp_XOR_0xEE));
        decoder.insert(0xEF as u16, (Cpu::instr_RST_0xEF, Cpu::disp_RST_0xEF));
        decoder.insert(0xF0 as u16, (Cpu::instr_LDH_0xF0, Cpu::disp_LDH_0xF0));
        decoder.insert(0xF1 as u16, (Cpu::instr_POP_0xF1, Cpu::disp_POP_0xF1));
        decoder.insert(0xF2 as u16, (Cpu::instr_LD_0xF2, Cpu::disp_LD_0xF2));
        decoder.insert(0xF3 as u16, (Cpu::instr_DI_0xF3, Cpu::disp_DI_0xF3));
        decoder.insert(0xF5 as u16, (Cpu::instr_PUSH_0xF5, Cpu::disp_PUSH_0xF5));
        decoder.insert(0xF6 as u16, (Cpu::instr_OR_0xF6, Cpu::disp_OR_0xF6));
        decoder.insert(0xF7 as u16, (Cpu::instr_RST_0xF7, Cpu::disp_RST_0xF7));
        decoder.insert(0xF8 as u16, (Cpu::instr_LD_0xF8, Cpu::disp_LD_0xF8));
        decoder.insert(0xF9 as u16, (Cpu::instr_LD_0xF9, Cpu::disp_LD_0xF9));
        decoder.insert(0xFA as u16, (Cpu::instr_LD_0xFA, Cpu::disp_LD_0xFA));
        decoder.insert(0xFB as u16, (Cpu::instr_EI_0xFB, Cpu::disp_EI_0xFB));
        decoder.insert(0xFE as u16, (Cpu::instr_CP_0xFE, Cpu::disp_CP_0xFE));
        decoder.insert(0xFF as u16, (Cpu::instr_RST_0xFF, Cpu::disp_RST_0xFF));
        decoder.insert(0xCB00 as u16, (Cpu::instr_RLC_0xCB00, Cpu::disp_RLC_0xCB00));
        decoder.insert(0xCB01 as u16, (Cpu::instr_RLC_0xCB01, Cpu::disp_RLC_0xCB01));
        decoder.insert(0xCB02 as u16, (Cpu::instr_RLC_0xCB02, Cpu::disp_RLC_0xCB02));
        decoder.insert(0xCB03 as u16, (Cpu::instr_RLC_0xCB03, Cpu::disp_RLC_0xCB03));
        decoder.insert(0xCB04 as u16, (Cpu::instr_RLC_0xCB04, Cpu::disp_RLC_0xCB04));
        decoder.insert(0xCB05 as u16, (Cpu::instr_RLC_0xCB05, Cpu::disp_RLC_0xCB05));
        decoder.insert(0xCB06 as u16, (Cpu::instr_RLC_0xCB06, Cpu::disp_RLC_0xCB06));
        decoder.insert(0xCB07 as u16, (Cpu::instr_RLC_0xCB07, Cpu::disp_RLC_0xCB07));
        decoder.insert(0xCB08 as u16, (Cpu::instr_RRC_0xCB08, Cpu::disp_RRC_0xCB08));
        decoder.insert(0xCB09 as u16, (Cpu::instr_RRC_0xCB09, Cpu::disp_RRC_0xCB09));
        decoder.insert(0xCB0A as u16, (Cpu::instr_RRC_0xCB0A, Cpu::disp_RRC_0xCB0A));
        decoder.insert(0xCB0B as u16, (Cpu::instr_RRC_0xCB0B, Cpu::disp_RRC_0xCB0B));
        decoder.insert(0xCB0C as u16, (Cpu::instr_RRC_0xCB0C, Cpu::disp_RRC_0xCB0C));
        decoder.insert(0xCB0D as u16, (Cpu::instr_RRC_0xCB0D, Cpu::disp_RRC_0xCB0D));
        decoder.insert(0xCB0E as u16, (Cpu::instr_RRC_0xCB0E, Cpu::disp_RRC_0xCB0E));
        decoder.insert(0xCB0F as u16, (Cpu::instr_RRC_0xCB0F, Cpu::disp_RRC_0xCB0F));
        decoder.insert(0xCB10 as u16, (Cpu::instr_RL_0xCB10, Cpu::disp_RL_0xCB10));
        decoder.insert(0xCB11 as u16, (Cpu::instr_RL_0xCB11, Cpu::disp_RL_0xCB11));
        decoder.insert(0xCB12 as u16, (Cpu::instr_RL_0xCB12, Cpu::disp_RL_0xCB12));
        decoder.insert(0xCB13 as u16, (Cpu::instr_RL_0xCB13, Cpu::disp_RL_0xCB13));
        decoder.insert(0xCB14 as u16, (Cpu::instr_RL_0xCB14, Cpu::disp_RL_0xCB14));
        decoder.insert(0xCB15 as u16, (Cpu::instr_RL_0xCB15, Cpu::disp_RL_0xCB15));
        decoder.insert(0xCB16 as u16, (Cpu::instr_RL_0xCB16, Cpu::disp_RL_0xCB16));
        decoder.insert(0xCB17 as u16, (Cpu::instr_RL_0xCB17, Cpu::disp_RL_0xCB17));
        decoder.insert(0xCB18 as u16, (Cpu::instr_RR_0xCB18, Cpu::disp_RR_0xCB18));
        decoder.insert(0xCB19 as u16, (Cpu::instr_RR_0xCB19, Cpu::disp_RR_0xCB19));
        decoder.insert(0xCB1A as u16, (Cpu::instr_RR_0xCB1A, Cpu::disp_RR_0xCB1A));
        decoder.insert(0xCB1B as u16, (Cpu::instr_RR_0xCB1B, Cpu::disp_RR_0xCB1B));
        decoder.insert(0xCB1C as u16, (Cpu::instr_RR_0xCB1C, Cpu::disp_RR_0xCB1C));
        decoder.insert(0xCB1D as u16, (Cpu::instr_RR_0xCB1D, Cpu::disp_RR_0xCB1D));
        decoder.insert(0xCB1E as u16, (Cpu::instr_RR_0xCB1E, Cpu::disp_RR_0xCB1E));
        decoder.insert(0xCB1F as u16, (Cpu::instr_RR_0xCB1F, Cpu::disp_RR_0xCB1F));
        decoder.insert(0xCB20 as u16, (Cpu::instr_SLA_0xCB20, Cpu::disp_SLA_0xCB20));
        decoder.insert(0xCB21 as u16, (Cpu::instr_SLA_0xCB21, Cpu::disp_SLA_0xCB21));
        decoder.insert(0xCB22 as u16, (Cpu::instr_SLA_0xCB22, Cpu::disp_SLA_0xCB22));
        decoder.insert(0xCB23 as u16, (Cpu::instr_SLA_0xCB23, Cpu::disp_SLA_0xCB23));
        decoder.insert(0xCB24 as u16, (Cpu::instr_SLA_0xCB24, Cpu::disp_SLA_0xCB24));
        decoder.insert(0xCB25 as u16, (Cpu::instr_SLA_0xCB25, Cpu::disp_SLA_0xCB25));
        decoder.insert(0xCB26 as u16, (Cpu::instr_SLA_0xCB26, Cpu::disp_SLA_0xCB26));
        decoder.insert(0xCB27 as u16, (Cpu::instr_SLA_0xCB27, Cpu::disp_SLA_0xCB27));
        decoder.insert(0xCB28 as u16, (Cpu::instr_SRA_0xCB28, Cpu::disp_SRA_0xCB28));
        decoder.insert(0xCB29 as u16, (Cpu::instr_SRA_0xCB29, Cpu::disp_SRA_0xCB29));
        decoder.insert(0xCB2A as u16, (Cpu::instr_SRA_0xCB2A, Cpu::disp_SRA_0xCB2A));
        decoder.insert(0xCB2B as u16, (Cpu::instr_SRA_0xCB2B, Cpu::disp_SRA_0xCB2B));
        decoder.insert(0xCB2C as u16, (Cpu::instr_SRA_0xCB2C, Cpu::disp_SRA_0xCB2C));
        decoder.insert(0xCB2D as u16, (Cpu::instr_SRA_0xCB2D, Cpu::disp_SRA_0xCB2D));
        decoder.insert(0xCB2E as u16, (Cpu::instr_SRA_0xCB2E, Cpu::disp_SRA_0xCB2E));
        decoder.insert(0xCB2F as u16, (Cpu::instr_SRA_0xCB2F, Cpu::disp_SRA_0xCB2F));
        decoder.insert(0xCB30 as u16, (Cpu::instr_SWAP_0xCB30, Cpu::disp_SWAP_0xCB30));
        decoder.insert(0xCB31 as u16, (Cpu::instr_SWAP_0xCB31, Cpu::disp_SWAP_0xCB31));
        decoder.insert(0xCB32 as u16, (Cpu::instr_SWAP_0xCB32, Cpu::disp_SWAP_0xCB32));
        decoder.insert(0xCB33 as u16, (Cpu::instr_SWAP_0xCB33, Cpu::disp_SWAP_0xCB33));
        decoder.insert(0xCB34 as u16, (Cpu::instr_SWAP_0xCB34, Cpu::disp_SWAP_0xCB34));
        decoder.insert(0xCB35 as u16, (Cpu::instr_SWAP_0xCB35, Cpu::disp_SWAP_0xCB35));
        decoder.insert(0xCB36 as u16, (Cpu::instr_SWAP_0xCB36, Cpu::disp_SWAP_0xCB36));
        decoder.insert(0xCB37 as u16, (Cpu::instr_SWAP_0xCB37, Cpu::disp_SWAP_0xCB37));
        decoder.insert(0xCB38 as u16, (Cpu::instr_SRL_0xCB38, Cpu::disp_SRL_0xCB38));
        decoder.insert(0xCB39 as u16, (Cpu::instr_SRL_0xCB39, Cpu::disp_SRL_0xCB39));
        decoder.insert(0xCB3A as u16, (Cpu::instr_SRL_0xCB3A, Cpu::disp_SRL_0xCB3A));
        decoder.insert(0xCB3B as u16, (Cpu::instr_SRL_0xCB3B, Cpu::disp_SRL_0xCB3B));
        decoder.insert(0xCB3C as u16, (Cpu::instr_SRL_0xCB3C, Cpu::disp_SRL_0xCB3C));
        decoder.insert(0xCB3D as u16, (Cpu::instr_SRL_0xCB3D, Cpu::disp_SRL_0xCB3D));
        decoder.insert(0xCB3E as u16, (Cpu::instr_SRL_0xCB3E, Cpu::disp_SRL_0xCB3E));
        decoder.insert(0xCB3F as u16, (Cpu::instr_SRL_0xCB3F, Cpu::disp_SRL_0xCB3F));
        decoder.insert(0xCB40 as u16, (Cpu::instr_BIT_0xCB40, Cpu::disp_BIT_0xCB40));
        decoder.insert(0xCB41 as u16, (Cpu::instr_BIT_0xCB41, Cpu::disp_BIT_0xCB41));
        decoder.insert(0xCB42 as u16, (Cpu::instr_BIT_0xCB42, Cpu::disp_BIT_0xCB42));
        decoder.insert(0xCB43 as u16, (Cpu::instr_BIT_0xCB43, Cpu::disp_BIT_0xCB43));
        decoder.insert(0xCB44 as u16, (Cpu::instr_BIT_0xCB44, Cpu::disp_BIT_0xCB44));
        decoder.insert(0xCB45 as u16, (Cpu::instr_BIT_0xCB45, Cpu::disp_BIT_0xCB45));
        decoder.insert(0xCB46 as u16, (Cpu::instr_BIT_0xCB46, Cpu::disp_BIT_0xCB46));
        decoder.insert(0xCB47 as u16, (Cpu::instr_BIT_0xCB47, Cpu::disp_BIT_0xCB47));
        decoder.insert(0xCB48 as u16, (Cpu::instr_BIT_0xCB48, Cpu::disp_BIT_0xCB48));
        decoder.insert(0xCB49 as u16, (Cpu::instr_BIT_0xCB49, Cpu::disp_BIT_0xCB49));
        decoder.insert(0xCB4A as u16, (Cpu::instr_BIT_0xCB4A, Cpu::disp_BIT_0xCB4A));
        decoder.insert(0xCB4B as u16, (Cpu::instr_BIT_0xCB4B, Cpu::disp_BIT_0xCB4B));
        decoder.insert(0xCB4C as u16, (Cpu::instr_BIT_0xCB4C, Cpu::disp_BIT_0xCB4C));
        decoder.insert(0xCB4D as u16, (Cpu::instr_BIT_0xCB4D, Cpu::disp_BIT_0xCB4D));
        decoder.insert(0xCB4E as u16, (Cpu::instr_BIT_0xCB4E, Cpu::disp_BIT_0xCB4E));
        decoder.insert(0xCB4F as u16, (Cpu::instr_BIT_0xCB4F, Cpu::disp_BIT_0xCB4F));
        decoder.insert(0xCB50 as u16, (Cpu::instr_BIT_0xCB50, Cpu::disp_BIT_0xCB50));
        decoder.insert(0xCB51 as u16, (Cpu::instr_BIT_0xCB51, Cpu::disp_BIT_0xCB51));
        decoder.insert(0xCB52 as u16, (Cpu::instr_BIT_0xCB52, Cpu::disp_BIT_0xCB52));
        decoder.insert(0xCB53 as u16, (Cpu::instr_BIT_0xCB53, Cpu::disp_BIT_0xCB53));
        decoder.insert(0xCB54 as u16, (Cpu::instr_BIT_0xCB54, Cpu::disp_BIT_0xCB54));
        decoder.insert(0xCB55 as u16, (Cpu::instr_BIT_0xCB55, Cpu::disp_BIT_0xCB55));
        decoder.insert(0xCB56 as u16, (Cpu::instr_BIT_0xCB56, Cpu::disp_BIT_0xCB56));
        decoder.insert(0xCB57 as u16, (Cpu::instr_BIT_0xCB57, Cpu::disp_BIT_0xCB57));
        decoder.insert(0xCB58 as u16, (Cpu::instr_BIT_0xCB58, Cpu::disp_BIT_0xCB58));
        decoder.insert(0xCB59 as u16, (Cpu::instr_BIT_0xCB59, Cpu::disp_BIT_0xCB59));
        decoder.insert(0xCB5A as u16, (Cpu::instr_BIT_0xCB5A, Cpu::disp_BIT_0xCB5A));
        decoder.insert(0xCB5B as u16, (Cpu::instr_BIT_0xCB5B, Cpu::disp_BIT_0xCB5B));
        decoder.insert(0xCB5C as u16, (Cpu::instr_BIT_0xCB5C, Cpu::disp_BIT_0xCB5C));
        decoder.insert(0xCB5D as u16, (Cpu::instr_BIT_0xCB5D, Cpu::disp_BIT_0xCB5D));
        decoder.insert(0xCB5E as u16, (Cpu::instr_BIT_0xCB5E, Cpu::disp_BIT_0xCB5E));
        decoder.insert(0xCB5F as u16, (Cpu::instr_BIT_0xCB5F, Cpu::disp_BIT_0xCB5F));
        decoder.insert(0xCB60 as u16, (Cpu::instr_BIT_0xCB60, Cpu::disp_BIT_0xCB60));
        decoder.insert(0xCB61 as u16, (Cpu::instr_BIT_0xCB61, Cpu::disp_BIT_0xCB61));
        decoder.insert(0xCB62 as u16, (Cpu::instr_BIT_0xCB62, Cpu::disp_BIT_0xCB62));
        decoder.insert(0xCB63 as u16, (Cpu::instr_BIT_0xCB63, Cpu::disp_BIT_0xCB63));
        decoder.insert(0xCB64 as u16, (Cpu::instr_BIT_0xCB64, Cpu::disp_BIT_0xCB64));
        decoder.insert(0xCB65 as u16, (Cpu::instr_BIT_0xCB65, Cpu::disp_BIT_0xCB65));
        decoder.insert(0xCB66 as u16, (Cpu::instr_BIT_0xCB66, Cpu::disp_BIT_0xCB66));
        decoder.insert(0xCB67 as u16, (Cpu::instr_BIT_0xCB67, Cpu::disp_BIT_0xCB67));
        decoder.insert(0xCB68 as u16, (Cpu::instr_BIT_0xCB68, Cpu::disp_BIT_0xCB68));
        decoder.insert(0xCB69 as u16, (Cpu::instr_BIT_0xCB69, Cpu::disp_BIT_0xCB69));
        decoder.insert(0xCB6A as u16, (Cpu::instr_BIT_0xCB6A, Cpu::disp_BIT_0xCB6A));
        decoder.insert(0xCB6B as u16, (Cpu::instr_BIT_0xCB6B, Cpu::disp_BIT_0xCB6B));
        decoder.insert(0xCB6C as u16, (Cpu::instr_BIT_0xCB6C, Cpu::disp_BIT_0xCB6C));
        decoder.insert(0xCB6D as u16, (Cpu::instr_BIT_0xCB6D, Cpu::disp_BIT_0xCB6D));
        decoder.insert(0xCB6E as u16, (Cpu::instr_BIT_0xCB6E, Cpu::disp_BIT_0xCB6E));
        decoder.insert(0xCB6F as u16, (Cpu::instr_BIT_0xCB6F, Cpu::disp_BIT_0xCB6F));
        decoder.insert(0xCB70 as u16, (Cpu::instr_BIT_0xCB70, Cpu::disp_BIT_0xCB70));
        decoder.insert(0xCB71 as u16, (Cpu::instr_BIT_0xCB71, Cpu::disp_BIT_0xCB71));
        decoder.insert(0xCB72 as u16, (Cpu::instr_BIT_0xCB72, Cpu::disp_BIT_0xCB72));
        decoder.insert(0xCB73 as u16, (Cpu::instr_BIT_0xCB73, Cpu::disp_BIT_0xCB73));
        decoder.insert(0xCB74 as u16, (Cpu::instr_BIT_0xCB74, Cpu::disp_BIT_0xCB74));
        decoder.insert(0xCB75 as u16, (Cpu::instr_BIT_0xCB75, Cpu::disp_BIT_0xCB75));
        decoder.insert(0xCB76 as u16, (Cpu::instr_BIT_0xCB76, Cpu::disp_BIT_0xCB76));
        decoder.insert(0xCB77 as u16, (Cpu::instr_BIT_0xCB77, Cpu::disp_BIT_0xCB77));
        decoder.insert(0xCB78 as u16, (Cpu::instr_BIT_0xCB78, Cpu::disp_BIT_0xCB78));
        decoder.insert(0xCB79 as u16, (Cpu::instr_BIT_0xCB79, Cpu::disp_BIT_0xCB79));
        decoder.insert(0xCB7A as u16, (Cpu::instr_BIT_0xCB7A, Cpu::disp_BIT_0xCB7A));
        decoder.insert(0xCB7B as u16, (Cpu::instr_BIT_0xCB7B, Cpu::disp_BIT_0xCB7B));
        decoder.insert(0xCB7C as u16, (Cpu::instr_BIT_0xCB7C, Cpu::disp_BIT_0xCB7C));
        decoder.insert(0xCB7D as u16, (Cpu::instr_BIT_0xCB7D, Cpu::disp_BIT_0xCB7D));
        decoder.insert(0xCB7E as u16, (Cpu::instr_BIT_0xCB7E, Cpu::disp_BIT_0xCB7E));
        decoder.insert(0xCB7F as u16, (Cpu::instr_BIT_0xCB7F, Cpu::disp_BIT_0xCB7F));
        decoder.insert(0xCB80 as u16, (Cpu::instr_RES_0xCB80, Cpu::disp_RES_0xCB80));
        decoder.insert(0xCB81 as u16, (Cpu::instr_RES_0xCB81, Cpu::disp_RES_0xCB81));
        decoder.insert(0xCB82 as u16, (Cpu::instr_RES_0xCB82, Cpu::disp_RES_0xCB82));
        decoder.insert(0xCB83 as u16, (Cpu::instr_RES_0xCB83, Cpu::disp_RES_0xCB83));
        decoder.insert(0xCB84 as u16, (Cpu::instr_RES_0xCB84, Cpu::disp_RES_0xCB84));
        decoder.insert(0xCB85 as u16, (Cpu::instr_RES_0xCB85, Cpu::disp_RES_0xCB85));
        decoder.insert(0xCB86 as u16, (Cpu::instr_RES_0xCB86, Cpu::disp_RES_0xCB86));
        decoder.insert(0xCB87 as u16, (Cpu::instr_RES_0xCB87, Cpu::disp_RES_0xCB87));
        decoder.insert(0xCB88 as u16, (Cpu::instr_RES_0xCB88, Cpu::disp_RES_0xCB88));
        decoder.insert(0xCB89 as u16, (Cpu::instr_RES_0xCB89, Cpu::disp_RES_0xCB89));
        decoder.insert(0xCB8A as u16, (Cpu::instr_RES_0xCB8A, Cpu::disp_RES_0xCB8A));
        decoder.insert(0xCB8B as u16, (Cpu::instr_RES_0xCB8B, Cpu::disp_RES_0xCB8B));
        decoder.insert(0xCB8C as u16, (Cpu::instr_RES_0xCB8C, Cpu::disp_RES_0xCB8C));
        decoder.insert(0xCB8D as u16, (Cpu::instr_RES_0xCB8D, Cpu::disp_RES_0xCB8D));
        decoder.insert(0xCB8E as u16, (Cpu::instr_RES_0xCB8E, Cpu::disp_RES_0xCB8E));
        decoder.insert(0xCB8F as u16, (Cpu::instr_RES_0xCB8F, Cpu::disp_RES_0xCB8F));
        decoder.insert(0xCB90 as u16, (Cpu::instr_RES_0xCB90, Cpu::disp_RES_0xCB90));
        decoder.insert(0xCB91 as u16, (Cpu::instr_RES_0xCB91, Cpu::disp_RES_0xCB91));
        decoder.insert(0xCB92 as u16, (Cpu::instr_RES_0xCB92, Cpu::disp_RES_0xCB92));
        decoder.insert(0xCB93 as u16, (Cpu::instr_RES_0xCB93, Cpu::disp_RES_0xCB93));
        decoder.insert(0xCB94 as u16, (Cpu::instr_RES_0xCB94, Cpu::disp_RES_0xCB94));
        decoder.insert(0xCB95 as u16, (Cpu::instr_RES_0xCB95, Cpu::disp_RES_0xCB95));
        decoder.insert(0xCB96 as u16, (Cpu::instr_RES_0xCB96, Cpu::disp_RES_0xCB96));
        decoder.insert(0xCB97 as u16, (Cpu::instr_RES_0xCB97, Cpu::disp_RES_0xCB97));
        decoder.insert(0xCB98 as u16, (Cpu::instr_RES_0xCB98, Cpu::disp_RES_0xCB98));
        decoder.insert(0xCB99 as u16, (Cpu::instr_RES_0xCB99, Cpu::disp_RES_0xCB99));
        decoder.insert(0xCB9A as u16, (Cpu::instr_RES_0xCB9A, Cpu::disp_RES_0xCB9A));
        decoder.insert(0xCB9B as u16, (Cpu::instr_RES_0xCB9B, Cpu::disp_RES_0xCB9B));
        decoder.insert(0xCB9C as u16, (Cpu::instr_RES_0xCB9C, Cpu::disp_RES_0xCB9C));
        decoder.insert(0xCB9D as u16, (Cpu::instr_RES_0xCB9D, Cpu::disp_RES_0xCB9D));
        decoder.insert(0xCB9E as u16, (Cpu::instr_RES_0xCB9E, Cpu::disp_RES_0xCB9E));
        decoder.insert(0xCB9F as u16, (Cpu::instr_RES_0xCB9F, Cpu::disp_RES_0xCB9F));
        decoder.insert(0xCBA0 as u16, (Cpu::instr_RES_0xCBA0, Cpu::disp_RES_0xCBA0));
        decoder.insert(0xCBA1 as u16, (Cpu::instr_RES_0xCBA1, Cpu::disp_RES_0xCBA1));
        decoder.insert(0xCBA2 as u16, (Cpu::instr_RES_0xCBA2, Cpu::disp_RES_0xCBA2));
        decoder.insert(0xCBA3 as u16, (Cpu::instr_RES_0xCBA3, Cpu::disp_RES_0xCBA3));
        decoder.insert(0xCBA4 as u16, (Cpu::instr_RES_0xCBA4, Cpu::disp_RES_0xCBA4));
        decoder.insert(0xCBA5 as u16, (Cpu::instr_RES_0xCBA5, Cpu::disp_RES_0xCBA5));
        decoder.insert(0xCBA6 as u16, (Cpu::instr_RES_0xCBA6, Cpu::disp_RES_0xCBA6));
        decoder.insert(0xCBA7 as u16, (Cpu::instr_RES_0xCBA7, Cpu::disp_RES_0xCBA7));
        decoder.insert(0xCBA8 as u16, (Cpu::instr_RES_0xCBA8, Cpu::disp_RES_0xCBA8));
        decoder.insert(0xCBA9 as u16, (Cpu::instr_RES_0xCBA9, Cpu::disp_RES_0xCBA9));
        decoder.insert(0xCBAA as u16, (Cpu::instr_RES_0xCBAA, Cpu::disp_RES_0xCBAA));
        decoder.insert(0xCBAB as u16, (Cpu::instr_RES_0xCBAB, Cpu::disp_RES_0xCBAB));
        decoder.insert(0xCBAC as u16, (Cpu::instr_RES_0xCBAC, Cpu::disp_RES_0xCBAC));
        decoder.insert(0xCBAD as u16, (Cpu::instr_RES_0xCBAD, Cpu::disp_RES_0xCBAD));
        decoder.insert(0xCBAE as u16, (Cpu::instr_RES_0xCBAE, Cpu::disp_RES_0xCBAE));
        decoder.insert(0xCBAF as u16, (Cpu::instr_RES_0xCBAF, Cpu::disp_RES_0xCBAF));
        decoder.insert(0xCBB0 as u16, (Cpu::instr_RES_0xCBB0, Cpu::disp_RES_0xCBB0));
        decoder.insert(0xCBB1 as u16, (Cpu::instr_RES_0xCBB1, Cpu::disp_RES_0xCBB1));
        decoder.insert(0xCBB2 as u16, (Cpu::instr_RES_0xCBB2, Cpu::disp_RES_0xCBB2));
        decoder.insert(0xCBB3 as u16, (Cpu::instr_RES_0xCBB3, Cpu::disp_RES_0xCBB3));
        decoder.insert(0xCBB4 as u16, (Cpu::instr_RES_0xCBB4, Cpu::disp_RES_0xCBB4));
        decoder.insert(0xCBB5 as u16, (Cpu::instr_RES_0xCBB5, Cpu::disp_RES_0xCBB5));
        decoder.insert(0xCBB6 as u16, (Cpu::instr_RES_0xCBB6, Cpu::disp_RES_0xCBB6));
        decoder.insert(0xCBB7 as u16, (Cpu::instr_RES_0xCBB7, Cpu::disp_RES_0xCBB7));
        decoder.insert(0xCBB8 as u16, (Cpu::instr_RES_0xCBB8, Cpu::disp_RES_0xCBB8));
        decoder.insert(0xCBB9 as u16, (Cpu::instr_RES_0xCBB9, Cpu::disp_RES_0xCBB9));
        decoder.insert(0xCBBA as u16, (Cpu::instr_RES_0xCBBA, Cpu::disp_RES_0xCBBA));
        decoder.insert(0xCBBB as u16, (Cpu::instr_RES_0xCBBB, Cpu::disp_RES_0xCBBB));
        decoder.insert(0xCBBC as u16, (Cpu::instr_RES_0xCBBC, Cpu::disp_RES_0xCBBC));
        decoder.insert(0xCBBD as u16, (Cpu::instr_RES_0xCBBD, Cpu::disp_RES_0xCBBD));
        decoder.insert(0xCBBE as u16, (Cpu::instr_RES_0xCBBE, Cpu::disp_RES_0xCBBE));
        decoder.insert(0xCBBF as u16, (Cpu::instr_RES_0xCBBF, Cpu::disp_RES_0xCBBF));
        decoder.insert(0xCBC0 as u16, (Cpu::instr_SET_0xCBC0, Cpu::disp_SET_0xCBC0));
        decoder.insert(0xCBC1 as u16, (Cpu::instr_SET_0xCBC1, Cpu::disp_SET_0xCBC1));
        decoder.insert(0xCBC2 as u16, (Cpu::instr_SET_0xCBC2, Cpu::disp_SET_0xCBC2));
        decoder.insert(0xCBC3 as u16, (Cpu::instr_SET_0xCBC3, Cpu::disp_SET_0xCBC3));
        decoder.insert(0xCBC4 as u16, (Cpu::instr_SET_0xCBC4, Cpu::disp_SET_0xCBC4));
        decoder.insert(0xCBC5 as u16, (Cpu::instr_SET_0xCBC5, Cpu::disp_SET_0xCBC5));
        decoder.insert(0xCBC6 as u16, (Cpu::instr_SET_0xCBC6, Cpu::disp_SET_0xCBC6));
        decoder.insert(0xCBC7 as u16, (Cpu::instr_SET_0xCBC7, Cpu::disp_SET_0xCBC7));
        decoder.insert(0xCBC8 as u16, (Cpu::instr_SET_0xCBC8, Cpu::disp_SET_0xCBC8));
        decoder.insert(0xCBC9 as u16, (Cpu::instr_SET_0xCBC9, Cpu::disp_SET_0xCBC9));
        decoder.insert(0xCBCA as u16, (Cpu::instr_SET_0xCBCA, Cpu::disp_SET_0xCBCA));
        decoder.insert(0xCBCB as u16, (Cpu::instr_SET_0xCBCB, Cpu::disp_SET_0xCBCB));
        decoder.insert(0xCBCC as u16, (Cpu::instr_SET_0xCBCC, Cpu::disp_SET_0xCBCC));
        decoder.insert(0xCBCD as u16, (Cpu::instr_SET_0xCBCD, Cpu::disp_SET_0xCBCD));
        decoder.insert(0xCBCE as u16, (Cpu::instr_SET_0xCBCE, Cpu::disp_SET_0xCBCE));
        decoder.insert(0xCBCF as u16, (Cpu::instr_SET_0xCBCF, Cpu::disp_SET_0xCBCF));
        decoder.insert(0xCBD0 as u16, (Cpu::instr_SET_0xCBD0, Cpu::disp_SET_0xCBD0));
        decoder.insert(0xCBD1 as u16, (Cpu::instr_SET_0xCBD1, Cpu::disp_SET_0xCBD1));
        decoder.insert(0xCBD2 as u16, (Cpu::instr_SET_0xCBD2, Cpu::disp_SET_0xCBD2));
        decoder.insert(0xCBD3 as u16, (Cpu::instr_SET_0xCBD3, Cpu::disp_SET_0xCBD3));
        decoder.insert(0xCBD4 as u16, (Cpu::instr_SET_0xCBD4, Cpu::disp_SET_0xCBD4));
        decoder.insert(0xCBD5 as u16, (Cpu::instr_SET_0xCBD5, Cpu::disp_SET_0xCBD5));
        decoder.insert(0xCBD6 as u16, (Cpu::instr_SET_0xCBD6, Cpu::disp_SET_0xCBD6));
        decoder.insert(0xCBD7 as u16, (Cpu::instr_SET_0xCBD7, Cpu::disp_SET_0xCBD7));
        decoder.insert(0xCBD8 as u16, (Cpu::instr_SET_0xCBD8, Cpu::disp_SET_0xCBD8));
        decoder.insert(0xCBD9 as u16, (Cpu::instr_SET_0xCBD9, Cpu::disp_SET_0xCBD9));
        decoder.insert(0xCBDA as u16, (Cpu::instr_SET_0xCBDA, Cpu::disp_SET_0xCBDA));
        decoder.insert(0xCBDB as u16, (Cpu::instr_SET_0xCBDB, Cpu::disp_SET_0xCBDB));
        decoder.insert(0xCBDC as u16, (Cpu::instr_SET_0xCBDC, Cpu::disp_SET_0xCBDC));
        decoder.insert(0xCBDD as u16, (Cpu::instr_SET_0xCBDD, Cpu::disp_SET_0xCBDD));
        decoder.insert(0xCBDE as u16, (Cpu::instr_SET_0xCBDE, Cpu::disp_SET_0xCBDE));
        decoder.insert(0xCBDF as u16, (Cpu::instr_SET_0xCBDF, Cpu::disp_SET_0xCBDF));
        decoder.insert(0xCBE0 as u16, (Cpu::instr_SET_0xCBE0, Cpu::disp_SET_0xCBE0));
        decoder.insert(0xCBE1 as u16, (Cpu::instr_SET_0xCBE1, Cpu::disp_SET_0xCBE1));
        decoder.insert(0xCBE2 as u16, (Cpu::instr_SET_0xCBE2, Cpu::disp_SET_0xCBE2));
        decoder.insert(0xCBE3 as u16, (Cpu::instr_SET_0xCBE3, Cpu::disp_SET_0xCBE3));
        decoder.insert(0xCBE4 as u16, (Cpu::instr_SET_0xCBE4, Cpu::disp_SET_0xCBE4));
        decoder.insert(0xCBE5 as u16, (Cpu::instr_SET_0xCBE5, Cpu::disp_SET_0xCBE5));
        decoder.insert(0xCBE6 as u16, (Cpu::instr_SET_0xCBE6, Cpu::disp_SET_0xCBE6));
        decoder.insert(0xCBE7 as u16, (Cpu::instr_SET_0xCBE7, Cpu::disp_SET_0xCBE7));
        decoder.insert(0xCBE8 as u16, (Cpu::instr_SET_0xCBE8, Cpu::disp_SET_0xCBE8));
        decoder.insert(0xCBE9 as u16, (Cpu::instr_SET_0xCBE9, Cpu::disp_SET_0xCBE9));
        decoder.insert(0xCBEA as u16, (Cpu::instr_SET_0xCBEA, Cpu::disp_SET_0xCBEA));
        decoder.insert(0xCBEB as u16, (Cpu::instr_SET_0xCBEB, Cpu::disp_SET_0xCBEB));
        decoder.insert(0xCBEC as u16, (Cpu::instr_SET_0xCBEC, Cpu::disp_SET_0xCBEC));
        decoder.insert(0xCBED as u16, (Cpu::instr_SET_0xCBED, Cpu::disp_SET_0xCBED));
        decoder.insert(0xCBEE as u16, (Cpu::instr_SET_0xCBEE, Cpu::disp_SET_0xCBEE));
        decoder.insert(0xCBEF as u16, (Cpu::instr_SET_0xCBEF, Cpu::disp_SET_0xCBEF));
        decoder.insert(0xCBF0 as u16, (Cpu::instr_SET_0xCBF0, Cpu::disp_SET_0xCBF0));
        decoder.insert(0xCBF1 as u16, (Cpu::instr_SET_0xCBF1, Cpu::disp_SET_0xCBF1));
        decoder.insert(0xCBF2 as u16, (Cpu::instr_SET_0xCBF2, Cpu::disp_SET_0xCBF2));
        decoder.insert(0xCBF3 as u16, (Cpu::instr_SET_0xCBF3, Cpu::disp_SET_0xCBF3));
        decoder.insert(0xCBF4 as u16, (Cpu::instr_SET_0xCBF4, Cpu::disp_SET_0xCBF4));
        decoder.insert(0xCBF5 as u16, (Cpu::instr_SET_0xCBF5, Cpu::disp_SET_0xCBF5));
        decoder.insert(0xCBF6 as u16, (Cpu::instr_SET_0xCBF6, Cpu::disp_SET_0xCBF6));
        decoder.insert(0xCBF7 as u16, (Cpu::instr_SET_0xCBF7, Cpu::disp_SET_0xCBF7));
        decoder.insert(0xCBF8 as u16, (Cpu::instr_SET_0xCBF8, Cpu::disp_SET_0xCBF8));
        decoder.insert(0xCBF9 as u16, (Cpu::instr_SET_0xCBF9, Cpu::disp_SET_0xCBF9));
        decoder.insert(0xCBFA as u16, (Cpu::instr_SET_0xCBFA, Cpu::disp_SET_0xCBFA));
        decoder.insert(0xCBFB as u16, (Cpu::instr_SET_0xCBFB, Cpu::disp_SET_0xCBFB));
        decoder.insert(0xCBFC as u16, (Cpu::instr_SET_0xCBFC, Cpu::disp_SET_0xCBFC));
        decoder.insert(0xCBFD as u16, (Cpu::instr_SET_0xCBFD, Cpu::disp_SET_0xCBFD));
        decoder.insert(0xCBFE as u16, (Cpu::instr_SET_0xCBFE, Cpu::disp_SET_0xCBFE));
        decoder.insert(0xCBFF as u16, (Cpu::instr_SET_0xCBFF, Cpu::disp_SET_0xCBFF));
        decoder
    };
}
