#![allow(dead_code)]
#![allow(unstable)]
#![allow(unused_must_use)]
#![allow(missing_copy_implementations)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]

use std::io::File;
use std::path::Path;

#[cfg(test)]
mod tests;

/// This struct represents a cartridge.
struct Cartridge {
    rom0: [u8; 0x4000],
}

// ==============================================
// Implementation
// ==============================================

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            rom0: [0; 0x4000],
        }
    }

    pub fn from_file(path: String) -> Cartridge {
        let mut file = match File::open(&Path::new(path)) {
            Ok(f) => f,
            Err(e) => panic!("error: cannot open cartridge file."),
        };

        let stat = match file.stat() {
            Ok(s) => s,
            Err(e) => panic!("error: cannot stat cartridge file."),
        };

        if stat.size > 0x4000 {
            panic!("error: cannot read ROM size > 2KB");
        }

        let mut cart = Cartridge::new();
        let size = match file.read(&mut cart.rom0) {
            Ok(s) => s,
            Err(e) => panic!("error: cannot read cartridge file."),
        };

        return cart;
    }
}
