    use std::collections::BTreeMap;

    let opcodes = BTreeMap::new();

    pub fn instr_ADD_0x85(&mut self) {
        //! Prototype: ADD A, L
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x85
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds l to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xE7(&mut self) {
        //! Prototype: RST 20H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xE7
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 20h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xE6(&mut self) {
        //! Prototype: AND d8
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xE6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with *.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x28(&mut self) {
        //! Prototype: JR Z, r8
        //! Mnemonic:  JR
        //! Size:      1 byte
        //! Binary:    0x28
        //! Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x29(&mut self) {
        //! Prototype: ADD HL, HL
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x29
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   The value of hl is added to hl.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x22(&mut self) {
        //! Prototype: LD (HL+), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x22
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Stores hl into the memory location pointed to by **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x23(&mut self) {
        //! Prototype: INC HL
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x23
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Adds one to hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x20(&mut self) {
        //! Prototype: JR NZ, r8
        //! Mnemonic:  JR
        //! Size:      1 byte
        //! Binary:    0x20
        //! Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x21(&mut self) {
        //! Prototype: LD HL, d16
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x21
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads ** into hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x26(&mut self) {
        //! Prototype: LD H, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x26
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DAA_0x27(&mut self) {
        //! Prototype: DAA 
        //! Mnemonic:  DAA
        //! Size:      1 byte
        //! Binary:    0x27
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Preserved
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   Adjusts a for BCD addition and subtraction operations.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x24(&mut self) {
        //! Prototype: INC H
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x24
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x25(&mut self) {
        //! Prototype: DEC H
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x25
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from h.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xE9(&mut self) {
        //! Prototype: JP (HL)
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xE9
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value of hl into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0xE8(&mut self) {
        //! Prototype: ADD SP, r8
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0xE8
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RR_0xCB18(&mut self) {
        //! Prototype: RR B
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB18
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DEC_0x2B(&mut self) {
        //! Prototype: DEC HL
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x2B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x2C(&mut self) {
        //! Prototype: INC L
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x2C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x2A(&mut self) {
        //! Prototype: LD A, (HL+)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x2A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value pointed to by ** into hl.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CPL_0x2F(&mut self) {
        //! Prototype: CPL 
        //! Mnemonic:  CPL
        //! Size:      1 byte
        //! Binary:    0x2F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force set (1)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are inverted (one's complement).

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x2D(&mut self) {
        //! Prototype: DEC L
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x2D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from l.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x2E(&mut self) {
        //! Prototype: LD L, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x2E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RR_0xCB19(&mut self) {
        //! Prototype: RR C
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB19
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2F(&mut self) {
        //! Prototype: SRA A
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of a are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2E(&mut self) {
        //! Prototype: SRA (HL)
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of (hl) are shifted right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and the previous
        //!   contents of bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2D(&mut self) {
        //! Prototype: SRA L
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of l are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2C(&mut self) {
        //! Prototype: SRA H
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of h are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2B(&mut self) {
        //! Prototype: SRA E
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of e are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB2A(&mut self) {
        //! Prototype: SRA D
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB2A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of d are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB27(&mut self) {
        //! Prototype: SLA A
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB27
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB26(&mut self) {
        //! Prototype: SLA (HL)
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB26
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are shifted left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and a zero is put
        //!   into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB25(&mut self) {
        //! Prototype: SLA L
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB25
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB24(&mut self) {
        //! Prototype: SLA H
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB24
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB23(&mut self) {
        //! Prototype: SLA E
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB23
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB22(&mut self) {
        //! Prototype: SLA D
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB22
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB21(&mut self) {
        //! Prototype: SLA C
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB21
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SLA_0xCB20(&mut self) {
        //! Prototype: SLA B
        //! Mnemonic:  SLA
        //! Size:      2 bytes
        //! Binary:    0xCB20
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are shifted left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and a zero is put into bit
        //!   0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB29(&mut self) {
        //! Prototype: SRA C
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB29
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of c are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRA_0xCB28(&mut self) {
        //! Prototype: SRA B
        //! Mnemonic:  SRA
        //! Size:      2 bytes
        //! Binary:    0xCB28
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of b are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   bit 7 are unchanged.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x68(&mut self) {
        //! Prototype: LD L, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x68
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x69(&mut self) {
        //! Prototype: LD L, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x69
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x66(&mut self) {
        //! Prototype: LD H, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x66
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x67(&mut self) {
        //! Prototype: LD H, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x67
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x64(&mut self) {
        //! Prototype: LD H, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x64
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x65(&mut self) {
        //! Prototype: LD H, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x65
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x62(&mut self) {
        //! Prototype: LD H, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x62
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x63(&mut self) {
        //! Prototype: LD H, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x63
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x60(&mut self) {
        //! Prototype: LD H, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x60
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x61(&mut self) {
        //! Prototype: LD H, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x61
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6F(&mut self) {
        //! Prototype: LD L, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6D(&mut self) {
        //! Prototype: LD L, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6E(&mut self) {
        //! Prototype: LD L, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6B(&mut self) {
        //! Prototype: LD L, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6C(&mut self) {
        //! Prototype: LD L, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x6A(&mut self) {
        //! Prototype: LD L, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x6A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB6C(&mut self) {
        //! Prototype: BIT 5, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6B(&mut self) {
        //! Prototype: BIT 5, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6A(&mut self) {
        //! Prototype: BIT 5, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6F(&mut self) {
        //! Prototype: BIT 5, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6E(&mut self) {
        //! Prototype: BIT 5, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB6D(&mut self) {
        //! Prototype: BIT 5, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB6D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDA(&mut self) {
        //! Prototype: SET 3, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDC(&mut self) {
        //! Prototype: SET 3, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDB(&mut self) {
        //! Prototype: SET 3, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDE(&mut self) {
        //! Prototype: SET 3, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDD(&mut self) {
        //! Prototype: SET 3, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDD
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBDF(&mut self) {
        //! Prototype: SET 3, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBDF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADD_0x84(&mut self) {
        //! Prototype: ADD A, H
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x84
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds h to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xDA(&mut self) {
        //! Prototype: JP C, a16
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xDA
        //! Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x86(&mut self) {
        //! Prototype: ADD A, (HL)
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x86
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds (hl) to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xDC(&mut self) {
        //! Prototype: CALL C, a16
        //! Mnemonic:  CALL
        //! Size:      1 byte
        //! Binary:    0xDC
        //! Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x80(&mut self) {
        //! Prototype: ADD A, B
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x80
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds b to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x81(&mut self) {
        //! Prototype: ADD A, C
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x81
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds c to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x82(&mut self) {
        //! Prototype: ADD A, D
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x82
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds d to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x83(&mut self) {
        //! Prototype: ADD A, E
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x83
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds e to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RR_0xCB1F(&mut self) {
        //! Prototype: RR A
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1D(&mut self) {
        //! Prototype: RR L
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1E(&mut self) {
        //! Prototype: RR (HL)
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are rotated right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and the previous
        //!   contents of the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1B(&mut self) {
        //! Prototype: RR E
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1C(&mut self) {
        //! Prototype: RR H
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RR_0xCB1A(&mut self) {
        //! Prototype: RR D
        //! Mnemonic:  RR
        //! Size:      2 bytes
        //! Binary:    0xCB1A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RET_0xD0(&mut self) {
        //! Prototype: RET NC
        //! Mnemonic:  RET
        //! Size:      1 byte
        //! Binary:    0xD0
        //! Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8E(&mut self) {
        //! Prototype: ADC A, (HL)
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds (hl) and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8F(&mut self) {
        //! Prototype: ADC A, A
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds a and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xD4(&mut self) {
        //! Prototype: CALL NC, a16
        //! Mnemonic:  CALL
        //! Size:      1 byte
        //! Binary:    0xD4
        //! Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8A(&mut self) {
        //! Prototype: ADC A, D
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds d and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8B(&mut self) {
        //! Prototype: ADC A, E
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds e and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8C(&mut self) {
        //! Prototype: ADC A, H
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds h and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RL_0xCB16(&mut self) {
        //! Prototype: RL (HL)
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB16
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are rotated left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and the previous
        //!   contents of the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB17(&mut self) {
        //! Prototype: RL A
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB17
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB14(&mut self) {
        //! Prototype: RL H
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB14
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB15(&mut self) {
        //! Prototype: RL L
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB15
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB12(&mut self) {
        //! Prototype: RL D
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB12
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB13(&mut self) {
        //! Prototype: RL E
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB13
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB10(&mut self) {
        //! Prototype: RL B
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB10
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RL_0xCB11(&mut self) {
        //! Prototype: RL C
        //! Mnemonic:  RL
        //! Size:      2 bytes
        //! Binary:    0xCB11
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD1(&mut self) {
        //! Prototype: SET 2, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD0(&mut self) {
        //! Prototype: SET 2, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD3(&mut self) {
        //! Prototype: SET 2, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD2(&mut self) {
        //! Prototype: SET 2, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD5(&mut self) {
        //! Prototype: SET 2, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD4(&mut self) {
        //! Prototype: SET 2, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD7(&mut self) {
        //! Prototype: SET 2, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD6(&mut self) {
        //! Prototype: SET 2, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 2 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD9(&mut self) {
        //! Prototype: SET 3, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBD8(&mut self) {
        //! Prototype: SET 3, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBD8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 3 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB69(&mut self) {
        //! Prototype: BIT 5, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB69
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB68(&mut self) {
        //! Prototype: BIT 5, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB68
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 5 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB63(&mut self) {
        //! Prototype: BIT 4, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB63
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB62(&mut self) {
        //! Prototype: BIT 4, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB62
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB61(&mut self) {
        //! Prototype: BIT 4, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB61
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB60(&mut self) {
        //! Prototype: BIT 4, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB60
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB67(&mut self) {
        //! Prototype: BIT 4, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB67
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB66(&mut self) {
        //! Prototype: BIT 4, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB66
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB65(&mut self) {
        //! Prototype: BIT 4, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB65
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB64(&mut self) {
        //! Prototype: BIT 4, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB64
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 4 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB85(&mut self) {
        //! Prototype: RES 0, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB85
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB84(&mut self) {
        //! Prototype: RES 0, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB84
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB87(&mut self) {
        //! Prototype: RES 0, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB87
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB86(&mut self) {
        //! Prototype: RES 0, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB86
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB81(&mut self) {
        //! Prototype: RES 0, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB81
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB80(&mut self) {
        //! Prototype: RES 0, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB80
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB83(&mut self) {
        //! Prototype: RES 0, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB83
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB82(&mut self) {
        //! Prototype: RES 0, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB82
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 0 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB89(&mut self) {
        //! Prototype: RES 1, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB89
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB88(&mut self) {
        //! Prototype: RES 1, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB88
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x3E(&mut self) {
        //! Prototype: LD A, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x3E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x3D(&mut self) {
        //! Prototype: DEC A
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x3D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CCF_0x3F(&mut self) {
        //! Prototype: CCF 
        //! Mnemonic:  CCF
        //! Size:      1 byte
        //! Binary:    0x3F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   Inverts the carry flag.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x3A(&mut self) {
        //! Prototype: LD A, (HL-)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x3A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value pointed to by ** into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x3C(&mut self) {
        //! Prototype: INC A
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x3C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x3B(&mut self) {
        //! Prototype: DEC SP
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x3B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x87(&mut self) {
        //! Prototype: ADD A, A
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x87
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds a to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0xDE(&mut self) {
        //! Prototype: SBC A, d8
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0xDE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts * and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xDF(&mut self) {
        //! Prototype: RST 18H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xDF
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 18h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x35(&mut self) {
        //! Prototype: DEC (HL)
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x35
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from (hl).

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x34(&mut self) {
        //! Prototype: INC (HL)
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x34
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SCF_0x37(&mut self) {
        //! Prototype: SCF 
        //! Mnemonic:  SCF
        //! Size:      1 byte
        //! Binary:    0x37
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force set (1)
        //! Description:
        //!   Sets the carry flag.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f |= Flag::Carry as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x36(&mut self) {
        //! Prototype: LD (HL), d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x36
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x31(&mut self) {
        //! Prototype: LD SP, d16
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x31
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads ** into sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x30(&mut self) {
        //! Prototype: JR NC, r8
        //! Mnemonic:  JR
        //! Size:      1 byte
        //! Binary:    0x30
        //! Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x33(&mut self) {
        //! Prototype: INC SP
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x33
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Adds one to sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x32(&mut self) {
        //! Prototype: LD (HL-), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x32
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Stores a into the memory location pointed to by **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x39(&mut self) {
        //! Prototype: ADD HL, SP
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x39
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   The value of hl is added to hl.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x38(&mut self) {
        //! Prototype: JR C, r8
        //! Mnemonic:  JR
        //! Size:      1 byte
        //! Binary:    0x38
        //! Cycles:    8 cycles (not taken) or 12 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the signed value * is added to pc. The
        //!   jump is measured from the start of the instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCB8E(&mut self) {
        //! Prototype: RES 1, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8D(&mut self) {
        //! Prototype: RES 1, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8F(&mut self) {
        //! Prototype: RES 1, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8A(&mut self) {
        //! Prototype: RES 1, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8C(&mut self) {
        //! Prototype: RES 1, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB8B(&mut self) {
        //! Prototype: RES 1, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB8B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 1 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADC_0x88(&mut self) {
        //! Prototype: ADC A, B
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x88
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds b and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x89(&mut self) {
        //! Prototype: ADC A, C
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x89
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds c and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADC_0x8D(&mut self) {
        //! Prototype: ADC A, L
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0x8D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds l and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_POP_0xD1(&mut self) {
        //! Prototype: POP DE
        //! Mnemonic:  POP
        //! Size:      1 byte
        //! Binary:    0xD1
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The memory location pointed to by sp is stored into e and sp is
        //!   incremented. The memory location pointed to by sp is stored into d
        //!   and sp is incremented again.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xD2(&mut self) {
        //! Prototype: JP NC, a16
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xD2
        //! Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_PUSH_0xD5(&mut self) {
        //! Prototype: PUSH DE
        //! Mnemonic:  PUSH
        //! Size:      1 byte
        //! Binary:    0xD5
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   sp is decremented and d is stored into the memory location pointed
        //!   to by sp. sp is decremented again and e is stored into the memory
        //!   location pointed to by sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0xD6(&mut self) {
        //! Prototype: SUB d8
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0xD6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts * from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xD7(&mut self) {
        //! Prototype: RST 10H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xD7
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 10h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xD8(&mut self) {
        //! Prototype: RET C
        //! Mnemonic:  RET
        //! Size:      1 byte
        //! Binary:    0xD8
        //! Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RETI_0xD9(&mut self) {
        //! Prototype: RETI 
        //! Mnemonic:  RETI
        //! Size:      1 byte
        //! Binary:    0xD9
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Exchanges the 16-bit contents of bc, de, and hl with bc', de', and
        //!   hl'.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xA9(&mut self) {
        //! Prototype: XOR C
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xA9
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xA8(&mut self) {
        //! Prototype: XOR B
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xA8
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA7(&mut self) {
        //! Prototype: AND A
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA7
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA6(&mut self) {
        //! Prototype: AND (HL)
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA5(&mut self) {
        //! Prototype: AND L
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA5
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA4(&mut self) {
        //! Prototype: AND H
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA4
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA3(&mut self) {
        //! Prototype: AND E
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA3
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA2(&mut self) {
        //! Prototype: AND D
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA2
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA1(&mut self) {
        //! Prototype: AND C
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA1
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_AND_0xA0(&mut self) {
        //! Prototype: AND B
        //! Mnemonic:  AND
        //! Size:      1 byte
        //! Binary:    0xA0
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise AND on a with b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBC8(&mut self) {
        //! Prototype: SET 1, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC9(&mut self) {
        //! Prototype: SET 1, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x7A(&mut self) {
        //! Prototype: LD A, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7C(&mut self) {
        //! Prototype: LD A, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7B(&mut self) {
        //! Prototype: LD A, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7E(&mut self) {
        //! Prototype: LD A, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7D(&mut self) {
        //! Prototype: LD A, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x7F(&mut self) {
        //! Prototype: LD A, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x7F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB52(&mut self) {
        //! Prototype: BIT 2, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB52
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB53(&mut self) {
        //! Prototype: BIT 2, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB53
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB50(&mut self) {
        //! Prototype: BIT 2, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB50
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB51(&mut self) {
        //! Prototype: BIT 2, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB51
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB56(&mut self) {
        //! Prototype: BIT 2, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB56
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB57(&mut self) {
        //! Prototype: BIT 2, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB57
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB54(&mut self) {
        //! Prototype: BIT 2, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB54
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB55(&mut self) {
        //! Prototype: BIT 2, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB55
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 2 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC0(&mut self) {
        //! Prototype: SET 0, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC1(&mut self) {
        //! Prototype: SET 0, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB58(&mut self) {
        //! Prototype: BIT 3, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB58
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC3(&mut self) {
        //! Prototype: SET 0, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC4(&mut self) {
        //! Prototype: SET 0, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC5(&mut self) {
        //! Prototype: SET 0, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC6(&mut self) {
        //! Prototype: SET 0, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC7(&mut self) {
        //! Prototype: SET 0, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5B(&mut self) {
        //! Prototype: BIT 3, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5C(&mut self) {
        //! Prototype: BIT 3, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5A(&mut self) {
        //! Prototype: BIT 3, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5F(&mut self) {
        //! Prototype: BIT 3, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5D(&mut self) {
        //! Prototype: BIT 3, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB5E(&mut self) {
        //! Prototype: BIT 3, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB5E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCA(&mut self) {
        //! Prototype: SET 1, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCB(&mut self) {
        //! Prototype: SET 1, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCC(&mut self) {
        //! Prototype: SET 1, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCD(&mut self) {
        //! Prototype: SET 1, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCD
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCE(&mut self) {
        //! Prototype: SET 1, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBCF(&mut self) {
        //! Prototype: SET 1, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBCF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 1 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x79(&mut self) {
        //! Prototype: LD A, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x79
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x78(&mut self) {
        //! Prototype: LD A, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x78
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x71(&mut self) {
        //! Prototype: LD (HL), C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x71
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x70(&mut self) {
        //! Prototype: LD (HL), B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x70
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x73(&mut self) {
        //! Prototype: LD (HL), E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x73
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x72(&mut self) {
        //! Prototype: LD (HL), D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x72
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x75(&mut self) {
        //! Prototype: LD (HL), L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x75
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x74(&mut self) {
        //! Prototype: LD (HL), H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x74
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x77(&mut self) {
        //! Prototype: LD (HL), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x77
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_HALT_0x76(&mut self) {
        //! Prototype: HALT 
        //! Mnemonic:  HALT
        //! Size:      1 byte
        //! Binary:    0x76
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Suspends CPU operation until an interrupt or reset occurs.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB59(&mut self) {
        //! Prototype: BIT 3, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB59
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 3 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_XOR_0xAF(&mut self) {
        //! Prototype: XOR A
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAF
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAE(&mut self) {
        //! Prototype: XOR (HL)
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAD(&mut self) {
        //! Prototype: XOR L
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAD
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAC(&mut self) {
        //! Prototype: XOR H
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAC
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAB(&mut self) {
        //! Prototype: XOR E
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAB
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xAA(&mut self) {
        //! Prototype: XOR D
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xAA
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9F(&mut self) {
        //! Prototype: SBC A, A
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts a and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9E(&mut self) {
        //! Prototype: SBC A, (HL)
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts (hl) and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9D(&mut self) {
        //! Prototype: SBC A, L
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts l and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9C(&mut self) {
        //! Prototype: SBC A, H
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts h and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9B(&mut self) {
        //! Prototype: SBC A, E
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts e and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x9A(&mut self) {
        //! Prototype: SBC A, D
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x9A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts d and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x08(&mut self) {
        //! Prototype: LD (a16), SP
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x08
        //! Cycles:    20 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Exchanges the 16-bit contents of af and af'.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 5;
        self.t += 20;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0x09(&mut self) {
        //! Prototype: ADD HL, BC
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x09
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   The value of bc is added to hl.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x04(&mut self) {
        //! Prototype: INC B
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x04
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x05(&mut self) {
        //! Prototype: DEC B
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x05
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from b.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x06(&mut self) {
        //! Prototype: LD B, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x06
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RLCA_0x07(&mut self) {
        //! Prototype: RLCA 
        //! Mnemonic:  RLCA
        //! Size:      1 byte
        //! Binary:    0x07
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_NOP_0x00(&mut self) {
        //! Prototype: NOP 
        //! Mnemonic:  NOP
        //! Size:      1 byte
        //! Binary:    0x00
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   No operation is performed.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x01(&mut self) {
        //! Prototype: LD BC, d16
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x01
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads ** into bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x02(&mut self) {
        //! Prototype: LD (BC), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x02
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Stores a into the memory location pointed to by bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x03(&mut self) {
        //! Prototype: INC BC
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x03
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Adds one to bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x0D(&mut self) {
        //! Prototype: DEC C
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x0D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from c.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x0E(&mut self) {
        //! Prototype: LD C, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x0E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RRCA_0x0F(&mut self) {
        //! Prototype: RRCA 
        //! Mnemonic:  RRCA
        //! Size:      1 byte
        //! Binary:    0x0F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x0A(&mut self) {
        //! Prototype: LD A, (BC)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x0A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value pointed to by bc into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x0B(&mut self) {
        //! Prototype: DEC BC
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x0B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from bc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x0C(&mut self) {
        //! Prototype: INC C
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x0C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x97(&mut self) {
        //! Prototype: SUB A
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x97
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts a from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x96(&mut self) {
        //! Prototype: SUB (HL)
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x96
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts (hl) from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x95(&mut self) {
        //! Prototype: SUB L
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x95
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts l from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x94(&mut self) {
        //! Prototype: SUB H
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x94
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts h from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x93(&mut self) {
        //! Prototype: SUB E
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x93
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts e from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x92(&mut self) {
        //! Prototype: SUB D
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x92
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts d from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x91(&mut self) {
        //! Prototype: SUB C
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x91
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts c from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SUB_0x90(&mut self) {
        //! Prototype: SUB B
        //! Mnemonic:  SUB
        //! Size:      1 byte
        //! Binary:    0x90
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts b from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x99(&mut self) {
        //! Prototype: SBC A, C
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x99
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts c and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SBC_0x98(&mut self) {
        //! Prototype: SBC A, B
        //! Mnemonic:  SBC
        //! Size:      1 byte
        //! Binary:    0x98
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts b and the carry flag from a.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0xE2(&mut self) {
        //! Prototype: LD (C), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xE2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_POP_0xE1(&mut self) {
        //! Prototype: POP HL
        //! Mnemonic:  POP
        //! Size:      1 byte
        //! Binary:    0xE1
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The memory location pointed to by sp is stored into l and sp is
        //!   incremented. The memory location pointed to by sp is stored into h
        //!   and sp is incremented again.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LDH_0xE0(&mut self) {
        //! Prototype: LDH (a8), A
        //! Mnemonic:  LDH
        //! Size:      1 byte
        //! Binary:    0xE0
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RRC_0xCB09(&mut self) {
        //! Prototype: RRC C
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB09
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB08(&mut self) {
        //! Prototype: RRC B
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB08
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xE5(&mut self) {
        //! Prototype: PUSH HL
        //! Mnemonic:  PUSH
        //! Size:      1 byte
        //! Binary:    0xE5
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   sp is decremented and h is stored into the memory location pointed
        //!   to by sp. sp is decremented again and l is stored into the memory
        //!   location pointed to by sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RLC_0xCB05(&mut self) {
        //! Prototype: RLC L
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB05
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB04(&mut self) {
        //! Prototype: RLC H
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB04
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB07(&mut self) {
        //! Prototype: RLC A
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB07
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB06(&mut self) {
        //! Prototype: RLC (HL)
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB06
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are rotated left one bit position. The
        //!   contents of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB01(&mut self) {
        //! Prototype: RLC C
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB01
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB00(&mut self) {
        //! Prototype: RLC B
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB00
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB03(&mut self) {
        //! Prototype: RLC E
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB03
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RLC_0xCB02(&mut self) {
        //! Prototype: RLC D
        //! Mnemonic:  RLC
        //! Size:      2 bytes
        //! Binary:    0xCB02
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE5(&mut self) {
        //! Prototype: SET 4, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xEA(&mut self) {
        //! Prototype: LD (a16), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xEA
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xEF(&mut self) {
        //! Prototype: RST 28H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xEF
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 28h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_XOR_0xEE(&mut self) {
        //! Prototype: XOR d8
        //! Mnemonic:  XOR
        //! Size:      1 byte
        //! Binary:    0xEE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise XOR on a with *.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RRC_0xCB0E(&mut self) {
        //! Prototype: RRC (HL)
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are rotated right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0D(&mut self) {
        //! Prototype: RRC L
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0F(&mut self) {
        //! Prototype: RRC A
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0A(&mut self) {
        //! Prototype: RRC D
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0C(&mut self) {
        //! Prototype: RRC H
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRC_0xCB0B(&mut self) {
        //! Prototype: RRC E
        //! Mnemonic:  RRC
        //! Size:      2 bytes
        //! Binary:    0xCB0B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x40(&mut self) {
        //! Prototype: LD B, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x40
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x41(&mut self) {
        //! Prototype: LD B, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x41
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x42(&mut self) {
        //! Prototype: LD B, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x42
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x43(&mut self) {
        //! Prototype: LD B, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x43
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x44(&mut self) {
        //! Prototype: LD B, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x44
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x45(&mut self) {
        //! Prototype: LD B, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x45
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x46(&mut self) {
        //! Prototype: LD B, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x46
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x47(&mut self) {
        //! Prototype: LD B, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x47
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x48(&mut self) {
        //! Prototype: LD C, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x48
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x49(&mut self) {
        //! Prototype: LD C, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x49
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4A(&mut self) {
        //! Prototype: LD C, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4B(&mut self) {
        //! Prototype: LD C, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4C(&mut self) {
        //! Prototype: LD C, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4D(&mut self) {
        //! Prototype: LD C, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4E(&mut self) {
        //! Prototype: LD C, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x4F(&mut self) {
        //! Prototype: LD C, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x4F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_BIT_0xCB4A(&mut self) {
        //! Prototype: BIT 1, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4C(&mut self) {
        //! Prototype: BIT 1, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4B(&mut self) {
        //! Prototype: BIT 1, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4E(&mut self) {
        //! Prototype: BIT 1, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4D(&mut self) {
        //! Prototype: BIT 1, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB4F(&mut self) {
        //! Prototype: BIT 1, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB4F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBF(&mut self) {
        //! Prototype: RES 7, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBE(&mut self) {
        //! Prototype: RES 7, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBD(&mut self) {
        //! Prototype: RES 7, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBD
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBC(&mut self) {
        //! Prototype: RES 7, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBB(&mut self) {
        //! Prototype: RES 7, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBBA(&mut self) {
        //! Prototype: RES 7, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBBA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_CP_0xFE(&mut self) {
        //! Prototype: CP d8
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xFE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts * from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBB(&mut self) {
        //! Prototype: CP E
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBB
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts e from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBC(&mut self) {
        //! Prototype: CP H
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBC
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts h from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBA(&mut self) {
        //! Prototype: CP D
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBA
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts d from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBF(&mut self) {
        //! Prototype: CP A
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBF
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts a from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBD(&mut self) {
        //! Prototype: CP L
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBD
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts l from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xBE(&mut self) {
        //! Prototype: CP (HL)
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xBE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts (hl) from a and affects flags according to the result. a
        //!   is not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xB8(&mut self) {
        //! Prototype: CP B
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xB8
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts b from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CP_0xB9(&mut self) {
        //! Prototype: CP C
        //! Mnemonic:  CP
        //! Size:      1 byte
        //! Binary:    0xB9
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Subtracts c from a and affects flags according to the result. a is
        //!   not modified.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB2(&mut self) {
        //! Prototype: OR D
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB2
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB3(&mut self) {
        //! Prototype: OR E
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB3
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB0(&mut self) {
        //! Prototype: OR B
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB0
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB1(&mut self) {
        //! Prototype: OR C
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB1
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB6(&mut self) {
        //! Prototype: OR (HL)
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB7(&mut self) {
        //! Prototype: OR A
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB7
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB4(&mut self) {
        //! Prototype: OR H
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB4
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xB5(&mut self) {
        //! Prototype: OR L
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xB5
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBB7(&mut self) {
        //! Prototype: RES 6, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB6(&mut self) {
        //! Prototype: RES 6, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB5(&mut self) {
        //! Prototype: RES 6, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB4(&mut self) {
        //! Prototype: RES 6, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB3(&mut self) {
        //! Prototype: RES 6, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB2(&mut self) {
        //! Prototype: RES 6, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB1(&mut self) {
        //! Prototype: RES 6, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB0(&mut self) {
        //! Prototype: RES 6, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 6 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_EI_0xFB(&mut self) {
        //! Prototype: EI 
        //! Mnemonic:  EI
        //! Size:      1 byte
        //! Binary:    0xFB
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets both interrupt flip-flops, thus allowing maskable interrupts
        //!   to occur. An interrupt will not occur until after the immediatedly
        //!   following instruction.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBB9(&mut self) {
        //! Prototype: RES 7, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBB8(&mut self) {
        //! Prototype: RES 7, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBB8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 7 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB41(&mut self) {
        //! Prototype: BIT 0, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB41
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB40(&mut self) {
        //! Prototype: BIT 0, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB40
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB43(&mut self) {
        //! Prototype: BIT 0, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB43
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB42(&mut self) {
        //! Prototype: BIT 0, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB42
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB45(&mut self) {
        //! Prototype: BIT 0, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB45
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB44(&mut self) {
        //! Prototype: BIT 0, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB44
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB47(&mut self) {
        //! Prototype: BIT 0, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB47
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB46(&mut self) {
        //! Prototype: BIT 0, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB46
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 0 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB49(&mut self) {
        //! Prototype: BIT 1, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB49
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB48(&mut self) {
        //! Prototype: BIT 1, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB48
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 1 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RRA_0x1F(&mut self) {
        //! Prototype: RRA 
        //! Mnemonic:  RRA
        //! Size:      1 byte
        //! Binary:    0x1F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x1E(&mut self) {
        //! Prototype: LD E, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x1E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x1D(&mut self) {
        //! Prototype: DEC E
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x1D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from e.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x1C(&mut self) {
        //! Prototype: INC E
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x1C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x1B(&mut self) {
        //! Prototype: DEC DE
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x1B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x1A(&mut self) {
        //! Prototype: LD A, (DE)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x1A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value pointed to by de into a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_OR_0xF6(&mut self) {
        //! Prototype: OR d8
        //! Mnemonic:  OR
        //! Size:      1 byte
        //! Binary:    0xF6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   Bitwise OR on a with *.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xF7(&mut self) {
        //! Prototype: RST 30H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xF7
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 30h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE2(&mut self) {
        //! Prototype: SET 4, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xF5(&mut self) {
        //! Prototype: PUSH AF
        //! Mnemonic:  PUSH
        //! Size:      1 byte
        //! Binary:    0xF5
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   sp is decremented and a is stored into the memory location pointed
        //!   to by sp. sp is decremented again and f is stored into the memory
        //!   location pointed to by sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB34(&mut self) {
        //! Prototype: SWAP H
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB34
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of h are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE0(&mut self) {
        //! Prototype: SET 4, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_DI_0xF3(&mut self) {
        //! Prototype: DI 
        //! Mnemonic:  DI
        //! Size:      1 byte
        //! Binary:    0xF3
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets both interrupt flip-flops, thus prenting maskable
        //!   interrupts from triggering.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE1(&mut self) {
        //! Prototype: SET 4, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LDH_0xF0(&mut self) {
        //! Prototype: LDH A, (a8)
        //! Mnemonic:  LDH
        //! Size:      1 byte
        //! Binary:    0xF0
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE6(&mut self) {
        //! Prototype: SET 4, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_POP_0xF1(&mut self) {
        //! Prototype: POP AF
        //! Mnemonic:  POP
        //! Size:      1 byte
        //! Binary:    0xF1
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Set if appropriate
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   The memory location pointed to by sp is stored into f and sp is
        //!   incremented. The memory location pointed to by sp is stored into a
        //!   and sp is incremented again.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBE7(&mut self) {
        //! Prototype: SET 4, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADD_0x19(&mut self) {
        //! Prototype: ADD HL, DE
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0x19
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   The value of de is added to hl.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JR_0x18(&mut self) {
        //! Prototype: JR r8
        //! Mnemonic:  JR
        //! Size:      1 byte
        //! Binary:    0x18
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The signed value * is added to pc. The jump is measured from the
        //!   start of the instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RLA_0x17(&mut self) {
        //! Prototype: RLA 
        //! Mnemonic:  RLA
        //! Size:      1 byte
        //! Binary:    0x17
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are rotated left one bit position. The contents
        //!   of bit 7 are copied to the carry flag and the previous contents of
        //!   the carry flag are copied to bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x16(&mut self) {
        //! Prototype: LD D, d8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x16
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads * into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_DEC_0x15(&mut self) {
        //! Prototype: DEC D
        //! Mnemonic:  DEC
        //! Size:      1 byte
        //! Binary:    0x15
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force set (1)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Subtracts one from d.

        unimplemented!();

        // Update flags
        self.f |= Flag::Operation as u8;

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x14(&mut self) {
        //! Prototype: INC D
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x14
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Preserved
        //! Description:
        //!   Adds one to d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_INC_0x13(&mut self) {
        //! Prototype: INC DE
        //! Mnemonic:  INC
        //! Size:      1 byte
        //! Binary:    0x13
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Adds one to de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x12(&mut self) {
        //! Prototype: LD (DE), A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x12
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Stores a into the memory location pointed to by de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x11(&mut self) {
        //! Prototype: LD DE, d16
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x11
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads ** into de.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_STOP_0x10(&mut self) {
        //! Prototype: STOP 0
        //! Mnemonic:  STOP
        //! Size:      1 byte
        //! Binary:    0x10
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The b register is decremented, and if not zero, the signed value *
        //!   is added to pc. The jump is measured from the start of the
        //!   instruction opcode.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBFC(&mut self) {
        //! Prototype: SET 7, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFB(&mut self) {
        //! Prototype: SET 7, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFA(&mut self) {
        //! Prototype: SET 7, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFF(&mut self) {
        //! Prototype: SET 7, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFE(&mut self) {
        //! Prototype: SET 7, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBFD(&mut self) {
        //! Prototype: SET 7, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBFD
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RST_0xFF(&mut self) {
        //! Prototype: RST 38H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xFF
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 38h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SRL_0xCB3A(&mut self) {
        //! Prototype: SRL D
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of d are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3B(&mut self) {
        //! Prototype: SRL E
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of e are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3C(&mut self) {
        //! Prototype: SRL H
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of h are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3D(&mut self) {
        //! Prototype: SRL L
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of l are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3E(&mut self) {
        //! Prototype: SRL (HL)
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of (hl) are shifted right one bit position. The
        //!   contents of bit 0 are copied to the carry flag and a zero is put
        //!   into bit 7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB3F(&mut self) {
        //! Prototype: SRL A
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB3F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of a are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xFA(&mut self) {
        //! Prototype: LD A, (a16)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xFA
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SRL_0xCB38(&mut self) {
        //! Prototype: SRL B
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB38
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of b are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SRL_0xCB39(&mut self) {
        //! Prototype: SRL C
        //! Mnemonic:  SRL
        //! Size:      2 bytes
        //! Binary:    0xCB39
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Set if appropriate
        //! Description:
        //!   The contents of c are shifted right one bit position. The contents
        //!   of bit 0 are copied to the carry flag and a zero is put into bit
        //!   7.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xF8(&mut self) {
        //! Prototype: LD HL, SP+r8
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xF8
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Force unset (0)
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Zero as u8);
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0xF9(&mut self) {
        //! Prototype: LD SP, HL
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xF9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Loads the value of hl into sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB30(&mut self) {
        //! Prototype: SWAP B
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB30
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of b are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB31(&mut self) {
        //! Prototype: SWAP C
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB31
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of c are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB32(&mut self) {
        //! Prototype: SWAP D
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB32
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of d are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB33(&mut self) {
        //! Prototype: SWAP E
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB33
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of e are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0xF2(&mut self) {
        //! Prototype: LD A, (C)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0xF2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SWAP_0xCB35(&mut self) {
        //! Prototype: SWAP L
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB35
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of l are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB36(&mut self) {
        //! Prototype: SWAP (HL)
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB36
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of (hl) are shifted left one bit position. The
        //!   contents of bit 7 are put into the carry flag and a one is put
        //!   into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SWAP_0xCB37(&mut self) {
        //! Prototype: SWAP A
        //! Mnemonic:  SWAP
        //! Size:      2 bytes
        //! Binary:    0xCB37
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force unset (0)
        //!   - C: Force unset (0)
        //! Description:
        //!   The contents of a are shifted left one bit position. The contents
        //!   of bit 7 are put into the carry flag and a one is put into bit 0.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f &= !(Flag::HalfCarry as u8);
        self.f &= !(Flag::Carry as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF9(&mut self) {
        //! Prototype: SET 7, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF8(&mut self) {
        //! Prototype: SET 7, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 7 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF3(&mut self) {
        //! Prototype: SET 6, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF2(&mut self) {
        //! Prototype: SET 6, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF1(&mut self) {
        //! Prototype: SET 6, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF0(&mut self) {
        //! Prototype: SET 6, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF7(&mut self) {
        //! Prototype: SET 6, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF6(&mut self) {
        //! Prototype: SET 6, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF5(&mut self) {
        //! Prototype: SET 6, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBF4(&mut self) {
        //! Prototype: SET 6, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBF4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 6 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x5C(&mut self) {
        //! Prototype: LD E, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5C
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5B(&mut self) {
        //! Prototype: LD E, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5B
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5A(&mut self) {
        //! Prototype: LD E, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5A
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5F(&mut self) {
        //! Prototype: LD E, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5F
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5E(&mut self) {
        //! Prototype: LD E, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5E
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x5D(&mut self) {
        //! Prototype: LD E, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x5D
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCBA8(&mut self) {
        //! Prototype: RES 5, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA9(&mut self) {
        //! Prototype: RES 5, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA6(&mut self) {
        //! Prototype: RES 4, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA6
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA7(&mut self) {
        //! Prototype: RES 4, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA7
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA4(&mut self) {
        //! Prototype: RES 4, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA5(&mut self) {
        //! Prototype: RES 4, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA5
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA2(&mut self) {
        //! Prototype: RES 4, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA3(&mut self) {
        //! Prototype: RES 4, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA0(&mut self) {
        //! Prototype: RES 4, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA0
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBA1(&mut self) {
        //! Prototype: RES 4, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBA1
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 4 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAF(&mut self) {
        //! Prototype: RES 5, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAD(&mut self) {
        //! Prototype: RES 5, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAD
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAE(&mut self) {
        //! Prototype: RES 5, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAB(&mut self) {
        //! Prototype: RES 5, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAC(&mut self) {
        //! Prototype: RES 5, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCBAA(&mut self) {
        //! Prototype: RES 5, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCBAA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 5 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_LD_0x53(&mut self) {
        //! Prototype: LD D, E
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x53
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of e are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x52(&mut self) {
        //! Prototype: LD D, D
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x52
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of d are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x51(&mut self) {
        //! Prototype: LD D, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x51
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x50(&mut self) {
        //! Prototype: LD D, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x50
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x57(&mut self) {
        //! Prototype: LD D, A
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x57
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of a are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x56(&mut self) {
        //! Prototype: LD D, (HL)
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x56
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of (hl) are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x55(&mut self) {
        //! Prototype: LD D, L
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x55
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of l are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x54(&mut self) {
        //! Prototype: LD D, H
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x54
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of h are loaded into d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x59(&mut self) {
        //! Prototype: LD E, C
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x59
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of c are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_LD_0x58(&mut self) {
        //! Prototype: LD E, B
        //! Mnemonic:  LD
        //! Size:      1 byte
        //! Binary:    0x58
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The contents of b are loaded into e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_SET_0xCBEB(&mut self) {
        //! Prototype: SET 5, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBEB
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEC(&mut self) {
        //! Prototype: SET 5, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBEC
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEA(&mut self) {
        //! Prototype: SET 5, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBEA
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEF(&mut self) {
        //! Prototype: SET 5, A
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBEF
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBED(&mut self) {
        //! Prototype: SET 5, L
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBED
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBEE(&mut self) {
        //! Prototype: SET 5, (HL)
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBEE
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_PUSH_0xC5(&mut self) {
        //! Prototype: PUSH BC
        //! Mnemonic:  PUSH
        //! Size:      1 byte
        //! Binary:    0xC5
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   sp is decremented and b is stored into the memory location pointed
        //!   to by sp. sp is decremented again and c is stored into the memory
        //!   location pointed to by sp.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xC4(&mut self) {
        //! Prototype: CALL NZ, a16
        //! Mnemonic:  CALL
        //! Size:      1 byte
        //! Binary:    0xC4
        //! Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xC7(&mut self) {
        //! Prototype: RST 00H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xC7
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 00h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_ADD_0xC6(&mut self) {
        //! Prototype: ADD A, d8
        //! Mnemonic:  ADD
        //! Size:      1 byte
        //! Binary:    0xC6
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds * to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_POP_0xC1(&mut self) {
        //! Prototype: POP BC
        //! Mnemonic:  POP
        //! Size:      1 byte
        //! Binary:    0xC1
        //! Cycles:    12 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The memory location pointed to by sp is stored into c and sp is
        //!   incremented. The memory location pointed to by sp is stored into b
        //!   and sp is incremented again.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xC0(&mut self) {
        //! Prototype: RET NZ
        //! Mnemonic:  RET
        //! Size:      1 byte
        //! Binary:    0xC0
        //! Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xC3(&mut self) {
        //! Prototype: JP a16
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xC3
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xC2(&mut self) {
        //! Prototype: JP NZ, a16
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xC2
        //! Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xC9(&mut self) {
        //! Prototype: RET 
        //! Mnemonic:  RET
        //! Size:      1 byte
        //! Binary:    0xC9
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RET_0xC8(&mut self) {
        //! Prototype: RET Z
        //! Mnemonic:  RET
        //! Size:      1 byte
        //! Binary:    0xC8
        //! Cycles:    8 cycles (not taken) or 20 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the top stack entry is popped into pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RES_0xCB9F(&mut self) {
        //! Prototype: RES 3, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE3(&mut self) {
        //! Prototype: SET 4, E
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE3
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9D(&mut self) {
        //! Prototype: RES 3, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9E(&mut self) {
        //! Prototype: RES 3, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9B(&mut self) {
        //! Prototype: RES 3, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9C(&mut self) {
        //! Prototype: RES 3, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE4(&mut self) {
        //! Prototype: SET 4, H
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE4
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 4 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB9A(&mut self) {
        //! Prototype: RES 3, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB9A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE8(&mut self) {
        //! Prototype: SET 5, B
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE8
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBE9(&mut self) {
        //! Prototype: SET 5, C
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBE9
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 5 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_SET_0xCBC2(&mut self) {
        //! Prototype: SET 0, D
        //! Mnemonic:  SET
        //! Size:      2 bytes
        //! Binary:    0xCBC2
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Sets bit 0 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB78(&mut self) {
        //! Prototype: BIT 7, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB78
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB79(&mut self) {
        //! Prototype: BIT 7, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB79
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB74(&mut self) {
        //! Prototype: BIT 6, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB74
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB75(&mut self) {
        //! Prototype: BIT 6, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB75
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB76(&mut self) {
        //! Prototype: BIT 6, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB76
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB77(&mut self) {
        //! Prototype: BIT 6, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB77
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB70(&mut self) {
        //! Prototype: BIT 6, B
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB70
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of b.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB71(&mut self) {
        //! Prototype: BIT 6, C
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB71
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of c.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB72(&mut self) {
        //! Prototype: BIT 6, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB72
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB73(&mut self) {
        //! Prototype: BIT 6, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB73
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 6 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7D(&mut self) {
        //! Prototype: BIT 7, L
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7D
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of l.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7E(&mut self) {
        //! Prototype: BIT 7, (HL)
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7E
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of (hl).

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7F(&mut self) {
        //! Prototype: BIT 7, A
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7F
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7A(&mut self) {
        //! Prototype: BIT 7, D
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7A
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of d.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7B(&mut self) {
        //! Prototype: BIT 7, E
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7B
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of e.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_BIT_0xCB7C(&mut self) {
        //! Prototype: BIT 7, H
        //! Mnemonic:  BIT
        //! Size:      2 bytes
        //! Binary:    0xCB7C
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Force set (1)
        //!   - C: Preserved
        //! Description:
        //!   Tests bit 7 of h.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);
        self.f |= Flag::HalfCarry as u8;

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB96(&mut self) {
        //! Prototype: RES 2, (HL)
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB96
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of (hl).

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB97(&mut self) {
        //! Prototype: RES 2, A
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB97
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of a.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB94(&mut self) {
        //! Prototype: RES 2, H
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB94
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB95(&mut self) {
        //! Prototype: RES 2, L
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB95
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of l.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB92(&mut self) {
        //! Prototype: RES 2, D
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB92
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of d.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB93(&mut self) {
        //! Prototype: RES 2, E
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB93
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of e.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB90(&mut self) {
        //! Prototype: RES 2, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB90
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB91(&mut self) {
        //! Prototype: RES 2, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB91
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 2 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB98(&mut self) {
        //! Prototype: RES 3, B
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB98
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of b.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_RES_0xCB99(&mut self) {
        //! Prototype: RES 3, C
        //! Mnemonic:  RES
        //! Size:      2 bytes
        //! Binary:    0xCB99
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   Resets bit 3 of c.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 2;
    }

    pub fn instr_ADC_0xCE(&mut self) {
        //! Prototype: ADC A, d8
        //! Mnemonic:  ADC
        //! Size:      1 byte
        //! Binary:    0xCE
        //! Cycles:    8 cycles
        //! Flags:
        //!   - Z: Set if appropriate
        //!   - N: Force unset (0)
        //!   - H: Set if appropriate
        //!   - C: Set if appropriate
        //! Description:
        //!   Adds * and the carry flag to a.

        unimplemented!();

        // Update flags
        self.f &= !(Flag::Operation as u8);

        // Update clocks
        self.m += 2;
        self.t += 8;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xCD(&mut self) {
        //! Prototype: CALL a16
        //! Mnemonic:  CALL
        //! Size:      1 byte
        //! Binary:    0xCD
        //! Cycles:    24 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus three is pushed onto the stack, then is
        //!   loaded with **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 6;
        self.t += 24;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_RST_0xCF(&mut self) {
        //! Prototype: RST 08H
        //! Mnemonic:  RST
        //! Size:      1 byte
        //! Binary:    0xCF
        //! Cycles:    16 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   The current pc value plus one is pushed onto the stack, then is
        //!   loaded with 08h.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 4;
        self.t += 16;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_JP_0xCA(&mut self) {
        //! Prototype: JP Z, a16
        //! Mnemonic:  JP
        //! Size:      1 byte
        //! Binary:    0xCA
        //! Cycles:    12 cycles (not taken) or 16 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, ** is copied to pc.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_CALL_0xCC(&mut self) {
        //! Prototype: CALL Z, a16
        //! Mnemonic:  CALL
        //! Size:      1 byte
        //! Binary:    0xCC
        //! Cycles:    12 cycles (not taken) or 24 cycles (taken)
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   If condition cc is true, the current pc value plus three is pushed
        //!   onto the stack, then is loaded with **.

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 3;
        self.t += 12;

        // Update program counter
        self.pc += 1;
    }

    pub fn instr_PREFIX_0xCB(&mut self) {
        //! Prototype: PREFIX CB
        //! Mnemonic:  PREFIX
        //! Size:      1 byte
        //! Binary:    0xCB
        //! Cycles:    4 cycles
        //! Flags:
        //!   - Z: Preserved
        //!   - N: Preserved
        //!   - H: Preserved
        //!   - C: Preserved
        //! Description:
        //!   BITS instruction prefix

        unimplemented!();

        // Update flags

        // Update clocks
        self.m += 1;
        self.t += 4;

        // Update program counter
        self.pc += 1;
    }

    opcodes.insert(0x85, instr_ADD_0x85);
    opcodes.insert(0xE7, instr_RST_0xE7);
    opcodes.insert(0xE6, instr_AND_0xE6);
    opcodes.insert(0x28, instr_JR_0x28);
    opcodes.insert(0x29, instr_ADD_0x29);
    opcodes.insert(0x22, instr_LD_0x22);
    opcodes.insert(0x23, instr_INC_0x23);
    opcodes.insert(0x20, instr_JR_0x20);
    opcodes.insert(0x21, instr_LD_0x21);
    opcodes.insert(0x26, instr_LD_0x26);
    opcodes.insert(0x27, instr_DAA_0x27);
    opcodes.insert(0x24, instr_INC_0x24);
    opcodes.insert(0x25, instr_DEC_0x25);
    opcodes.insert(0xE9, instr_JP_0xE9);
    opcodes.insert(0xE8, instr_ADD_0xE8);
    opcodes.insert(0xCB18, instr_RR_0xCB18);
    opcodes.insert(0x2B, instr_DEC_0x2B);
    opcodes.insert(0x2C, instr_INC_0x2C);
    opcodes.insert(0x2A, instr_LD_0x2A);
    opcodes.insert(0x2F, instr_CPL_0x2F);
    opcodes.insert(0x2D, instr_DEC_0x2D);
    opcodes.insert(0x2E, instr_LD_0x2E);
    opcodes.insert(0xCB19, instr_RR_0xCB19);
    opcodes.insert(0xCB2F, instr_SRA_0xCB2F);
    opcodes.insert(0xCB2E, instr_SRA_0xCB2E);
    opcodes.insert(0xCB2D, instr_SRA_0xCB2D);
    opcodes.insert(0xCB2C, instr_SRA_0xCB2C);
    opcodes.insert(0xCB2B, instr_SRA_0xCB2B);
    opcodes.insert(0xCB2A, instr_SRA_0xCB2A);
    opcodes.insert(0xCB27, instr_SLA_0xCB27);
    opcodes.insert(0xCB26, instr_SLA_0xCB26);
    opcodes.insert(0xCB25, instr_SLA_0xCB25);
    opcodes.insert(0xCB24, instr_SLA_0xCB24);
    opcodes.insert(0xCB23, instr_SLA_0xCB23);
    opcodes.insert(0xCB22, instr_SLA_0xCB22);
    opcodes.insert(0xCB21, instr_SLA_0xCB21);
    opcodes.insert(0xCB20, instr_SLA_0xCB20);
    opcodes.insert(0xCB29, instr_SRA_0xCB29);
    opcodes.insert(0xCB28, instr_SRA_0xCB28);
    opcodes.insert(0x68, instr_LD_0x68);
    opcodes.insert(0x69, instr_LD_0x69);
    opcodes.insert(0x66, instr_LD_0x66);
    opcodes.insert(0x67, instr_LD_0x67);
    opcodes.insert(0x64, instr_LD_0x64);
    opcodes.insert(0x65, instr_LD_0x65);
    opcodes.insert(0x62, instr_LD_0x62);
    opcodes.insert(0x63, instr_LD_0x63);
    opcodes.insert(0x60, instr_LD_0x60);
    opcodes.insert(0x61, instr_LD_0x61);
    opcodes.insert(0x6F, instr_LD_0x6F);
    opcodes.insert(0x6D, instr_LD_0x6D);
    opcodes.insert(0x6E, instr_LD_0x6E);
    opcodes.insert(0x6B, instr_LD_0x6B);
    opcodes.insert(0x6C, instr_LD_0x6C);
    opcodes.insert(0x6A, instr_LD_0x6A);
    opcodes.insert(0xCB6C, instr_BIT_0xCB6C);
    opcodes.insert(0xCB6B, instr_BIT_0xCB6B);
    opcodes.insert(0xCB6A, instr_BIT_0xCB6A);
    opcodes.insert(0xCB6F, instr_BIT_0xCB6F);
    opcodes.insert(0xCB6E, instr_BIT_0xCB6E);
    opcodes.insert(0xCB6D, instr_BIT_0xCB6D);
    opcodes.insert(0xCBDA, instr_SET_0xCBDA);
    opcodes.insert(0xCBDC, instr_SET_0xCBDC);
    opcodes.insert(0xCBDB, instr_SET_0xCBDB);
    opcodes.insert(0xCBDE, instr_SET_0xCBDE);
    opcodes.insert(0xCBDD, instr_SET_0xCBDD);
    opcodes.insert(0xCBDF, instr_SET_0xCBDF);
    opcodes.insert(0x84, instr_ADD_0x84);
    opcodes.insert(0xDA, instr_JP_0xDA);
    opcodes.insert(0x86, instr_ADD_0x86);
    opcodes.insert(0xDC, instr_CALL_0xDC);
    opcodes.insert(0x80, instr_ADD_0x80);
    opcodes.insert(0x81, instr_ADD_0x81);
    opcodes.insert(0x82, instr_ADD_0x82);
    opcodes.insert(0x83, instr_ADD_0x83);
    opcodes.insert(0xCB1F, instr_RR_0xCB1F);
    opcodes.insert(0xCB1D, instr_RR_0xCB1D);
    opcodes.insert(0xCB1E, instr_RR_0xCB1E);
    opcodes.insert(0xCB1B, instr_RR_0xCB1B);
    opcodes.insert(0xCB1C, instr_RR_0xCB1C);
    opcodes.insert(0xCB1A, instr_RR_0xCB1A);
    opcodes.insert(0xD0, instr_RET_0xD0);
    opcodes.insert(0x8E, instr_ADC_0x8E);
    opcodes.insert(0x8F, instr_ADC_0x8F);
    opcodes.insert(0xD4, instr_CALL_0xD4);
    opcodes.insert(0x8A, instr_ADC_0x8A);
    opcodes.insert(0x8B, instr_ADC_0x8B);
    opcodes.insert(0x8C, instr_ADC_0x8C);
    opcodes.insert(0xCB16, instr_RL_0xCB16);
    opcodes.insert(0xCB17, instr_RL_0xCB17);
    opcodes.insert(0xCB14, instr_RL_0xCB14);
    opcodes.insert(0xCB15, instr_RL_0xCB15);
    opcodes.insert(0xCB12, instr_RL_0xCB12);
    opcodes.insert(0xCB13, instr_RL_0xCB13);
    opcodes.insert(0xCB10, instr_RL_0xCB10);
    opcodes.insert(0xCB11, instr_RL_0xCB11);
    opcodes.insert(0xCBD1, instr_SET_0xCBD1);
    opcodes.insert(0xCBD0, instr_SET_0xCBD0);
    opcodes.insert(0xCBD3, instr_SET_0xCBD3);
    opcodes.insert(0xCBD2, instr_SET_0xCBD2);
    opcodes.insert(0xCBD5, instr_SET_0xCBD5);
    opcodes.insert(0xCBD4, instr_SET_0xCBD4);
    opcodes.insert(0xCBD7, instr_SET_0xCBD7);
    opcodes.insert(0xCBD6, instr_SET_0xCBD6);
    opcodes.insert(0xCBD9, instr_SET_0xCBD9);
    opcodes.insert(0xCBD8, instr_SET_0xCBD8);
    opcodes.insert(0xCB69, instr_BIT_0xCB69);
    opcodes.insert(0xCB68, instr_BIT_0xCB68);
    opcodes.insert(0xCB63, instr_BIT_0xCB63);
    opcodes.insert(0xCB62, instr_BIT_0xCB62);
    opcodes.insert(0xCB61, instr_BIT_0xCB61);
    opcodes.insert(0xCB60, instr_BIT_0xCB60);
    opcodes.insert(0xCB67, instr_BIT_0xCB67);
    opcodes.insert(0xCB66, instr_BIT_0xCB66);
    opcodes.insert(0xCB65, instr_BIT_0xCB65);
    opcodes.insert(0xCB64, instr_BIT_0xCB64);
    opcodes.insert(0xCB85, instr_RES_0xCB85);
    opcodes.insert(0xCB84, instr_RES_0xCB84);
    opcodes.insert(0xCB87, instr_RES_0xCB87);
    opcodes.insert(0xCB86, instr_RES_0xCB86);
    opcodes.insert(0xCB81, instr_RES_0xCB81);
    opcodes.insert(0xCB80, instr_RES_0xCB80);
    opcodes.insert(0xCB83, instr_RES_0xCB83);
    opcodes.insert(0xCB82, instr_RES_0xCB82);
    opcodes.insert(0xCB89, instr_RES_0xCB89);
    opcodes.insert(0xCB88, instr_RES_0xCB88);
    opcodes.insert(0x3E, instr_LD_0x3E);
    opcodes.insert(0x3D, instr_DEC_0x3D);
    opcodes.insert(0x3F, instr_CCF_0x3F);
    opcodes.insert(0x3A, instr_LD_0x3A);
    opcodes.insert(0x3C, instr_INC_0x3C);
    opcodes.insert(0x3B, instr_DEC_0x3B);
    opcodes.insert(0x87, instr_ADD_0x87);
    opcodes.insert(0xDE, instr_SBC_0xDE);
    opcodes.insert(0xDF, instr_RST_0xDF);
    opcodes.insert(0x35, instr_DEC_0x35);
    opcodes.insert(0x34, instr_INC_0x34);
    opcodes.insert(0x37, instr_SCF_0x37);
    opcodes.insert(0x36, instr_LD_0x36);
    opcodes.insert(0x31, instr_LD_0x31);
    opcodes.insert(0x30, instr_JR_0x30);
    opcodes.insert(0x33, instr_INC_0x33);
    opcodes.insert(0x32, instr_LD_0x32);
    opcodes.insert(0x39, instr_ADD_0x39);
    opcodes.insert(0x38, instr_JR_0x38);
    opcodes.insert(0xCB8E, instr_RES_0xCB8E);
    opcodes.insert(0xCB8D, instr_RES_0xCB8D);
    opcodes.insert(0xCB8F, instr_RES_0xCB8F);
    opcodes.insert(0xCB8A, instr_RES_0xCB8A);
    opcodes.insert(0xCB8C, instr_RES_0xCB8C);
    opcodes.insert(0xCB8B, instr_RES_0xCB8B);
    opcodes.insert(0x88, instr_ADC_0x88);
    opcodes.insert(0x89, instr_ADC_0x89);
    opcodes.insert(0x8D, instr_ADC_0x8D);
    opcodes.insert(0xD1, instr_POP_0xD1);
    opcodes.insert(0xD2, instr_JP_0xD2);
    opcodes.insert(0xD5, instr_PUSH_0xD5);
    opcodes.insert(0xD6, instr_SUB_0xD6);
    opcodes.insert(0xD7, instr_RST_0xD7);
    opcodes.insert(0xD8, instr_RET_0xD8);
    opcodes.insert(0xD9, instr_RETI_0xD9);
    opcodes.insert(0xA9, instr_XOR_0xA9);
    opcodes.insert(0xA8, instr_XOR_0xA8);
    opcodes.insert(0xA7, instr_AND_0xA7);
    opcodes.insert(0xA6, instr_AND_0xA6);
    opcodes.insert(0xA5, instr_AND_0xA5);
    opcodes.insert(0xA4, instr_AND_0xA4);
    opcodes.insert(0xA3, instr_AND_0xA3);
    opcodes.insert(0xA2, instr_AND_0xA2);
    opcodes.insert(0xA1, instr_AND_0xA1);
    opcodes.insert(0xA0, instr_AND_0xA0);
    opcodes.insert(0xCBC8, instr_SET_0xCBC8);
    opcodes.insert(0xCBC9, instr_SET_0xCBC9);
    opcodes.insert(0x7A, instr_LD_0x7A);
    opcodes.insert(0x7C, instr_LD_0x7C);
    opcodes.insert(0x7B, instr_LD_0x7B);
    opcodes.insert(0x7E, instr_LD_0x7E);
    opcodes.insert(0x7D, instr_LD_0x7D);
    opcodes.insert(0x7F, instr_LD_0x7F);
    opcodes.insert(0xCB52, instr_BIT_0xCB52);
    opcodes.insert(0xCB53, instr_BIT_0xCB53);
    opcodes.insert(0xCB50, instr_BIT_0xCB50);
    opcodes.insert(0xCB51, instr_BIT_0xCB51);
    opcodes.insert(0xCB56, instr_BIT_0xCB56);
    opcodes.insert(0xCB57, instr_BIT_0xCB57);
    opcodes.insert(0xCB54, instr_BIT_0xCB54);
    opcodes.insert(0xCB55, instr_BIT_0xCB55);
    opcodes.insert(0xCBC0, instr_SET_0xCBC0);
    opcodes.insert(0xCBC1, instr_SET_0xCBC1);
    opcodes.insert(0xCB58, instr_BIT_0xCB58);
    opcodes.insert(0xCBC3, instr_SET_0xCBC3);
    opcodes.insert(0xCBC4, instr_SET_0xCBC4);
    opcodes.insert(0xCBC5, instr_SET_0xCBC5);
    opcodes.insert(0xCBC6, instr_SET_0xCBC6);
    opcodes.insert(0xCBC7, instr_SET_0xCBC7);
    opcodes.insert(0xCB5B, instr_BIT_0xCB5B);
    opcodes.insert(0xCB5C, instr_BIT_0xCB5C);
    opcodes.insert(0xCB5A, instr_BIT_0xCB5A);
    opcodes.insert(0xCB5F, instr_BIT_0xCB5F);
    opcodes.insert(0xCB5D, instr_BIT_0xCB5D);
    opcodes.insert(0xCB5E, instr_BIT_0xCB5E);
    opcodes.insert(0xCBCA, instr_SET_0xCBCA);
    opcodes.insert(0xCBCB, instr_SET_0xCBCB);
    opcodes.insert(0xCBCC, instr_SET_0xCBCC);
    opcodes.insert(0xCBCD, instr_SET_0xCBCD);
    opcodes.insert(0xCBCE, instr_SET_0xCBCE);
    opcodes.insert(0xCBCF, instr_SET_0xCBCF);
    opcodes.insert(0x79, instr_LD_0x79);
    opcodes.insert(0x78, instr_LD_0x78);
    opcodes.insert(0x71, instr_LD_0x71);
    opcodes.insert(0x70, instr_LD_0x70);
    opcodes.insert(0x73, instr_LD_0x73);
    opcodes.insert(0x72, instr_LD_0x72);
    opcodes.insert(0x75, instr_LD_0x75);
    opcodes.insert(0x74, instr_LD_0x74);
    opcodes.insert(0x77, instr_LD_0x77);
    opcodes.insert(0x76, instr_HALT_0x76);
    opcodes.insert(0xCB59, instr_BIT_0xCB59);
    opcodes.insert(0xAF, instr_XOR_0xAF);
    opcodes.insert(0xAE, instr_XOR_0xAE);
    opcodes.insert(0xAD, instr_XOR_0xAD);
    opcodes.insert(0xAC, instr_XOR_0xAC);
    opcodes.insert(0xAB, instr_XOR_0xAB);
    opcodes.insert(0xAA, instr_XOR_0xAA);
    opcodes.insert(0x9F, instr_SBC_0x9F);
    opcodes.insert(0x9E, instr_SBC_0x9E);
    opcodes.insert(0x9D, instr_SBC_0x9D);
    opcodes.insert(0x9C, instr_SBC_0x9C);
    opcodes.insert(0x9B, instr_SBC_0x9B);
    opcodes.insert(0x9A, instr_SBC_0x9A);
    opcodes.insert(0x08, instr_LD_0x08);
    opcodes.insert(0x09, instr_ADD_0x09);
    opcodes.insert(0x04, instr_INC_0x04);
    opcodes.insert(0x05, instr_DEC_0x05);
    opcodes.insert(0x06, instr_LD_0x06);
    opcodes.insert(0x07, instr_RLCA_0x07);
    opcodes.insert(0x00, instr_NOP_0x00);
    opcodes.insert(0x01, instr_LD_0x01);
    opcodes.insert(0x02, instr_LD_0x02);
    opcodes.insert(0x03, instr_INC_0x03);
    opcodes.insert(0x0D, instr_DEC_0x0D);
    opcodes.insert(0x0E, instr_LD_0x0E);
    opcodes.insert(0x0F, instr_RRCA_0x0F);
    opcodes.insert(0x0A, instr_LD_0x0A);
    opcodes.insert(0x0B, instr_DEC_0x0B);
    opcodes.insert(0x0C, instr_INC_0x0C);
    opcodes.insert(0x97, instr_SUB_0x97);
    opcodes.insert(0x96, instr_SUB_0x96);
    opcodes.insert(0x95, instr_SUB_0x95);
    opcodes.insert(0x94, instr_SUB_0x94);
    opcodes.insert(0x93, instr_SUB_0x93);
    opcodes.insert(0x92, instr_SUB_0x92);
    opcodes.insert(0x91, instr_SUB_0x91);
    opcodes.insert(0x90, instr_SUB_0x90);
    opcodes.insert(0x99, instr_SBC_0x99);
    opcodes.insert(0x98, instr_SBC_0x98);
    opcodes.insert(0xE2, instr_LD_0xE2);
    opcodes.insert(0xE1, instr_POP_0xE1);
    opcodes.insert(0xE0, instr_LDH_0xE0);
    opcodes.insert(0xCB09, instr_RRC_0xCB09);
    opcodes.insert(0xCB08, instr_RRC_0xCB08);
    opcodes.insert(0xE5, instr_PUSH_0xE5);
    opcodes.insert(0xCB05, instr_RLC_0xCB05);
    opcodes.insert(0xCB04, instr_RLC_0xCB04);
    opcodes.insert(0xCB07, instr_RLC_0xCB07);
    opcodes.insert(0xCB06, instr_RLC_0xCB06);
    opcodes.insert(0xCB01, instr_RLC_0xCB01);
    opcodes.insert(0xCB00, instr_RLC_0xCB00);
    opcodes.insert(0xCB03, instr_RLC_0xCB03);
    opcodes.insert(0xCB02, instr_RLC_0xCB02);
    opcodes.insert(0xCBE5, instr_SET_0xCBE5);
    opcodes.insert(0xEA, instr_LD_0xEA);
    opcodes.insert(0xEF, instr_RST_0xEF);
    opcodes.insert(0xEE, instr_XOR_0xEE);
    opcodes.insert(0xCB0E, instr_RRC_0xCB0E);
    opcodes.insert(0xCB0D, instr_RRC_0xCB0D);
    opcodes.insert(0xCB0F, instr_RRC_0xCB0F);
    opcodes.insert(0xCB0A, instr_RRC_0xCB0A);
    opcodes.insert(0xCB0C, instr_RRC_0xCB0C);
    opcodes.insert(0xCB0B, instr_RRC_0xCB0B);
    opcodes.insert(0x40, instr_LD_0x40);
    opcodes.insert(0x41, instr_LD_0x41);
    opcodes.insert(0x42, instr_LD_0x42);
    opcodes.insert(0x43, instr_LD_0x43);
    opcodes.insert(0x44, instr_LD_0x44);
    opcodes.insert(0x45, instr_LD_0x45);
    opcodes.insert(0x46, instr_LD_0x46);
    opcodes.insert(0x47, instr_LD_0x47);
    opcodes.insert(0x48, instr_LD_0x48);
    opcodes.insert(0x49, instr_LD_0x49);
    opcodes.insert(0x4A, instr_LD_0x4A);
    opcodes.insert(0x4B, instr_LD_0x4B);
    opcodes.insert(0x4C, instr_LD_0x4C);
    opcodes.insert(0x4D, instr_LD_0x4D);
    opcodes.insert(0x4E, instr_LD_0x4E);
    opcodes.insert(0x4F, instr_LD_0x4F);
    opcodes.insert(0xCB4A, instr_BIT_0xCB4A);
    opcodes.insert(0xCB4C, instr_BIT_0xCB4C);
    opcodes.insert(0xCB4B, instr_BIT_0xCB4B);
    opcodes.insert(0xCB4E, instr_BIT_0xCB4E);
    opcodes.insert(0xCB4D, instr_BIT_0xCB4D);
    opcodes.insert(0xCB4F, instr_BIT_0xCB4F);
    opcodes.insert(0xCBBF, instr_RES_0xCBBF);
    opcodes.insert(0xCBBE, instr_RES_0xCBBE);
    opcodes.insert(0xCBBD, instr_RES_0xCBBD);
    opcodes.insert(0xCBBC, instr_RES_0xCBBC);
    opcodes.insert(0xCBBB, instr_RES_0xCBBB);
    opcodes.insert(0xCBBA, instr_RES_0xCBBA);
    opcodes.insert(0xFE, instr_CP_0xFE);
    opcodes.insert(0xBB, instr_CP_0xBB);
    opcodes.insert(0xBC, instr_CP_0xBC);
    opcodes.insert(0xBA, instr_CP_0xBA);
    opcodes.insert(0xBF, instr_CP_0xBF);
    opcodes.insert(0xBD, instr_CP_0xBD);
    opcodes.insert(0xBE, instr_CP_0xBE);
    opcodes.insert(0xB8, instr_CP_0xB8);
    opcodes.insert(0xB9, instr_CP_0xB9);
    opcodes.insert(0xB2, instr_OR_0xB2);
    opcodes.insert(0xB3, instr_OR_0xB3);
    opcodes.insert(0xB0, instr_OR_0xB0);
    opcodes.insert(0xB1, instr_OR_0xB1);
    opcodes.insert(0xB6, instr_OR_0xB6);
    opcodes.insert(0xB7, instr_OR_0xB7);
    opcodes.insert(0xB4, instr_OR_0xB4);
    opcodes.insert(0xB5, instr_OR_0xB5);
    opcodes.insert(0xCBB7, instr_RES_0xCBB7);
    opcodes.insert(0xCBB6, instr_RES_0xCBB6);
    opcodes.insert(0xCBB5, instr_RES_0xCBB5);
    opcodes.insert(0xCBB4, instr_RES_0xCBB4);
    opcodes.insert(0xCBB3, instr_RES_0xCBB3);
    opcodes.insert(0xCBB2, instr_RES_0xCBB2);
    opcodes.insert(0xCBB1, instr_RES_0xCBB1);
    opcodes.insert(0xCBB0, instr_RES_0xCBB0);
    opcodes.insert(0xFB, instr_EI_0xFB);
    opcodes.insert(0xCBB9, instr_RES_0xCBB9);
    opcodes.insert(0xCBB8, instr_RES_0xCBB8);
    opcodes.insert(0xCB41, instr_BIT_0xCB41);
    opcodes.insert(0xCB40, instr_BIT_0xCB40);
    opcodes.insert(0xCB43, instr_BIT_0xCB43);
    opcodes.insert(0xCB42, instr_BIT_0xCB42);
    opcodes.insert(0xCB45, instr_BIT_0xCB45);
    opcodes.insert(0xCB44, instr_BIT_0xCB44);
    opcodes.insert(0xCB47, instr_BIT_0xCB47);
    opcodes.insert(0xCB46, instr_BIT_0xCB46);
    opcodes.insert(0xCB49, instr_BIT_0xCB49);
    opcodes.insert(0xCB48, instr_BIT_0xCB48);
    opcodes.insert(0x1F, instr_RRA_0x1F);
    opcodes.insert(0x1E, instr_LD_0x1E);
    opcodes.insert(0x1D, instr_DEC_0x1D);
    opcodes.insert(0x1C, instr_INC_0x1C);
    opcodes.insert(0x1B, instr_DEC_0x1B);
    opcodes.insert(0x1A, instr_LD_0x1A);
    opcodes.insert(0xF6, instr_OR_0xF6);
    opcodes.insert(0xF7, instr_RST_0xF7);
    opcodes.insert(0xCBE2, instr_SET_0xCBE2);
    opcodes.insert(0xF5, instr_PUSH_0xF5);
    opcodes.insert(0xCB34, instr_SWAP_0xCB34);
    opcodes.insert(0xCBE0, instr_SET_0xCBE0);
    opcodes.insert(0xF3, instr_DI_0xF3);
    opcodes.insert(0xCBE1, instr_SET_0xCBE1);
    opcodes.insert(0xF0, instr_LDH_0xF0);
    opcodes.insert(0xCBE6, instr_SET_0xCBE6);
    opcodes.insert(0xF1, instr_POP_0xF1);
    opcodes.insert(0xCBE7, instr_SET_0xCBE7);
    opcodes.insert(0x19, instr_ADD_0x19);
    opcodes.insert(0x18, instr_JR_0x18);
    opcodes.insert(0x17, instr_RLA_0x17);
    opcodes.insert(0x16, instr_LD_0x16);
    opcodes.insert(0x15, instr_DEC_0x15);
    opcodes.insert(0x14, instr_INC_0x14);
    opcodes.insert(0x13, instr_INC_0x13);
    opcodes.insert(0x12, instr_LD_0x12);
    opcodes.insert(0x11, instr_LD_0x11);
    opcodes.insert(0x10, instr_STOP_0x10);
    opcodes.insert(0xCBFC, instr_SET_0xCBFC);
    opcodes.insert(0xCBFB, instr_SET_0xCBFB);
    opcodes.insert(0xCBFA, instr_SET_0xCBFA);
    opcodes.insert(0xCBFF, instr_SET_0xCBFF);
    opcodes.insert(0xCBFE, instr_SET_0xCBFE);
    opcodes.insert(0xCBFD, instr_SET_0xCBFD);
    opcodes.insert(0xFF, instr_RST_0xFF);
    opcodes.insert(0xCB3A, instr_SRL_0xCB3A);
    opcodes.insert(0xCB3B, instr_SRL_0xCB3B);
    opcodes.insert(0xCB3C, instr_SRL_0xCB3C);
    opcodes.insert(0xCB3D, instr_SRL_0xCB3D);
    opcodes.insert(0xCB3E, instr_SRL_0xCB3E);
    opcodes.insert(0xCB3F, instr_SRL_0xCB3F);
    opcodes.insert(0xFA, instr_LD_0xFA);
    opcodes.insert(0xCB38, instr_SRL_0xCB38);
    opcodes.insert(0xCB39, instr_SRL_0xCB39);
    opcodes.insert(0xF8, instr_LD_0xF8);
    opcodes.insert(0xF9, instr_LD_0xF9);
    opcodes.insert(0xCB30, instr_SWAP_0xCB30);
    opcodes.insert(0xCB31, instr_SWAP_0xCB31);
    opcodes.insert(0xCB32, instr_SWAP_0xCB32);
    opcodes.insert(0xCB33, instr_SWAP_0xCB33);
    opcodes.insert(0xF2, instr_LD_0xF2);
    opcodes.insert(0xCB35, instr_SWAP_0xCB35);
    opcodes.insert(0xCB36, instr_SWAP_0xCB36);
    opcodes.insert(0xCB37, instr_SWAP_0xCB37);
    opcodes.insert(0xCBF9, instr_SET_0xCBF9);
    opcodes.insert(0xCBF8, instr_SET_0xCBF8);
    opcodes.insert(0xCBF3, instr_SET_0xCBF3);
    opcodes.insert(0xCBF2, instr_SET_0xCBF2);
    opcodes.insert(0xCBF1, instr_SET_0xCBF1);
    opcodes.insert(0xCBF0, instr_SET_0xCBF0);
    opcodes.insert(0xCBF7, instr_SET_0xCBF7);
    opcodes.insert(0xCBF6, instr_SET_0xCBF6);
    opcodes.insert(0xCBF5, instr_SET_0xCBF5);
    opcodes.insert(0xCBF4, instr_SET_0xCBF4);
    opcodes.insert(0x5C, instr_LD_0x5C);
    opcodes.insert(0x5B, instr_LD_0x5B);
    opcodes.insert(0x5A, instr_LD_0x5A);
    opcodes.insert(0x5F, instr_LD_0x5F);
    opcodes.insert(0x5E, instr_LD_0x5E);
    opcodes.insert(0x5D, instr_LD_0x5D);
    opcodes.insert(0xCBA8, instr_RES_0xCBA8);
    opcodes.insert(0xCBA9, instr_RES_0xCBA9);
    opcodes.insert(0xCBA6, instr_RES_0xCBA6);
    opcodes.insert(0xCBA7, instr_RES_0xCBA7);
    opcodes.insert(0xCBA4, instr_RES_0xCBA4);
    opcodes.insert(0xCBA5, instr_RES_0xCBA5);
    opcodes.insert(0xCBA2, instr_RES_0xCBA2);
    opcodes.insert(0xCBA3, instr_RES_0xCBA3);
    opcodes.insert(0xCBA0, instr_RES_0xCBA0);
    opcodes.insert(0xCBA1, instr_RES_0xCBA1);
    opcodes.insert(0xCBAF, instr_RES_0xCBAF);
    opcodes.insert(0xCBAD, instr_RES_0xCBAD);
    opcodes.insert(0xCBAE, instr_RES_0xCBAE);
    opcodes.insert(0xCBAB, instr_RES_0xCBAB);
    opcodes.insert(0xCBAC, instr_RES_0xCBAC);
    opcodes.insert(0xCBAA, instr_RES_0xCBAA);
    opcodes.insert(0x53, instr_LD_0x53);
    opcodes.insert(0x52, instr_LD_0x52);
    opcodes.insert(0x51, instr_LD_0x51);
    opcodes.insert(0x50, instr_LD_0x50);
    opcodes.insert(0x57, instr_LD_0x57);
    opcodes.insert(0x56, instr_LD_0x56);
    opcodes.insert(0x55, instr_LD_0x55);
    opcodes.insert(0x54, instr_LD_0x54);
    opcodes.insert(0x59, instr_LD_0x59);
    opcodes.insert(0x58, instr_LD_0x58);
    opcodes.insert(0xCBEB, instr_SET_0xCBEB);
    opcodes.insert(0xCBEC, instr_SET_0xCBEC);
    opcodes.insert(0xCBEA, instr_SET_0xCBEA);
    opcodes.insert(0xCBEF, instr_SET_0xCBEF);
    opcodes.insert(0xCBED, instr_SET_0xCBED);
    opcodes.insert(0xCBEE, instr_SET_0xCBEE);
    opcodes.insert(0xC5, instr_PUSH_0xC5);
    opcodes.insert(0xC4, instr_CALL_0xC4);
    opcodes.insert(0xC7, instr_RST_0xC7);
    opcodes.insert(0xC6, instr_ADD_0xC6);
    opcodes.insert(0xC1, instr_POP_0xC1);
    opcodes.insert(0xC0, instr_RET_0xC0);
    opcodes.insert(0xC3, instr_JP_0xC3);
    opcodes.insert(0xC2, instr_JP_0xC2);
    opcodes.insert(0xC9, instr_RET_0xC9);
    opcodes.insert(0xC8, instr_RET_0xC8);
    opcodes.insert(0xCB9F, instr_RES_0xCB9F);
    opcodes.insert(0xCBE3, instr_SET_0xCBE3);
    opcodes.insert(0xCB9D, instr_RES_0xCB9D);
    opcodes.insert(0xCB9E, instr_RES_0xCB9E);
    opcodes.insert(0xCB9B, instr_RES_0xCB9B);
    opcodes.insert(0xCB9C, instr_RES_0xCB9C);
    opcodes.insert(0xCBE4, instr_SET_0xCBE4);
    opcodes.insert(0xCB9A, instr_RES_0xCB9A);
    opcodes.insert(0xCBE8, instr_SET_0xCBE8);
    opcodes.insert(0xCBE9, instr_SET_0xCBE9);
    opcodes.insert(0xCBC2, instr_SET_0xCBC2);
    opcodes.insert(0xCB78, instr_BIT_0xCB78);
    opcodes.insert(0xCB79, instr_BIT_0xCB79);
    opcodes.insert(0xCB74, instr_BIT_0xCB74);
    opcodes.insert(0xCB75, instr_BIT_0xCB75);
    opcodes.insert(0xCB76, instr_BIT_0xCB76);
    opcodes.insert(0xCB77, instr_BIT_0xCB77);
    opcodes.insert(0xCB70, instr_BIT_0xCB70);
    opcodes.insert(0xCB71, instr_BIT_0xCB71);
    opcodes.insert(0xCB72, instr_BIT_0xCB72);
    opcodes.insert(0xCB73, instr_BIT_0xCB73);
    opcodes.insert(0xCB7D, instr_BIT_0xCB7D);
    opcodes.insert(0xCB7E, instr_BIT_0xCB7E);
    opcodes.insert(0xCB7F, instr_BIT_0xCB7F);
    opcodes.insert(0xCB7A, instr_BIT_0xCB7A);
    opcodes.insert(0xCB7B, instr_BIT_0xCB7B);
    opcodes.insert(0xCB7C, instr_BIT_0xCB7C);
    opcodes.insert(0xCB96, instr_RES_0xCB96);
    opcodes.insert(0xCB97, instr_RES_0xCB97);
    opcodes.insert(0xCB94, instr_RES_0xCB94);
    opcodes.insert(0xCB95, instr_RES_0xCB95);
    opcodes.insert(0xCB92, instr_RES_0xCB92);
    opcodes.insert(0xCB93, instr_RES_0xCB93);
    opcodes.insert(0xCB90, instr_RES_0xCB90);
    opcodes.insert(0xCB91, instr_RES_0xCB91);
    opcodes.insert(0xCB98, instr_RES_0xCB98);
    opcodes.insert(0xCB99, instr_RES_0xCB99);
    opcodes.insert(0xCE, instr_ADC_0xCE);
    opcodes.insert(0xCD, instr_CALL_0xCD);
    opcodes.insert(0xCF, instr_RST_0xCF);
    opcodes.insert(0xCA, instr_JP_0xCA);
    opcodes.insert(0xCC, instr_CALL_0xCC);
    opcodes.insert(0xCB, instr_PREFIX_0xCB);

