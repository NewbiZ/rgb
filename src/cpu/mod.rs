#![allow(dead_code)]
#![allow(unstable)]
#![allow(unused_must_use)]

use std::fmt;
use std::u8;

#[cfg(test)]
mod tests;

/// All different values of flags that could
/// end up in `Cpu` register `Cpu::f`.
pub enum Flag {
    /// Previous operation resulted in zero
    Zero      = 0x80,
    /// Previous operation was a substraction
    Operation = 0x40,
    /// Previous operation resulted in an half carry
    HalfCarry = 0x20,
    /// Previous operation resulted in an overflow
    Carry     = 0x10,
    /// Previous operation triggered no flag
    None      = 0x00,
}

/// This struct models a GameBoy Z80 processor.
pub struct Cpu {
    /// General purpose register
    pub a:  u8,
    /// General purpose register
    pub b:  u8,
    /// General purpose register
    pub c:  u8,
    /// General purpose register
    pub d:  u8,
    /// General purpose register
    pub e:  u8,
    /// General purpose register
    pub h:  u8,
    /// General purpose register
    pub l:  u8,
    /// Flag register
    pub f:  u8,
    /// Program counter register
    pub pc: u8,
    /// Stack pointer register
    pub sp: u8,
    /// Clock
    pub m: u8,
    /// Clock
    pub t: u8,
}

// ==============================================
// Implementation
// ==============================================
impl Cpu {
    pub fn new() -> Cpu {
        //! Create a new `Cpu`. All registers and clocks should be set to 0.
        Cpu {
            a:  0,
            b:  0,
            c:  0,
            d:  0,
            e:  0,
            h:  0,
            l:  0,
            f:  Flag::None as u8,
            pc: 0,
            sp: 0,
            m:  0,
            t:  0,
        }
    }

    pub fn reset(&mut self) {
        //! Reset all `Cpu` registers and clocks to 0.
        self.a  = 0;
        self.b  = 0;
        self.c  = 0;
        self.d  = 0;
        self.e  = 0;
        self.h  = 0;
        self.l  = 0;
        self.f  = Flag::None as u8;
        self.pc = 0;
        self.sp = 0;
        self.m  = 0;
        self.t  = 0;
    }

    pub fn instr_0x84(&mut self) {
        //! Add register E to register A and set flags in register F if carry or zero.
        // Reset flags
        self.f = 0;
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
        self.m = 1;
        self.t = 4;
    }
}

// ==============================================
// Traits
// ==============================================

// Type is shallow-copyable
impl Copy for Cpu {}
impl Copy for Flag {}

macro_rules! match_bitmask {
    ($bits:expr, $($mask:expr => $blk:block), +) => ({
        $(if ($bits & $mask) != 0 || ($bits == $mask)  { $blk })+
    });
}

// Type is formattable to string
impl fmt::String for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Cpu {\n");
        f.write_fmt(format_args!("  a:  {},\n", self.a));
        f.write_fmt(format_args!("  b:  {},\n", self.b));
        f.write_fmt(format_args!("  c:  {},\n", self.c));
        f.write_fmt(format_args!("  d:  {},\n", self.d));
        f.write_fmt(format_args!("  e:  {},\n", self.e));
        f.write_fmt(format_args!("  h:  {},\n", self.h));
        f.write_fmt(format_args!("  l:  {},\n", self.l));

        f.write_fmt(format_args!("  f:  0x{:X} =", self.f));
        match_bitmask!(self.f,
            Flag::Zero      as u8 => { f.write_str(" Zero(0x80)");      },
            Flag::Operation as u8 => { f.write_str(" Operation(0x40)"); },
            Flag::HalfCarry as u8 => { f.write_str(" HalfCarry(0x20)"); },
            Flag::Carry     as u8 => { f.write_str(" Carry(0x10)");     },
            Flag::None      as u8 => { f.write_str(" None(0x00)");      }
        );
        f.write_str(",\n");

        f.write_fmt(format_args!("  pc: {},\n", self.pc));
        f.write_fmt(format_args!("  sp: {},\n", self.sp));
        f.write_fmt(format_args!("  m:  {},\n", self.m));
        f.write_fmt(format_args!("  t:  {},\n", self.t));
        f.write_str("}\n")
    }
}

