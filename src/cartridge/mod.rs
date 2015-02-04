#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]

use std::old_io::File;
use std::str;

#[cfg(test)]
mod tests;

/// This struct represents a cartridge.
pub struct Cartridge<'a> {
    rom0: [u8; 0x4000],
    romN: [u8; 0x4000],
    ram:  [u8; 0x2000],
}

// ==============================================
// Implementation
// ==============================================

impl<'a> Cartridge<'a> {
    pub fn new() -> Cartridge<'a> {
        Cartridge {
            rom0: [0; 0x4000],
            romN: [0; 0x4000],
            ram:  [0; 0x2000],
        }
    }

    pub fn from_file<'b>(path: &'b Path) -> Cartridge<'a> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("error: {}", e.desc),
        };

        let stat = match file.stat() {
            Ok(s) => s,
            Err(e) => panic!("error: {}", e.desc),
        };

        let mut cart = Cartridge::new();

        // Loading bank 0
        match file.read(&mut cart.rom0) {
            Ok(s) => s,
            Err(e) => panic!("error: {}", e.desc),
        };

        // Loading bank 1-N
        if stat.size >= 0x4000 {
            match file.read(&mut cart.romN) {
                Ok(s)  => s,
                Err(e) => panic!("error: {}", e.desc),
            };
        }

        return cart;
    }

    pub fn title(&self) -> & str {
        match str::from_utf8(&self.rom0[0x134..0x143]) {
            Ok(s)  => s,
            Err(e) => panic!("error: failed to retrieve cartridge title {:?}", e),
        }
    }
}
