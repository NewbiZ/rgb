#![allow(dead_code)]
#![allow(unstable)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]

use std::fmt;

#[cfg(test)]
mod tests;

/// This struct models a memory management unit. A small piece
/// of hardware that acts as proxy between the processor and the
/// actual memory. This allows easy address space translation (for
/// memory banking, etc.).
pub struct Mmu {
    /// For now, the `Mmu` just uses a single memory array
    /// of 2^16 bytes.
    memory: [u8; 0x10000],
}

// ==============================================
// Implementation
// ==============================================

impl Mmu {
    pub fn new() -> Mmu {
        //! Create a new `Mmu`. All of its memory is zero initialized.
        Mmu {
            memory: [0; 0x10000],
        }
    }

    pub fn write8(&mut self, address:u16, data: u8) {
        //! Write a single byte to memory
        self.memory[address as usize] = data;
    }

    pub fn write16(&mut self, address: u16, data: u16) {
        //! Write a single word to memory
        self.memory[address as usize+1] = (data >> 8) as u8;
        self.memory[address as usize]   = data as u8;
    }

    pub fn read8(&self, address: u16) -> u8 {
        //! Read a single byte from memory
        self.memory[address as usize]
    }

    pub fn read16(&self, address: u16) -> u16 {
        //! Read a single word from memory
        ((self.memory[address as usize+1] as u16) << 8) + self.memory[address as usize] as u16
    }
}

// ==============================================
// Traits
// ==============================================

// Type is formattable to string
impl fmt::Debug for Mmu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Mmu { memory: [...] }")
    }
}
