use super::super::*;

#[test]
fn cpu_new() {
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
fn cpu_instr_0x83() {
    let mut c = Cpu::new();

    c.a = 42;
    c.e = 42;
    c.instr_0x83();
    assert_eq!(c.a, 84);
    assert_eq!(c.e, 42);
    assert_eq!(c.f, Flag::None as u8);

    c = Cpu::new();

    c.a = 250;
    c.e = 5;
    c.instr_0x83();
    assert_eq!(c.a, 255);
    assert_eq!(c.e, 5);
    assert_eq!(c.f, Flag::None as u8);

    c.a = 250;
    c.e = 6;
    c.instr_0x83();
    assert_eq!(c.a, 0);
    assert_eq!(c.e, 6);
    assert_eq!(c.f, Flag::Zero as u8 | Flag::Carry as u8);

    c.a = 250;
    c.e = 7;
    c.instr_0x83();
    assert_eq!(c.a, 1);
    assert_eq!(c.e, 7);
    assert_eq!(c.f, Flag::Carry as u8);
}
