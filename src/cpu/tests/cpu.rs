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
    c.run();
}

#[test]
fn instr_ADC_0x88() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x88();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x89() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x89();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0x8F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0x8F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADC_0xCE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADC_0xCE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x09() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x09();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x19() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x19();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x29() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x29();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x39() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x39();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x80() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x80();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x81() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x81();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x82() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x82();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x83() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x83();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x84() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x84();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x85() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x85();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x86() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x86();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0x87() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0x87();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0xC6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0xC6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_ADD_0xE8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_ADD_0xE8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB40() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB40();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB41() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB41();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB42() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB42();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB43() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB43();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB44() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB44();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB45() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB45();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB46() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB46();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB47() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB47();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB48() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB48();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB49() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB49();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB4F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB4F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB50() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB50();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB51() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB51();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB52() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB52();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB53() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB53();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB54() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB54();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB55() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB55();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB56() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB56();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB57() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB57();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB58() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB58();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB59() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB59();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB5F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB5F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB60() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB60();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB61() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB61();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB62() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB62();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB63() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB63();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB64() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB64();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB65() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB65();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB66() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB66();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB67() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB67();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB68() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB68();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB69() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB69();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB6F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB6F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB70() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB70();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB71() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB71();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB72() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB72();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB73() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB73();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB74() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB74();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB75() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB75();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB76() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB76();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB77() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB77();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB78() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB78();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB79() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB79();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_BIT_0xCB7F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_BIT_0xCB7F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CALL_0xC4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CALL_0xC4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CALL_0xCC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CALL_0xCC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CALL_0xCD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CALL_0xCD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CALL_0xD4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CALL_0xD4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CALL_0xDC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CALL_0xDC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CCF_0x3F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CCF_0x3F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xB8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xB8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xB9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xB9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xBF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xBF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CP_0xFE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CP_0xFE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_CPL_0x2F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_CPL_0x2F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DAA_0x27() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DAA_0x27();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x05() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x05();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x0B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x0B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x0D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x0D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x15() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x15();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x1B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x1B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x1D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x1D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x25() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x25();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x2B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x2B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x2D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x2D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x35() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x35();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x3B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x3B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DEC_0x3D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DEC_0x3D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_DI_0xF3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_DI_0xF3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_EI_0xFB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_EI_0xFB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_HALT_0x76() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_HALT_0x76();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x03() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x03();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x04() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x04();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x0C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x0C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x13() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x13();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x14() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x14();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x1C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x1C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x23() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x23();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x24() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x24();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x2C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x2C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x33() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x33();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x34() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x34();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_INC_0x3C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_INC_0x3C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xC2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xC2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xC3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xC3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xCA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xCA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xD2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xD2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xDA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xDA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_JP_0xE9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_JP_0xE9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x01() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x01();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x02() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x02();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x06() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x06();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x08() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x08();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x0A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x0A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x0E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x0E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x11() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x11();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x12() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x12();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x16() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x16();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x1A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x1A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x1E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x1E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x21() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x21();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x22() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x22();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x26() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x26();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x2A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x2A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x2E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x2E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x31() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x31();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x32() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x32();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x36() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x36();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x3A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x3A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x3E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x3E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x40() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x40();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x41() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x41();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x42() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x42();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x43() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x43();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x44() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x44();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x45() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x45();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x46() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x46();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x47() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x47();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x48() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x48();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x49() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x49();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x4F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x4F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x50() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x50();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x51() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x51();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x52() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x52();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x53() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x53();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x54() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x54();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x55() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x55();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x56() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x56();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x57() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x57();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x58() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x58();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x59() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x59();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x5F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x5F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x60() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x60();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x61() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x61();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x62() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x62();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x63() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x63();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x64() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x64();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x65() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x65();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x66() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x66();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x67() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x67();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x68() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x68();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x69() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x69();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x6F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x6F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x70() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x70();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x71() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x71();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x72() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x72();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x73() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x73();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x74() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x74();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x75() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x75();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x77() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x77();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x78() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x78();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x79() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x79();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0x7F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0x7F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xE2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xE2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xEA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xEA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xF2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xF2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xF8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xF8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xF9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xF9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LD_0xFA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LD_0xFA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LDH_0xE0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LDH_0xE0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_LDH_0xF0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_LDH_0xF0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_PREFIX_0xCB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_PREFIX_0xCB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB80() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB80();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB81() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB81();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB82() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB82();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB83() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB83();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB84() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB84();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB85() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB85();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB86() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB86();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB87() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB87();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB88() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB88();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB89() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB89();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB8F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB8F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB90() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB90();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB91() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB91();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB92() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB92();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB93() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB93();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB94() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB94();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB95() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB95();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB96() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB96();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB97() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB97();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB98() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB98();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB99() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB99();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCB9F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCB9F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA1() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA1();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA5() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA5();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA7() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA7();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBA9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBA9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBAF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBAF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB1() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB1();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB5() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB5();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB7() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB7();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBB9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBB9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RES_0xCBBF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RES_0xCBBF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RET_0xC0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RET_0xC0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RET_0xC8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RET_0xC8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RET_0xC9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RET_0xC9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RET_0xD0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RET_0xD0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RET_0xD8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RET_0xD8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RETI_0xD9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RETI_0xD9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB10() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB10();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB11() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB11();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB12() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB12();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB13() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB13();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB14() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB14();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB15() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB15();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB16() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB16();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RL_0xCB17() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RL_0xCB17();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLA_0x17() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLA_0x17();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB00() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB00();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB01() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB01();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB02() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB02();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB03() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB03();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB04() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB04();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB05() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB05();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB06() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB06();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLC_0xCB07() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLC_0xCB07();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RLCA_0x07() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RLCA_0x07();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB18() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB18();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB19() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB19();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RR_0xCB1F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RR_0xCB1F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRA_0x1F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRA_0x1F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB08() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB08();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB09() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB09();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRC_0xCB0F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRC_0xCB0F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_RRCA_0x0F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_RRCA_0x0F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x98() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x98();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x99() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x99();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0x9F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0x9F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SBC_0xDE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SBC_0xDE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SCF_0x37() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SCF_0x37();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBC2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBC2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBC8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBC8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBC9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBC9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBCF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBCF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD1() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD1();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD5() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD5();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD7() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD7();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBD9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBD9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBDF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBDF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE1() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE1();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE5() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE5();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE7() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE7();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBE9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBE9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBEA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBEA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBEB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBEB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBEC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBEC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBED() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBED();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBEE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBEE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBEF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBEF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF0() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF0();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF1() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF1();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF2() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF2();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF3() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF3();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF4() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF4();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF5() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF5();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF6() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF6();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF7() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF7();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF8() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF8();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBF9() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBF9();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFA() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFA();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFB() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFB();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFC() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFC();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFD() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFD();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFE() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFE();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SET_0xCBFF() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SET_0xCBFF();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB20() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB20();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB21() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB21();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB22() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB22();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB23() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB23();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB24() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB24();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB25() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB25();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB26() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB26();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SLA_0xCB27() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SLA_0xCB27();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB28() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB28();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB29() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB29();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRA_0xCB2F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRA_0xCB2F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB38() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB38();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB39() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB39();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3A() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3A();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3B() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3B();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3C() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3C();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3D() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3D();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3E() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3E();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SRL_0xCB3F() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SRL_0xCB3F();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_STOP_0x10() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_STOP_0x10();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SUB_0x93() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SUB_0x93();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB30() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB30();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB31() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB31();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB32() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB32();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB33() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB33();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB34() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB34();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB35() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB35();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB36() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB36();
    assert_eq!(true, true); // Unimplemented
}

#[test]
fn instr_SWAP_0xCB37() {
    let mut c = Cpu::new();

    c.reset();
    c.instr_SWAP_0xCB37();
    assert_eq!(true, true); // Unimplemented
}

