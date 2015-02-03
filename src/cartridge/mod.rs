#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]

use std::old_io::File;

#[cfg(test)]
mod tests;

/// This struct represents a cartridge.
pub struct Cartridge {
    rom0: [u8; 0x4000],
    romN: [u8; 0x4000],
    ram:  [u8; 0x2000],
}

// ==============================================
// Implementation
// ==============================================

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            rom0: [0; 0x4000],
            romN: [0; 0x4000],
            ram:  [0; 0x2000],
        }
    }

    pub fn from_file<'a>(path: &'a Path) -> Cartridge {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("error: {}", e.desc),
        };

        let stat = match file.stat() {
            Ok(s) => s,
            Err(e) => panic!("error: {}", e.desc),
        };

        if stat.size > 0x4000 {
            panic!("error: cannot read ROM size > 2KB");
        }

        let mut cart = Cartridge::new();
        match file.read(&mut cart.rom0) {
            Ok(s) => s,
            Err(e) => panic!("error: {}", e.desc),
        };

        return cart;
    }
}
