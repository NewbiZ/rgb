use super::super::*;

#[test]
fn new() {
    // Check that a newly created Cpu is zero initialized
    let c = Cpu::new();

    assert_eq!(c.a,  0);
    assert_eq!(c.b,  0);
    assert_eq!(c.c,  0);
    assert_eq!(c.d,  0);
    assert_eq!(c.e,  0);
    assert_eq!(c.h,  0);
    assert_eq!(c.l,  0);
    assert_eq!(c.f,  Flag::None as u8);
    assert_eq!(c.pc, 0);
    assert_eq!(c.sp, 0);
    assert_eq!(c.m,  0);
    assert_eq!(c.t,  0);
}

#[test]
fn reset() {
    // Check that after a reset, every register is set to 0.
    let mut c = Cpu::new();

    c.a =  1;
    c.b =  2;
    c.c =  3;
    c.d =  4;
    c.e =  5;
    c.h =  6;
    c.l =  7;
    c.f =  Flag::Carry as u8;
    c.pc = 8;
    c.sp = 9;
    c.m =  10;
    c.t =  11;

    c.reset();

    assert_eq!(c.a,  0);
    assert_eq!(c.b,  0);
    assert_eq!(c.c,  0);
    assert_eq!(c.d,  0);
    assert_eq!(c.e,  0);
    assert_eq!(c.h,  0);
    assert_eq!(c.l,  0);
    assert_eq!(c.f,  Flag::None as u8);
    assert_eq!(c.pc, 0);
    assert_eq!(c.sp, 0);
    assert_eq!(c.m,  0);
    assert_eq!(c.t,  0);
}

#[test]
fn instr_RST_0xE7() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xE7();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x20);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xDF() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xDF();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x18);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xD7() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xD7();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x10);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xEF() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xEF();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x28);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xF7() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xF7();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x30);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xFF() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xFF();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x38);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xC7() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xC7();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x00);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_RST_0xCF() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.instr_RST_0xCF();
    assert_eq!(c.mmu.read16(8), 6);
    assert_eq!(c.pc, 0x08);
    assert_eq!(c.sp, 8);
}

#[test]
fn instr_PUSH_0xD5() {
    let mut c = Cpu::new();

    c.reset();
    c.d  = 0x01;
    c.e  = 0x02;
    c.pc = 5;
    c.sp = 10;
    c.instr_PUSH_0xD5();
    assert_eq!(c.d,  0x01);
    assert_eq!(c.e,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 8);
    assert_eq!(c.mmu.read16(c.sp), 0x0102);
}

#[test]
fn instr_PUSH_0xE5() {
    let mut c = Cpu::new();

    c.reset();
    c.h  = 0x01;
    c.l  = 0x02;
    c.pc = 5;
    c.sp = 10;
    c.instr_PUSH_0xE5();
    assert_eq!(c.h,  0x01);
    assert_eq!(c.l,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 8);
    assert_eq!(c.mmu.read16(c.sp), 0x0102);
}

#[test]
fn instr_PUSH_0xF5() {
    let mut c = Cpu::new();

    c.reset();
    c.a  = 0x01;
    c.f  = 0x02;
    c.pc = 5;
    c.sp = 10;
    c.instr_PUSH_0xF5();
    assert_eq!(c.a,  0x01);
    assert_eq!(c.f,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 8);
    assert_eq!(c.mmu.read16(c.sp), 0x0102);
}

#[test]
fn instr_PUSH_0xC5() {
    let mut c = Cpu::new();

    c.reset();
    c.b  = 0x01;
    c.c  = 0x02;
    c.pc = 5;
    c.sp = 10;
    c.instr_PUSH_0xC5();
    assert_eq!(c.b,  0x01);
    assert_eq!(c.c,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 8);
    assert_eq!(c.mmu.read16(c.sp), 0x0102);
}

#[test]
fn instr_POP_0xD1() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.mmu.write16(10, 0x0102);
    c.instr_POP_0xD1();
    assert_eq!(c.d,  0x01);
    assert_eq!(c.e,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 12);
}

#[test]
fn instr_POP_0xE1() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.mmu.write16(10, 0x0102);
    c.instr_POP_0xE1();
    assert_eq!(c.h,  0x01);
    assert_eq!(c.l,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 12);
}

#[test]
fn instr_POP_0xF1() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.mmu.write16(10, 0x0102);
    c.instr_POP_0xF1();
    assert_eq!(c.a,  0x01);
    assert_eq!(c.f,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 12);
}

#[test]
fn instr_POP_0xC1() {
    let mut c = Cpu::new();

    c.reset();
    c.pc = 5;
    c.sp = 10;
    c.mmu.write16(10, 0x0102);
    c.instr_POP_0xC1();
    assert_eq!(c.b,  0x01);
    assert_eq!(c.c,  0x02);
    assert_eq!(c.pc, 6);
    assert_eq!(c.sp, 12);
}

#[test]
fn instr_AND_0xE6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.mmu.write8(1, 0b00001111 as u8);
    c.instr_AND_0xE6();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.mmu.write8(1, 0b10101010 as u8);
    c.instr_AND_0xE6();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA7() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.instr_AND_0xA7();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01010101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.instr_AND_0xA7();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b00001111);
    c.instr_AND_0xA6();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b10101010 as u8);
    c.instr_AND_0xA6();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA5() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.l = 0b00001111 as u8;
    c.instr_AND_0xA5();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.l = 0b10101010 as u8;
    c.instr_AND_0xA5();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA4() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0b00001111 as u8;
    c.instr_AND_0xA4();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0b10101010 as u8;
    c.instr_AND_0xA4();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA3() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.e = 0b00001111 as u8;
    c.instr_AND_0xA3();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.e = 0b10101010 as u8;
    c.instr_AND_0xA3();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA2() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.d = 0b00001111 as u8;
    c.instr_AND_0xA2();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.d = 0b10101010 as u8;
    c.instr_AND_0xA2();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA1() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.c = 0b00001111 as u8;
    c.instr_AND_0xA1();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.c = 0b10101010 as u8;
    c.instr_AND_0xA1();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_AND_0xA0() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.b = 0b00001111 as u8;
    c.instr_AND_0xA0();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000101 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.b = 0b10101010 as u8;
    c.instr_AND_0xA0();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::HalfCarry as u8 | Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xA9() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.c = 0b00001111 as u8;
    c.instr_XOR_0xA9();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.c = 0b01010101 as u8;
    c.instr_XOR_0xA9();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xA8() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.b = 0b00001111 as u8;
    c.instr_XOR_0xA8();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.b = 0b01010101 as u8;
    c.instr_XOR_0xA8();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAF() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.instr_XOR_0xAF();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAE() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b00001111);
    c.instr_XOR_0xAE();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b01010101 as u8);
    c.instr_XOR_0xAE();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAD() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.l = 0b00001111 as u8;
    c.instr_XOR_0xAD();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.l = 0b01010101 as u8;
    c.instr_XOR_0xAD();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAC() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0b00001111 as u8;
    c.instr_XOR_0xAC();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0b01010101 as u8;
    c.instr_XOR_0xAC();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAB() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.e = 0b00001111 as u8;
    c.instr_XOR_0xAB();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.e = 0b01010101 as u8;
    c.instr_XOR_0xAB();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xAA() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.d = 0b00001111 as u8;
    c.instr_XOR_0xAA();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.d = 0b01010101 as u8;
    c.instr_XOR_0xAA();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_XOR_0xEE() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.mmu.write8(1, 0b00001111 as u8);
    c.instr_XOR_0xEE();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b01011010 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b01010101 as u8;
    c.mmu.write8(1, 0b01010101 as u8);
    c.instr_XOR_0xEE();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_NOP_0x00() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_NOP_0x00();
    c.instr_NOP_0x00();

    assert_eq!(c.a,  0);
    assert_eq!(c.b,  0);
    assert_eq!(c.c,  0);
    assert_eq!(c.d,  0);
    assert_eq!(c.e,  0);
    assert_eq!(c.h,  0);
    assert_eq!(c.l,  0);
    assert_eq!(c.f,  Flag::None as u8);
    assert_eq!(c.pc, 2);
    assert_eq!(c.sp, 0);
    assert_eq!(c.m,  2);
    assert_eq!(c.t,  8);
}

#[test]
fn instr_OR_0xB2() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.d = 0b00001111 as u8;
    c.instr_OR_0xB2();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.d = 0b00000000 as u8;
    c.instr_OR_0xB2();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB3() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.e = 0b00001111 as u8;
    c.instr_OR_0xB3();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.e = 0b00000000 as u8;
    c.instr_OR_0xB3();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB0() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.b = 0b00001111 as u8;
    c.instr_OR_0xB0();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.b = 0b00000000 as u8;
    c.instr_OR_0xB0();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB1() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.c = 0b00001111 as u8;
    c.instr_OR_0xB1();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.c = 0b00000000 as u8;
    c.instr_OR_0xB1();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b00001111);
    c.instr_OR_0xB6();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.h = 0;
    c.l = 10;
    c.mmu.write8(10, 0b00000000 as u8);
    c.instr_OR_0xB6();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB7() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.instr_OR_0xB7();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01010101 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.instr_OR_0xB7();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB4() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.h = 0b00001111 as u8;
    c.instr_OR_0xB4();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.h = 0b00000000 as u8;
    c.instr_OR_0xB4();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xB5() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.l = 0b00001111 as u8;
    c.instr_OR_0xB5();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.l = 0b00000000 as u8;
    c.instr_OR_0xB5();
    assert_eq!(c.pc, 1);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_OR_0xF6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b01010101 as u8;
    c.mmu.write8(1, 0b00001111 as u8);
    c.instr_OR_0xF6();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b01011111 as u8);
    assert_eq!(c.f, Flag::None as u8);

    c.reset();
    c.a = 0b00000000 as u8;
    c.mmu.write8(1, 0b00000000 as u8);
    c.instr_OR_0xF6();
    assert_eq!(c.pc, 2);
    assert_eq!(c.a, 0b00000000 as u8);
    assert_eq!(c.f, Flag::Zero as u8);
}

#[test]
fn instr_JR_0x28() {
    let mut c = Cpu::new();

    c.reset();
    c.f = Flag::None as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x28();
    assert_eq!(c.pc, 2);
    assert_eq!(c.m, 2);
    assert_eq!(c.t, 8);

    c.reset();
    c.f = Flag::Zero as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x28();
    assert_eq!(c.pc, 5);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);

    c.reset();
    c.pc = 5;
    c.f = Flag::Zero as u8;
    c.mmu.write8(6, -5 as u8);
    c.instr_JR_0x28();
    assert_eq!(c.pc, 0);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);
}

#[test]
fn instr_JR_0x20() {
    let mut c = Cpu::new();

    c.reset();
    c.f = Flag::Zero as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x20();
    assert_eq!(c.pc, 2);
    assert_eq!(c.m, 2);
    assert_eq!(c.t, 8);

    c.reset();
    c.f = Flag::None as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x20();
    assert_eq!(c.pc, 5);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);

    c.reset();
    c.pc = 5;
    c.f = Flag::None as u8;
    c.mmu.write8(6, -5 as u8);
    c.instr_JR_0x20();
    assert_eq!(c.pc, 0);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);
}

#[test]
fn instr_JR_0x30() {
    let mut c = Cpu::new();

    c.reset();
    c.f = Flag::Carry as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x30();
    assert_eq!(c.pc, 2);
    assert_eq!(c.m, 2);
    assert_eq!(c.t, 8);

    c.reset();
    c.f = Flag::None as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x30();
    assert_eq!(c.pc, 5);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);

    c.reset();
    c.pc = 5;
    c.f = Flag::None as u8;
    c.mmu.write8(6, -5 as u8);
    c.instr_JR_0x30();
    assert_eq!(c.pc, 0);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);
}

#[test]
fn instr_JR_0x38() {
    let mut c = Cpu::new();

    c.reset();
    c.f = Flag::None as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x38();
    assert_eq!(c.pc, 2);
    assert_eq!(c.m, 2);
    assert_eq!(c.t, 8);

    c.reset();
    c.f = Flag::Carry as u8;
    c.mmu.write8(1, 5);
    c.instr_JR_0x38();
    assert_eq!(c.pc, 5);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);

    c.reset();
    c.pc = 5;
    c.f = Flag::Carry as u8;
    c.mmu.write8(6, -5 as u8);
    c.instr_JR_0x38();
    assert_eq!(c.pc, 0);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);
}

#[test]
fn instr_JR_0x18() {
    let mut c = Cpu::new();

    c.reset();
    c.mmu.write8(1, 5);
    c.instr_JR_0x18();
    assert_eq!(c.pc, 5);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);

    c.reset();
    c.pc = 5;
    c.mmu.write8(6, -5 as u8);
    c.instr_JR_0x18();
    assert_eq!(c.pc, 0);
    assert_eq!(c.m, 3);
    assert_eq!(c.t, 12);
}

#[test]
fn instr_SET_0xCBC0() {
    let mut c = Cpu::new();

    c.reset();
    c.b = 0b00000000;
    c.instr_SET_0xCBC0();
    assert_eq!(c.b, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC1() {
    let mut c = Cpu::new();

    c.reset();
    c.c = 0b00000000;
    c.instr_SET_0xCBC1();
    assert_eq!(c.c, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC3() {
    let mut c = Cpu::new();

    c.reset();
    c.e = 0b00000000;
    c.instr_SET_0xCBC3();
    assert_eq!(c.e, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC4() {
    let mut c = Cpu::new();

    c.reset();
    c.h = 0b00000000;
    c.instr_SET_0xCBC4();
    assert_eq!(c.h, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC5() {
    let mut c = Cpu::new();

    c.reset();
    c.l = 0b00000000;
    c.instr_SET_0xCBC5();
    assert_eq!(c.l, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC7() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 0b00000000;
    c.instr_SET_0xCBC7();
    assert_eq!(c.a, 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SET_0xCBC6() {
    let mut c = Cpu::new();

    c.reset();
    c.mmu.write8(0x0005, 0b00000000);
    c.h = 0x00;
    c.l = 0x05;
    c.instr_SET_0xCBC6();
    assert_eq!(c.mmu.read8(0x0005), 0b00000001);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SUB_0x95() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.l = 42;
    c.instr_SUB_0x95();
    assert_eq!(c.a, 10);
    assert_eq!(c.l, 42);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.l = 10;
    c.instr_SUB_0x95();
    assert_eq!(c.a, 1);
    assert_eq!(c.l, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.l = 10;
    c.instr_SUB_0x95();
    assert_eq!(c.a, 0);
    assert_eq!(c.l, 10);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.l = 11;
    c.instr_SUB_0x95();
    assert_eq!(c.a, 255);
    assert_eq!(c.l, 11);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0x94() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.h = 42;
    c.instr_SUB_0x94();
    assert_eq!(c.a, 10);
    assert_eq!(c.h, 42);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.h = 10;
    c.instr_SUB_0x94();
    assert_eq!(c.a, 1);
    assert_eq!(c.h, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.h = 10;
    c.instr_SUB_0x94();
    assert_eq!(c.a, 0);
    assert_eq!(c.h, 10);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.h = 11;
    c.instr_SUB_0x94();
    assert_eq!(c.a, 255);
    assert_eq!(c.h, 11);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0x90() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.b = 42;
    c.instr_SUB_0x90();
    assert_eq!(c.a, 10);
    assert_eq!(c.b, 42);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.b = 10;
    c.instr_SUB_0x90();
    assert_eq!(c.a, 1);
    assert_eq!(c.b, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.b = 10;
    c.instr_SUB_0x90();
    assert_eq!(c.a, 0);
    assert_eq!(c.b, 10);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.b = 11;
    c.instr_SUB_0x90();
    assert_eq!(c.a, 255);
    assert_eq!(c.b, 11);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0x91() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.c = 42;
    c.instr_SUB_0x91();
    assert_eq!(c.a, 10);
    assert_eq!(c.c, 42);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.c = 10;
    c.instr_SUB_0x91();
    assert_eq!(c.a, 1);
    assert_eq!(c.c, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.c = 10;
    c.instr_SUB_0x91();
    assert_eq!(c.a, 0);
    assert_eq!(c.c, 10);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.c = 11;
    c.instr_SUB_0x91();
    assert_eq!(c.a, 255);
    assert_eq!(c.c, 11);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0x92() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.d = 42;
    c.instr_SUB_0x92();
    assert_eq!(c.a, 10);
    assert_eq!(c.d, 42);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.d = 10;
    c.instr_SUB_0x92();
    assert_eq!(c.a, 1);
    assert_eq!(c.d, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.d = 10;
    c.instr_SUB_0x92();
    assert_eq!(c.a, 0);
    assert_eq!(c.d, 10);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.d = 11;
    c.instr_SUB_0x92();
    assert_eq!(c.a, 255);
    assert_eq!(c.d, 11);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0x97() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 10;
    c.l = 10;
    c.instr_SUB_0x97();
    assert_eq!(c.a, 0);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_SUB_0xD6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.mmu.write8(1, 42);
    c.instr_SUB_0xD6();
    assert_eq!(c.a, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 11;
    c.mmu.write8(1, 10);
    c.instr_SUB_0xD6();
    assert_eq!(c.a, 1);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 10;
    c.mmu.write8(1, 10);
    c.instr_SUB_0xD6();
    assert_eq!(c.a, 0);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 10;
    c.mmu.write8(1, 11);
    c.instr_SUB_0xD6();
    assert_eq!(c.a, 255);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 2);
}

#[test]
fn instr_SUB_0x96() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 52;
    c.mmu.write8(0x1337, 42);
    c.h = 0x13;
    c.l = 0x37;
    c.instr_SUB_0x96();
    assert_eq!(c.a, 10);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 11;
    c.mmu.write8(0x1337, 10);
    c.h = 0x13;
    c.l = 0x37;
    c.instr_SUB_0x96();
    assert_eq!(c.a, 1);
    assert_eq!(c.f, Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.mmu.write8(0x1337, 10);
    c.h = 0x13;
    c.l = 0x37;
    c.instr_SUB_0x96();
    assert_eq!(c.a, 0);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 10;
    c.mmu.write8(0x1337, 11);
    c.h = 0x13;
    c.l = 0x37;
    c.instr_SUB_0x96();
    assert_eq!(c.a, 255);
    assert_eq!(c.f, Flag::Carry as u8 | Flag::Operation as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn sum_ints() {
    let mut c = Cpu::new();
    // Load program in memory
    c.mmu.write8(0, 0x06);          // 00 LD    B,  5
    c.mmu.write8(1,   10);          // 01 ^5
    c.mmu.write8(2, 0xAF);          // 02 XOR   A,  A
    c.mmu.write8(3, 0x80);          // 03 ADD   A,  B
    c.mmu.write8(4, 0x05);          // 04 DEC   B
    c.mmu.write8(5, 0x20);          // 05 JRNZ -2
    c.mmu.write8(6,   -2 as u8);    // 06 ^-2
    c.mmu.write8(7, 0x10);          // 07 STOP
    // Run it
    c.run(true);
}
