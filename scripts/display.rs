fn disp_NOP_0x00(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("NOP");
    (result, 1)
}

fn disp_LD_0x01(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("BC");
    result.push_str(", ");
    let d16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
    (result, 3)
}

fn disp_LD_0x02(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(BC)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_INC_0x03(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("BC");
    (result, 1)
}

fn disp_INC_0x04(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_DEC_0x05(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x06(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RLCA_0x07(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLCA");
    (result, 1)
}

fn disp_LD_0x08(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    let pa16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
    result.push_str(", ");
    result.push_str("SP");
    (result, 3)
}

fn disp_ADD_0x09(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    result.push_str("BC");
    (result, 1)
}

fn disp_LD_0x0A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(BC)");
    (result, 1)
}

fn disp_DEC_0x0B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("BC");
    (result, 1)
}

fn disp_INC_0x0C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_DEC_0x0D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x0E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RRCA_0x0F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRCA");
    (result, 1)
}

fn disp_STOP_0x10(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("STOP");
    result.push_str(" ");
    result.push_str("0");
    (result, 1)
}

fn disp_LD_0x11(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("DE");
    result.push_str(", ");
    let d16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
    (result, 3)
}

fn disp_LD_0x12(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(DE)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_INC_0x13(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("DE");
    (result, 1)
}

fn disp_INC_0x14(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_DEC_0x15(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x16(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RLA_0x17(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLA");
    (result, 1)
}

fn disp_JR_0x18(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JR");
    result.push_str(" ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_ADD_0x19(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    result.push_str("DE");
    (result, 1)
}

fn disp_LD_0x1A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(DE)");
    (result, 1)
}

fn disp_DEC_0x1B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("DE");
    (result, 1)
}

fn disp_INC_0x1C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_DEC_0x1D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x1E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RRA_0x1F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRA");
    (result, 1)
}

fn disp_JR_0x20(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JR");
    result.push_str(" ");
    result.push_str("NZ");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_LD_0x21(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    let d16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
    (result, 3)
}

fn disp_LD_0x22(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL+)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_INC_0x23(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("HL");
    (result, 1)
}

fn disp_INC_0x24(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_DEC_0x25(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x26(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_DAA_0x27(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DAA");
    (result, 1)
}

fn disp_JR_0x28(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JR");
    result.push_str(" ");
    result.push_str("Z");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_ADD_0x29(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    result.push_str("HL");
    (result, 1)
}

fn disp_LD_0x2A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL+)");
    (result, 1)
}

fn disp_DEC_0x2B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("HL");
    (result, 1)
}

fn disp_INC_0x2C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_DEC_0x2D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x2E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_CPL_0x2F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CPL");
    (result, 1)
}

fn disp_JR_0x30(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JR");
    result.push_str(" ");
    result.push_str("NC");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_LD_0x31(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("SP");
    result.push_str(", ");
    let d16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", d16).as_slice());
    (result, 3)
}

fn disp_LD_0x32(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL-)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_INC_0x33(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("SP");
    (result, 1)
}

fn disp_INC_0x34(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_DEC_0x35(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x36(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_SCF_0x37(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SCF");
    (result, 1)
}

fn disp_JR_0x38(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JR");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_ADD_0x39(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    result.push_str("SP");
    (result, 1)
}

fn disp_LD_0x3A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL-)");
    (result, 1)
}

fn disp_DEC_0x3B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("SP");
    (result, 1)
}

fn disp_INC_0x3C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("INC");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_DEC_0x3D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DEC");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x3E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_CCF_0x3F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CCF");
    (result, 1)
}

fn disp_LD_0x40(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x41(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x42(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x43(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x44(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x45(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x46(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x47(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("B");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x48(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x49(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x4A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x4B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x4C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x4D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x4E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x4F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x50(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x51(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x52(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x53(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x54(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x55(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x56(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x57(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("D");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x58(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x59(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x5A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x5B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x5C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x5D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x5E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x5F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("E");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x60(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x61(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x62(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x63(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x64(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x65(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x66(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x67(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("H");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x68(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x69(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x6A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x6B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x6C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x6D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x6E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x6F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("L");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x70(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x71(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x72(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x73(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x74(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x75(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_HALT_0x76(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("HALT");
    (result, 1)
}

fn disp_LD_0x77(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(HL)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_LD_0x78(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_LD_0x79(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_LD_0x7A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_LD_0x7B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_LD_0x7C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_LD_0x7D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_LD_0x7E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0x7F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_ADD_0x80(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_ADD_0x81(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_ADD_0x82(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_ADD_0x83(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_ADD_0x84(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_ADD_0x85(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_ADD_0x86(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_ADD_0x87(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_ADC_0x88(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_ADC_0x89(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_ADC_0x8A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_ADC_0x8B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_ADC_0x8C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_ADC_0x8D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_ADC_0x8E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_ADC_0x8F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_SUB_0x90(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_SUB_0x91(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_SUB_0x92(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_SUB_0x93(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_SUB_0x94(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_SUB_0x95(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_SUB_0x96(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_SUB_0x97(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_SBC_0x98(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("B");
    (result, 1)
}

fn disp_SBC_0x99(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("C");
    (result, 1)
}

fn disp_SBC_0x9A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("D");
    (result, 1)
}

fn disp_SBC_0x9B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("E");
    (result, 1)
}

fn disp_SBC_0x9C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("H");
    (result, 1)
}

fn disp_SBC_0x9D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("L");
    (result, 1)
}

fn disp_SBC_0x9E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_SBC_0x9F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_AND_0xA0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_AND_0xA1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_AND_0xA2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_AND_0xA3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_AND_0xA4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_AND_0xA5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_AND_0xA6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_AND_0xA7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_XOR_0xA8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_XOR_0xA9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_XOR_0xAA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_XOR_0xAB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_XOR_0xAC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_XOR_0xAD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_XOR_0xAE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_XOR_0xAF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_OR_0xB0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_OR_0xB1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_OR_0xB2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_OR_0xB3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_OR_0xB4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_OR_0xB5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_OR_0xB6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_OR_0xB7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_CP_0xB8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("B");
    (result, 1)
}

fn disp_CP_0xB9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_CP_0xBA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("D");
    (result, 1)
}

fn disp_CP_0xBB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("E");
    (result, 1)
}

fn disp_CP_0xBC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("H");
    (result, 1)
}

fn disp_CP_0xBD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("L");
    (result, 1)
}

fn disp_CP_0xBE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_CP_0xBF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    result.push_str("A");
    (result, 1)
}

fn disp_RET_0xC0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RET");
    result.push_str(" ");
    result.push_str("NZ");
    (result, 1)
}

fn disp_POP_0xC1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("POP");
    result.push_str(" ");
    result.push_str("BC");
    (result, 1)
}

fn disp_JP_0xC2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    result.push_str("NZ");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_JP_0xC3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_CALL_0xC4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CALL");
    result.push_str(" ");
    result.push_str("NZ");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_PUSH_0xC5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("PUSH");
    result.push_str(" ");
    result.push_str("BC");
    (result, 1)
}

fn disp_ADD_0xC6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xC7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("00H");
    (result, 1)
}

fn disp_RET_0xC8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RET");
    result.push_str(" ");
    result.push_str("Z");
    (result, 1)
}

fn disp_RET_0xC9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RET");
    (result, 1)
}

fn disp_JP_0xCA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    result.push_str("Z");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_PREFIX_0xCB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("PREFIX");
    result.push_str(" ");
    result.push_str("CB");
    (result, 1)
}

fn disp_RLC_0xCB00(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_RLC_0xCB01(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_RLC_0xCB02(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_RLC_0xCB03(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_RLC_0xCB04(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_RLC_0xCB05(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_RLC_0xCB06(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RLC_0xCB07(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RLC");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_RRC_0xCB08(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_RRC_0xCB09(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_RRC_0xCB0A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_RRC_0xCB0B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_RRC_0xCB0C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_RRC_0xCB0D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_RRC_0xCB0E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RRC_0xCB0F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RRC");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_RL_0xCB10(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_RL_0xCB11(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_RL_0xCB12(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_RL_0xCB13(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_RL_0xCB14(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_RL_0xCB15(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_RL_0xCB16(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RL_0xCB17(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RL");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_RR_0xCB18(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_RR_0xCB19(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_RR_0xCB1A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_RR_0xCB1B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_RR_0xCB1C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_RR_0xCB1D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_RR_0xCB1E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RR_0xCB1F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RR");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_SLA_0xCB20(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_SLA_0xCB21(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_SLA_0xCB22(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_SLA_0xCB23(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_SLA_0xCB24(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_SLA_0xCB25(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_SLA_0xCB26(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SLA_0xCB27(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SLA");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_SRA_0xCB28(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_SRA_0xCB29(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_SRA_0xCB2A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_SRA_0xCB2B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_SRA_0xCB2C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_SRA_0xCB2D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_SRA_0xCB2E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SRA_0xCB2F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRA");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_SWAP_0xCB30(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_SWAP_0xCB31(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_SWAP_0xCB32(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_SWAP_0xCB33(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_SWAP_0xCB34(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_SWAP_0xCB35(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_SWAP_0xCB36(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SWAP_0xCB37(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SWAP");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_SRL_0xCB38(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("B");
    (result, 2)
}

fn disp_SRL_0xCB39(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("C");
    (result, 2)
}

fn disp_SRL_0xCB3A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("D");
    (result, 2)
}

fn disp_SRL_0xCB3B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("E");
    (result, 2)
}

fn disp_SRL_0xCB3C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("H");
    (result, 2)
}

fn disp_SRL_0xCB3D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("L");
    (result, 2)
}

fn disp_SRL_0xCB3E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SRL_0xCB3F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SRL");
    result.push_str(" ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB40(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB41(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB42(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB43(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB44(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB45(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB46(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB47(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB48(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB49(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB4A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB4B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB4C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB4D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB4E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB4F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB50(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB51(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB52(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB53(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB54(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB55(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB56(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB57(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB58(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB59(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB5A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB5B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB5C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB5D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB5E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB5F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB60(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB61(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB62(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB63(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB64(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB65(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB66(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB67(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB68(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB69(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB6A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB6B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB6C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB6D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB6E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB6F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB70(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB71(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB72(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB73(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB74(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB75(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB76(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB77(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_BIT_0xCB78(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_BIT_0xCB79(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_BIT_0xCB7A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_BIT_0xCB7B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_BIT_0xCB7C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_BIT_0xCB7D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_BIT_0xCB7E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_BIT_0xCB7F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("BIT");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCB80(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCB81(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCB82(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCB83(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCB84(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCB85(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCB86(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCB87(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCB88(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCB89(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCB8A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCB8B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCB8C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCB8D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCB8E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCB8F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCB90(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCB91(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCB92(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCB93(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCB94(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCB95(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCB96(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCB97(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCB98(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCB99(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCB9A(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCB9B(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCB9C(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCB9D(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCB9E(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCB9F(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCBA0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCBA1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCBA2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCBA3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCBA4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCBA5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCBA6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCBA7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCBA8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCBA9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCBAA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCBAB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCBAC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCBAD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCBAE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCBAF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCBB0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCBB1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCBB2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCBB3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCBB4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCBB5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCBB6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCBB7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_RES_0xCBB8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_RES_0xCBB9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_RES_0xCBBA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_RES_0xCBBB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_RES_0xCBBC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_RES_0xCBBD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_RES_0xCBBE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_RES_0xCBBF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RES");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBC0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBC1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBC2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBC3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBC4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBC5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBC6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBC7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("0");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBC8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBC9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBCA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBCB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBCC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBCD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBCE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBCF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("1");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBD0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBD1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBD2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBD3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBD4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBD5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBD6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBD7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("2");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBD8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBD9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBDA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBDB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBDC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBDD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBDE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBDF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("3");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBE0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBE1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBE2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBE3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBE4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBE5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBE6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBE7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("4");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBE8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBE9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBEA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBEB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBEC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBED(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBEE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBEF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("5");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBF0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBF1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBF2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBF3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBF4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBF5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBF6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBF7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("6");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_SET_0xCBF8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("B");
    (result, 2)
}

fn disp_SET_0xCBF9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("C");
    (result, 2)
}

fn disp_SET_0xCBFA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("D");
    (result, 2)
}

fn disp_SET_0xCBFB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("E");
    (result, 2)
}

fn disp_SET_0xCBFC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("H");
    (result, 2)
}

fn disp_SET_0xCBFD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("L");
    (result, 2)
}

fn disp_SET_0xCBFE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("(HL)");
    (result, 2)
}

fn disp_SET_0xCBFF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SET");
    result.push_str(" ");
    result.push_str("7");
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_CALL_0xCC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CALL");
    result.push_str(" ");
    result.push_str("Z");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_CALL_0xCD(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CALL");
    result.push_str(" ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_ADC_0xCE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xCF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("08H");
    (result, 1)
}

fn disp_RET_0xD0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RET");
    result.push_str(" ");
    result.push_str("NC");
    (result, 1)
}

fn disp_POP_0xD1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("POP");
    result.push_str(" ");
    result.push_str("DE");
    (result, 1)
}

fn disp_JP_0xD2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    result.push_str("NC");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_CALL_0xD4(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CALL");
    result.push_str(" ");
    result.push_str("NC");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_PUSH_0xD5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("PUSH");
    result.push_str(" ");
    result.push_str("DE");
    (result, 1)
}

fn disp_SUB_0xD6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SUB");
    result.push_str(" ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xD7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("10H");
    (result, 1)
}

fn disp_RET_0xD8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RET");
    result.push_str(" ");
    result.push_str("C");
    (result, 1)
}

fn disp_RETI_0xD9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RETI");
    (result, 1)
}

fn disp_JP_0xDA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_CALL_0xDC(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CALL");
    result.push_str(" ");
    result.push_str("C");
    result.push_str(", ");
    let a16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("0x{:0>4.4X}", a16).as_slice());
    (result, 3)
}

fn disp_SBC_0xDE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("SBC");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xDF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("18H");
    (result, 1)
}

fn disp_LDH_0xE0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LDH");
    result.push_str(" ");
    let pa8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("(0x{:0>2.2X})", pa8).as_slice());
    result.push_str(", ");
    result.push_str("A");
    (result, 2)
}

fn disp_POP_0xE1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("POP");
    result.push_str(" ");
    result.push_str("HL");
    (result, 1)
}

fn disp_LD_0xE2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("(C)");
    result.push_str(", ");
    result.push_str("A");
    (result, 1)
}

fn disp_PUSH_0xE5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("PUSH");
    result.push_str(" ");
    result.push_str("HL");
    (result, 1)
}

fn disp_AND_0xE6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("AND");
    result.push_str(" ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xE7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("20H");
    (result, 1)
}

fn disp_ADD_0xE8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("ADD");
    result.push_str(" ");
    result.push_str("SP");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_JP_0xE9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("JP");
    result.push_str(" ");
    result.push_str("(HL)");
    (result, 1)
}

fn disp_LD_0xEA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    let pa16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
    result.push_str(", ");
    result.push_str("A");
    (result, 3)
}

fn disp_XOR_0xEE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("XOR");
    result.push_str(" ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xEF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("28H");
    (result, 1)
}

fn disp_LDH_0xF0(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LDH");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let pa8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("(0x{:0>2.2X})", pa8).as_slice());
    (result, 2)
}

fn disp_POP_0xF1(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("POP");
    result.push_str(" ");
    result.push_str("AF");
    (result, 1)
}

fn disp_LD_0xF2(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    result.push_str("(C)");
    (result, 1)
}

fn disp_DI_0xF3(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("DI");
    (result, 1)
}

fn disp_PUSH_0xF5(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("PUSH");
    result.push_str(" ");
    result.push_str("AF");
    (result, 1)
}

fn disp_OR_0xF6(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("OR");
    result.push_str(" ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xF7(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("30H");
    (result, 1)
}

fn disp_LD_0xF8(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("HL");
    result.push_str(", ");
    let r8: i8 = self.mmu.read8(self.pc + 1) as i8;
    result.push_str(format!("SP+0x{:0>2.2X}", r8).as_slice());
    (result, 2)
}

fn disp_LD_0xF9(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("SP");
    result.push_str(", ");
    result.push_str("HL");
    (result, 1)
}

fn disp_LD_0xFA(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("LD");
    result.push_str(" ");
    result.push_str("A");
    result.push_str(", ");
    let pa16: u16 = self.mmu.read16(self.pc + 1);
    result.push_str(format!("(0x{:0>4.4X})", pa16).as_slice());
    (result, 3)
}

fn disp_EI_0xFB(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("EI");
    (result, 1)
}

fn disp_CP_0xFE(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("CP");
    result.push_str(" ");
    let d8: u8 = self.mmu.read8(self.pc + 1);
    result.push_str(format!("0x{:0>2.2X}", d8).as_slice());
    (result, 2)
}

fn disp_RST_0xFF(&self) -> (String, u8) {
    let mut result: String = String::new();
    result.push_str("RST");
    result.push_str(" ");
    result.push_str("38H");
    (result, 1)
}

