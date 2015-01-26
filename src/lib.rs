//! Rust Game Boy emulator.
//! Linked is built as `rgbld`, assembler as `rgbas` and emulator
//! as `rgbemu`. The `rgb` library itself can be used to create your
//! own emulator.

// ==============================================
// Inject rgb::*::* names in rgb::*
// ==============================================
// cpu
pub use self::cpu::Cpu;
pub use self::cpu::Flag;
// mmu
pub use self::mmu::Mmu;
// gameboy
pub use self::gameboy::GameBoy;

// ==============================================
// Import generated configuration
// ==============================================
include!(concat!(env!("OUT_DIR"), "/config.rs"));

// ==============================================
// Import submodules
// ==============================================
// Keep util first for its macros need
// to be available to subsequent modules.
#[macro_use]
mod util;
mod cpu;
mod mmu;
mod gameboy;
mod cartridge;
