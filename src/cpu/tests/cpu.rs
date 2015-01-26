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
