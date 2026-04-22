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

fn main() {
    let mut hart = Hart::new();
    let mut ram = [0u8; 65536];
    ram[0] = 0o231; // lui x3, 128
    ram[1] = 0;
    ram[2] = 0o022; // li x2, 31
    ram[3] = 0x7c;
    ram[4] = 0o011; // lui x1, 256
    ram[5] = 0x1;
    ram[6] = 0o022; // 1: addi x2, x2, 1
    ram[7] = 0x5;
    ram[8] = 0o204; // sb x2, x1
    ram[9] = 0x8;
    ram[10] = 0o163; // blt x2, x3, 1b
    ram[11] = 0xed;
    ram[12] = 7; // jalr zero, zero, 0
    ram[13] = 0;
    while hart.step(&mut |addr, value, we| {
        if we {
            ram[addr as usize] = value;
            if addr == 0x100 {
                print!("{}", value as char);
            }
            value
        } else {
            ram[addr as usize]
        }
    }) {
        //println!("{hart:?}");
    }
    println!("{hart:?}");
}
