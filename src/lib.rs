//! Rust Game Boy emulator.
//! Linked is built as `rgbld`, assembler as `rgbas` and emulator
//! as `rgbemu`. The `rgb` library itself can be used to create your
//! own emulator.

// Inject rgb::cpu::* names in rgb::*
pub use self::cpu::*;

// Import cpu::*
mod cpu;
