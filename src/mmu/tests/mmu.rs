use super::super::*;

#[test]
fn new() {
    let m = Mmu::new();
    // Ensure memory is zero initialized
    for i in 0us..0x10000 {
        assert_eq!(m.memory[i], 0);
    }
}

#[test]
fn write8() {
    let mut m = Mmu::new();
    m.write8(10, 0x42);
    // Ensure there is no overflow on adjacent cells
    assert_eq!(m.memory[ 9], 0x00);
    assert_eq!(m.memory[10], 0x42);
    assert_eq!(m.memory[11], 0x00);
}

#[test]
fn write16() {
    // GameBoy is little-endian
    let mut m = Mmu::new();
    m.write16(10, 0x1337);
    // Ensure there is no overflow on adjacent cells
    assert_eq!(m.memory[ 8], 0x00);
    assert_eq!(m.memory[ 9], 0x00);
    assert_eq!(m.memory[10], 0x37);
    assert_eq!(m.memory[11], 0x13);
    assert_eq!(m.memory[12], 0x00);
    assert_eq!(m.memory[13], 0x00);
}

#[test]
fn read8() {
    let mut m = Mmu::new();
    m.memory[10] = 0x42;
    // Ensure value is there
    assert_eq!(m.read8( 9), 0x00);
    assert_eq!(m.read8(10), 0x42);
    assert_eq!(m.read8(11), 0x00);
}

#[test]
fn read16() {
    // GameBoy is little-endian
    let mut m = Mmu::new();
    m.memory[10] = 0x37;
    m.memory[11] = 0x13;
    // Ensure value is there
    assert_eq!(m.read8( 9), 0x00);
    assert_eq!(m.read8(10), 0x37);
    assert_eq!(m.read8(11), 0x13);
    assert_eq!(m.read8(12), 0x00);

    assert_eq!(m.read16( 8), 0x0000);
    assert_eq!(m.read16( 9), 0x3700);
    assert_eq!(m.read16(10), 0x1337);
    assert_eq!(m.read16(11), 0x0013);
    assert_eq!(m.read16(12), 0x0000);
}
