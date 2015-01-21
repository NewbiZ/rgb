// Inject rgb::cpu::* names in rgb::*
pub use self::cpu::Cpu;
pub use self::cpu::Flag;

// Import cpu::*
mod cpu;
