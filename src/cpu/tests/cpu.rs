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
fn instr_ADD_0x85() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.l = 42;
    c.instr_ADD_0x85();
    assert_eq!(c.a, 84);
    assert_eq!(c.l, 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.l = 5;
    c.instr_ADD_0x85();
    assert_eq!(c.a, 255);
    assert_eq!(c.l, 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.l = 6;
    c.instr_ADD_0x85();
    assert_eq!(c.a, 0);
    assert_eq!(c.l, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.l = 7;
    c.instr_ADD_0x85();
    assert_eq!(c.a, 1);
    assert_eq!(c.l, 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0x84() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.h = 42;
    c.instr_ADD_0x84();
    assert_eq!(c.a, 84);
    assert_eq!(c.h, 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.h = 5;
    c.instr_ADD_0x84();
    assert_eq!(c.a, 255);
    assert_eq!(c.h, 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.h = 6;
    c.instr_ADD_0x84();
    assert_eq!(c.a, 0);
    assert_eq!(c.h, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.h = 7;
    c.instr_ADD_0x84();
    assert_eq!(c.a, 1);
    assert_eq!(c.h, 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0x80() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.b = 42;
    c.instr_ADD_0x80();
    assert_eq!(c.a, 84);
    assert_eq!(c.b, 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.b = 5;
    c.instr_ADD_0x80();
    assert_eq!(c.a, 255);
    assert_eq!(c.b, 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.b = 6;
    c.instr_ADD_0x80();
    assert_eq!(c.a, 0);
    assert_eq!(c.b, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.b = 7;
    c.instr_ADD_0x80();
    assert_eq!(c.a, 1);
    assert_eq!(c.b, 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0x81() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.c = 42;
    c.instr_ADD_0x81();
    assert_eq!(c.a, 84);
    assert_eq!(c.c, 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.c = 5;
    c.instr_ADD_0x81();
    assert_eq!(c.a, 255);
    assert_eq!(c.c, 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.c = 6;
    c.instr_ADD_0x81();
    assert_eq!(c.a, 0);
    assert_eq!(c.c, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.c = 7;
    c.instr_ADD_0x81();
    assert_eq!(c.a, 1);
    assert_eq!(c.c, 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0x82() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.d = 42;
    c.instr_ADD_0x82();
    assert_eq!(c.a, 84);
    assert_eq!(c.d, 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.d = 5;
    c.instr_ADD_0x82();
    assert_eq!(c.a, 255);
    assert_eq!(c.d, 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.d = 6;
    c.instr_ADD_0x82();
    assert_eq!(c.a, 0);
    assert_eq!(c.d, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 250;
    c.d = 7;
    c.instr_ADD_0x82();
    assert_eq!(c.a, 1);
    assert_eq!(c.d, 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0x87() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.instr_ADD_0x87();
    assert_eq!(c.a, 84);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 127;
    c.instr_ADD_0x87();
    assert_eq!(c.a, 254);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 128;
    c.instr_ADD_0x87();
    assert_eq!(c.a, 0);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 1);

    c.reset();
    c.a = 150;
    c.instr_ADD_0x87();
    assert_eq!(c.a, 44);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 1);
}

#[test]
fn instr_ADD_0xC6() {
    let mut c = Cpu::new();

    c.reset();
    c.a = 42;
    c.mmu.write8(1, 42);
    c.instr_ADD_0xC6();
    assert_eq!(c.a, 84);
    assert_eq!(c.mmu.read8(1), 42);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 250;
    c.mmu.write8(1, 5);
    c.instr_ADD_0xC6();
    assert_eq!(c.a, 255);
    assert_eq!(c.mmu.read8(1), 5);
    assert_eq!(c.f, Flag::None as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 250;
    c.mmu.write8(1, 6);
    c.instr_ADD_0xC6();
    assert_eq!(c.a, 0);
    assert_eq!(c.mmu.read8(1), 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);
    assert_eq!(c.pc, 2);

    c.reset();
    c.a = 250;
    c.mmu.write8(1, 7);
    c.instr_ADD_0xC6();
    assert_eq!(c.a, 1);
    assert_eq!(c.mmu.read8(1), 7);
    assert_eq!(c.f, Flag::Carry as u8);
    assert_eq!(c.pc, 2);
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
