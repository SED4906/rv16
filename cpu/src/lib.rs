pub struct Instruction(pub u16);

pub enum ROperation {
    Add,
    Sub,
    Xor,
    Or,
    And,
    Sll,
    Srl,
    Sra,
    Slt,
    Sltu,
}

pub enum IOperation {
    Lb,
    Lh,
    Addi,
    Lbu,
    Jalr,
    Ecall,
    Ebreak,
}

pub enum SOperation {
    Sb,
    Sh,
    Beq,
    Blt,
}

pub enum UOperation {
    Lui,
    Auipc,
}

pub enum Operation {
    Invalid,
    Reserved,
    R(ROperation),
    I(IOperation),
    S(SOperation),
    U(UOperation),
}

impl Instruction {
    pub fn operation(&self) -> Operation {
        if self.0 == 0 || self.0 == (-1i16 as u16) {
            return Operation::Invalid;
        }
        match self.0 & 0x7 {
            0 => {
                if self.0 & (1 << 6) != 0 {
                    Operation::I(IOperation::Lh)
                } else {
                    Operation::I(IOperation::Lb)
                }
            }
            1 => Operation::U(UOperation::Lui),
            2 => {
                if self.0 & (1 << 6) != 0 {
                    Operation::I(IOperation::Lbu)
                } else {
                    Operation::I(IOperation::Addi)
                }
            }
            3 => {
                if self.0 & (1 << 6) != 0 {
                    Operation::S(SOperation::Blt)
                } else {
                    Operation::S(SOperation::Beq)
                }
            }
            4 => {
                if self.0 & (1 << 6) != 0 {
                    Operation::S(SOperation::Sh)
                } else {
                    Operation::S(SOperation::Sb)
                }
            }
            5 => Operation::U(UOperation::Auipc),
            6 => match self.0 >> 13 {
                0 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::R(ROperation::Sub)
                    } else {
                        Operation::R(ROperation::Add)
                    }
                }
                1 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::Sll)
                    }
                }
                2 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::Slt)
                    }
                }
                3 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::Sltu)
                    }
                }
                4 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::Xor)
                    }
                }
                5 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::R(ROperation::Sra)
                    } else {
                        Operation::R(ROperation::Srl)
                    }
                }
                6 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::Or)
                    }
                }
                7 => {
                    if self.0 & (1 << 6) != 0 {
                        Operation::Reserved
                    } else {
                        Operation::R(ROperation::And)
                    }
                }
                _ => unreachable!(),
            },
            7 => {
                if self.0 & (1 << 6) != 0 {
                    match self.0 >> 11 {
                        0 => Operation::I(IOperation::Ecall),
                        1 => Operation::I(IOperation::Ebreak),
                        _ => Operation::Reserved,
                    }
                } else {
                    Operation::I(IOperation::Jalr)
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn rd(&self) -> usize {
        ((self.0 >> 3) & 0x7) as usize
    }

    pub fn rs1(&self) -> usize {
        ((self.0 >> 7) & 0x7) as usize
    }

    pub fn rs2(&self) -> usize {
        ((self.0 >> 10) & 0x7) as usize
    }

    pub fn imm(&self) -> i16 {
        (self.0 as i16) >> 10
    }

    pub fn imms(&self) -> i16 {
        ((self.0 >> 3) & 0x7) as i16 | (((self.0 as i16) >> 13) << 3)
    }
    pub fn immu(&self) -> i16 {
        (self.0 & 0xFFC0) as i16
    }

    fn register_name(which: usize) -> &'static str {
        match which {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for ROperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ROperation::Add => f.write_str("add"),
            ROperation::Sub => f.write_str("sub"),
            ROperation::Xor => f.write_str("xor"),
            ROperation::Or => f.write_str("or"),
            ROperation::And => f.write_str("and"),
            ROperation::Sll => f.write_str("sll"),
            ROperation::Srl => f.write_str("srl"),
            ROperation::Sra => f.write_str("sra"),
            ROperation::Slt => f.write_str("slt"),
            ROperation::Sltu => f.write_str("sltu"),
        }
    }
}

impl core::fmt::Display for IOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOperation::Lb => f.write_str("lb"),
            IOperation::Lh => f.write_str("lh"),
            IOperation::Addi => f.write_str("addi"),
            IOperation::Lbu => f.write_str("lbu"),
            IOperation::Jalr => f.write_str("jalr"),
            IOperation::Ecall => f.write_str("ecall"),
            IOperation::Ebreak => f.write_str("ebreak"),
        }
    }
}

impl core::fmt::Display for SOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SOperation::Sb => f.write_str("sb"),
            SOperation::Sh => f.write_str("sh"),
            SOperation::Beq => f.write_str("beq"),
            SOperation::Blt => f.write_str("blt"),
        }
    }
}

impl core::fmt::Display for UOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UOperation::Lui => f.write_str("lui"),
            UOperation::Auipc => f.write_str("auipc"),
        }
    }
}

impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = self.operation();
        match operation {
            Operation::Invalid => write!(f, ".dw 0x{:X} /* invalid */", self.0),
            Operation::Reserved => write!(f, ".dw 0x{:X} /* reserved */", self.0),
            Operation::R(roperation) => {
                let rd = Self::register_name(self.rd());
                let rs1 = Self::register_name(self.rs1());
                let rs2 = Self::register_name(self.rs2());
                write!(f, "{roperation} {rd}, {rs1}, {rs2}")
            }
            Operation::I(ioperation) => {
                let rd = Self::register_name(self.rd());
                let rs1 = Self::register_name(self.rs1());
                let imm = self.imm();
                match ioperation {
                    IOperation::Lb
                    | IOperation::Lh
                    | IOperation::Addi
                    | IOperation::Lbu
                    | IOperation::Jalr => write!(f, "{ioperation} {rd}, {imm}({rs1})"),
                    IOperation::Ecall | IOperation::Ebreak => write!(f, "{ioperation} {rd}, {rs1}"),
                }
            }
            Operation::S(soperation) => {
                let rs1 = Self::register_name(self.rs1());
                let rs2 = Self::register_name(self.rs2());
                let imm = self.imms();
                match soperation {
                    SOperation::Sb | SOperation::Sh => {
                        write!(f, "{soperation} {rs2}, {imm}({rs1})")
                    }
                    SOperation::Beq | SOperation::Blt => {
                        write!(f, "{soperation} {rs1}, {rs2}, {imm}")
                    }
                }
            }
            Operation::U(uoperation) => {
                let rd = Self::register_name(self.rd());
                let imm = self.immu();
                write!(f, "{uoperation} {rd}, {imm}")
            }
        }
    }
}

#[derive(Debug)]
pub struct Hart {
    registers: [u16; 7],
    pc: u16,
}

impl Hart {
    pub fn new() -> Self {
        Self {
            registers: [0; 7],
            pc: 0,
        }
    }

    pub fn step(&mut self, bus: &mut dyn FnMut(u16, u8, bool) -> u8) -> bool {
        let mut readb = |addr| bus(addr, 0, false);
        let instruction = Instruction(u16::from_le_bytes([readb(self.pc), readb(self.pc + 1)]));
        let operation = instruction.operation();
        match operation {
            Operation::Invalid => return false,
            Operation::Reserved => return false,
            Operation::R(roperation) => {
                let rs1 = instruction.rs1();
                let rs1 = if rs1 == 0 { 0 } else { self.registers[rs1 - 1] };
                let rs2 = instruction.rs2();
                let rs2 = if rs2 == 0 { 0 } else { self.registers[rs2 - 1] };
                let rd = instruction.rd();
                let rd = if rd == 0 {
                    &mut 0
                } else {
                    self.registers.get_mut(rd - 1).unwrap()
                };
                match roperation {
                    ROperation::Add => {
                        *rd = rs1.wrapping_add(rs2);
                    }
                    ROperation::Sub => {
                        *rd = rs1.wrapping_sub(rs2);
                    }
                    ROperation::Xor => {
                        *rd = rs1 ^ rs2;
                    }
                    ROperation::Or => {
                        *rd = rs1 | rs2;
                    }
                    ROperation::And => {
                        *rd = rs1 & rs2;
                    }
                    ROperation::Sll => {
                        *rd = rs1.unbounded_shl(rs2 as u32);
                    }
                    ROperation::Srl => {
                        *rd = rs1.unbounded_shr(rs2 as u32);
                    }
                    ROperation::Sra => {
                        *rd = rs1.unbounded_shr(rs2 as u32)
                            | if rs1 & 0x8000 == 0 {
                                0
                            } else {
                                0xFFFFu16 << 16u32.saturating_sub(rs2 as u32)
                            };
                    }
                    ROperation::Slt => {
                        *rd = if (rs1 as i16) < (rs2 as i16) { 1 } else { 0 };
                    }
                    ROperation::Sltu => {
                        *rd = if rs1 < rs2 { 1 } else { 0 };
                    }
                }
                self.pc += 2;
            }
            Operation::I(ioperation) => {
                let imm = instruction.imm();
                let rs1 = instruction.rs1();
                let rs1 = if rs1 == 0 { 0 } else { self.registers[rs1 - 1] };
                let rd = instruction.rd();
                let rd = if rd == 0 {
                    &mut 0
                } else {
                    self.registers.get_mut(rd - 1).unwrap()
                };
                match ioperation {
                    IOperation::Lb => {
                        *rd = ((readb(rs1.wrapping_add_signed(imm)) as i8) as i16) as u16;
                        self.pc += 2;
                    }
                    IOperation::Lh => {
                        *rd = u16::from_le_bytes([
                            readb(rs1.wrapping_add_signed(imm)),
                            readb(rs1.wrapping_add_signed(imm).wrapping_add(1)),
                        ]);
                        self.pc += 2;
                    }
                    IOperation::Addi => {
                        *rd = rs1.wrapping_add_signed(imm);
                        self.pc += 2;
                    }
                    IOperation::Lbu => {
                        *rd = readb(rs1.wrapping_add_signed(imm)) as u16;
                        self.pc += 2;
                    }
                    IOperation::Jalr => {
                        *rd = self.pc.wrapping_add(2);
                        self.pc = rs1.wrapping_add_signed(imm << 1);
                    }
                    IOperation::Ecall => {
                        self.pc += 2;
                    }
                    IOperation::Ebreak => {
                        self.pc += 2;
                    }
                }
            }

            Operation::S(soperation) => {
                let imm = instruction.imms();
                let rs1 = instruction.rs1();
                let rs1 = if rs1 == 0 { 0 } else { self.registers[rs1 - 1] };
                let rs2 = instruction.rs2();
                let rs2 = if rs2 == 0 { 0 } else { self.registers[rs2 - 1] };
                let mut writeb = |addr, value| bus(addr, value, true);
                match soperation {
                    SOperation::Sb => {
                        writeb(rs1.wrapping_add_signed(imm), rs2 as u8);
                        self.pc += 2;
                    }
                    SOperation::Sh => {
                        writeb(rs1.wrapping_add_signed(imm), rs2 as u8);
                        writeb(
                            rs1.wrapping_add_signed(imm).wrapping_add(1),
                            (rs2 >> 8) as u8,
                        );
                        self.pc += 2;
                    }
                    SOperation::Beq => {
                        if rs1 == rs2 {
                            self.pc = self.pc.wrapping_add_signed(imm << 1);
                        } else {
                            self.pc += 2;
                        }
                    }
                    SOperation::Blt => {
                        if rs1 < rs2 {
                            self.pc = self.pc.wrapping_add_signed(imm << 1);
                        } else {
                            self.pc += 2;
                        }
                    }
                }
            }
            Operation::U(uoperation) => {
                let imm = instruction.immu();
                let rd = instruction.rd();
                let rd = if rd == 0 {
                    &mut 0
                } else {
                    self.registers.get_mut(rd - 1).unwrap()
                };
                match uoperation {
                    UOperation::Lui => {
                        *rd = imm as u16;
                    }
                    UOperation::Auipc => {
                        *rd = self.pc.wrapping_add_signed(imm);
                    }
                }
                self.pc += 2;
            }
        }
        true
    }
}
