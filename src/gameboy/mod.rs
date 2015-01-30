#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]

use super::cpu::Cpu;

#[cfg(test)]
mod tests;

pub struct GameBoy {
    cpu: Cpu,
}

// ==============================================
// Implementation
// ==============================================

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy {
            cpu: Cpu::new(),
        }
    }
}
