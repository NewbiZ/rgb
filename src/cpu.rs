#![allow(dead_code)]
#![allow(unstable)]
#![allow(unused_must_use)]

use std::fmt;
use std::u8;

pub enum Flag {
    Zero      = 0x80,
    Operation = 0x40,
    HalfCarry = 0x20,
    Carry     = 0x10,
    None      = 0x00,
}

pub struct Cpu {
    // Registers
    a:  u8, // General purpose
    b:  u8, // General purpose
    c:  u8, // General purpose
    d:  u8, // General purpose
    e:  u8, // General purpose
    h:  u8, // General purpose
    l:  u8, // General purpose
    f:  u8, // Flag register
    pc: u8, // Program counter
    sp: u8, // Stack pointer
    
    // Clocks
    m: u8,
    t: u8,
}

// ==============================================
// Implementation
// ==============================================
impl Cpu {
    pub fn new() -> Cpu {
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

    pub fn instr_0x84(&mut self) {
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
