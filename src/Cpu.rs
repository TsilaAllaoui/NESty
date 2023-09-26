use crate::opcode::Opcode;
use bit::BitIndex;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub register_s: u8,
    pub status: u8,
    pub pc: u16,
    pub opcodes: Vec<Opcode>,
    cycles: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            register_s: 0,
            status: 0,
            pc: 0,
            cycles: 0,
            memory: [0; 0xFFFF],
            #[rustfmt::skip]
            opcodes: {
                vec![
                    // BRK
                    Opcode::new( 0x00, String::from("BRK"), 1, 7, AddressingMode::NoneAddressing),

                    // INX
                    Opcode::new( 0xE8, String::from("INX"), 1, 2, AddressingMode::NoneAddressing),

                    // INY
                    Opcode::new( 0xC8, String::from("INY"), 1, 2, AddressingMode::NoneAddressing),

                    // Transfer Instructions
                    Opcode::new(0xA9, String::from("LDA"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA5, String::from("LDA"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB5, String::from("LDA"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0xAD, String::from("LDA"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBD, String::from("LDA"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0xB9, String::from("LDA"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0xA1, String::from("LDA"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0xB1, String::from("LDA"), 2, 5, AddressingMode::Indirect_Y),

                    Opcode::new(0xA2, String::from("LDX"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA6, String::from("LDX"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB6, String::from("LDX"), 2, 4, AddressingMode::ZeroPage_Y),
                    Opcode::new(0xAE, String::from("LDX"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBE, String::from("LDX"), 3, 4, AddressingMode::Absolute_Y),

                    Opcode::new(0xA0, String::from("LDY"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA4, String::from("LDY"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB4, String::from("LDY"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0xAC, String::from("LDY"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBC, String::from("LDY"), 3, 4, AddressingMode::Absolute_X),

                    Opcode::new(0x85, String::from("STA"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x95, String::from("STA"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x8D, String::from("STA"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0x9D, String::from("STA"), 3, 5, AddressingMode::Absolute_X),
                    Opcode::new(0x99, String::from("STA"), 3, 5, AddressingMode::Absolute_Y),
                    Opcode::new(0x81, String::from("STA"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0x91, String::from("STA"), 2, 6, AddressingMode::Indirect_Y),

                    Opcode::new(0x86, String::from("STX"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x96, String::from("STX"), 2, 4, AddressingMode::ZeroPage_Y),
                    Opcode::new(0x8E, String::from("STX"), 3, 4, AddressingMode::Absolute),

                    Opcode::new(0x84, String::from("STY"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x94, String::from("STY"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x8C, String::from("STY"), 3, 4, AddressingMode::Absolute),

                    Opcode::new( 0xAA, String::from("TAX"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0xA8, String::from("TAY"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0xBA, String::from("TSX"), 1, 2, AddressingMode::NoneAddressing),

                    Opcode::new( 0x8A, String::from("TXA"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0x9A, String::from("TXS"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0x98, String::from("TYA"), 1, 2, AddressingMode::NoneAddressing),

                    // Stack Instructions
                    Opcode::new( 0x48, String::from("PHA"), 1, 3, AddressingMode::NoneAddressing),
                    Opcode::new( 0x08, String::from("PHP"), 1, 3, AddressingMode::NoneAddressing),
                    Opcode::new( 0x68, String::from("PLA"), 1, 4, AddressingMode::NoneAddressing),
                    Opcode::new( 0x28, String::from("PLP"), 1, 4, AddressingMode::NoneAddressing),

                    // Status flags instructions
                    Opcode::new(0x18, String::from("CLC"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0xD8, String::from("CLD"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x58, String::from("CLI"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0xB8, String::from("CLV"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x38, String::from("SEC"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0xF8, String::from("SED"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x78, String::from("SEI"), 1, 2, AddressingMode::NoneAddressing),
                ]
            },
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_read_16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub fn mem_write_16(&mut self, addr: u16, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = (val & 0xff) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    pub fn mem_write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.register_s = 0xFF;
        self.status = 0;
        self.pc = self.mem_read_16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            let code = self.memory[self.pc as usize];
            self.pc += 1;

            let opcode = self
                .opcodes
                .iter()
                .filter(|o| o.code == code)
                .next()
                .expect("Can't get opcode from filter");

            match opcode.code {
                // BRK
                0x00 => {
                    self.brk();
                    break;
                }

                // Transfer Instructions
                0xAA => self.tax(),
                0xA8 => self.tay(),
                0xBA => self.tsx(),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode.mode),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode.mode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(opcode.mode),
                0x86 | 0x96 | 0x8E => self.stx(opcode.mode),
                0x84 | 0x94 | 0x8C => self.sty(opcode.mode),
                0x8A => self.txa(),
                0x9A => self.txs(),
                0x98 => self.tya(),

                // Stack Instructions
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),

                // INX
                0xE8 => self.inx(),

                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(opcode.mode),

                // Status flags instructions
                0x18 => self.clear_carry_flag(),
                0xD8 => self.clear_decimal_flag(),
                0x58 => self.clear_interrupt_flag(),
                0xB8 => self.clear_overflow_flag(),
                0x38 => self.set_carry_flag(),
                0xF8 => self.set_decimal_flag(),
                0x78 => self.set_interrupt_flag(),

                _ => {
                    self.brk();
                    break;
                }
            }
        }
    }

    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                self.cycles += 2;
                let pc = self.pc;
                self.pc += 1;
                pc
            }

            AddressingMode::ZeroPage => {
                self.cycles += 3;
                let pc = self.pc;
                self.pc += 1;
                self.mem_read(pc) as u16
            }

            AddressingMode::Absolute => {
                self.cycles += 4;
                let pc = self.pc;
                self.pc += 2;
                self.mem_read_16(pc)
            }

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.register_x) as u16;
                self.cycles += 4;
                self.pc += 1;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.register_y) as u16;
                self.cycles += 4;
                self.pc += 1;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_16(self.pc);
                let addr = base.wrapping_add(self.register_x as u16);
                self.cycles += 4;
                self.pc += 2;
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_16(self.pc);
                let addr = base.wrapping_add(self.register_y as u16);
                self.cycles += 4;
                self.pc += 2;
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                self.cycles += 6;
                self.pc += 1;
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.pc);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                self.cycles += 5;
                self.pc += 1;
                deref
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn set_flag(&mut self, flag: &str, val: bool) {
        match flag {
            "C" => {
                self.status.set_bit(0, val);
            }
            "Z" => {
                self.status.set_bit(1, val);
            }
            "I" => {
                self.status.set_bit(2, val);
            }
            "D" => {
                self.status.set_bit(3, val);
            }
            "B" => {
                self.status.set_bit(4, val);
            }
            "V" => {
                self.status.set_bit(6, val);
            }
            "N" => {
                self.status.set_bit(7, val);
            }
            _ => {
                panic!("Flag {} is unknown", flag);
            }
        }
    }

    pub fn get_flag(&mut self, flag: &str) -> bool {
        match flag {
            "C" => self.status.bit(0),
            "Z" => self.status.bit(1),
            "I" => self.status.bit(2),
            "D" => self.status.bit(3),
            "B" => self.status.bit(4),
            "V" => self.status.bit(6),
            "N" => self.status.bit(7),
            _ => false,
        }
    }

    /// ************** BRK instructions **************
    ///
    pub fn brk(&mut self) {
        self.status.set_bit(2, true);
        return;
    }

    /// ************** Transfer instructions **************
    ///
    pub fn ld_reg(&mut self, reg: &str, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let reg = match reg {
            "a" => {
                self.register_a = self.mem_read(addr);
                self.register_a
            }
            "x" => {
                self.register_x = self.mem_read(addr);
                self.register_x
            }
            "y" => {
                self.register_y = self.mem_read(addr);
                self.register_y
            }
            _ => panic!("Unknown Register: {}", reg),
        };

        self.set_flag("Z", reg == 0);
        self.set_flag("N", reg.bit(7));
    }

    pub fn lda(&mut self, mode: AddressingMode) {
        self.ld_reg("a", mode);
    }

    pub fn ldx(&mut self, mode: AddressingMode) {
        self.ld_reg("x", mode);
    }

    pub fn ldy(&mut self, mode: AddressingMode) {
        self.ld_reg("y", mode);
    }

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_flag("Z", self.register_x == 0);
        self.set_flag("N", self.register_x.bit(7));
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.set_flag("Z", self.register_y == 0);
        self.set_flag("N", self.register_y.bit(7));
    }

    pub fn tsx(&mut self) {
        self.register_x = self.mem_read(self.register_s as u16);
        self.set_flag("Z", self.register_x == 0);
        self.set_flag("N", self.register_x.bit(7));
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    pub fn txs(&mut self) {
        self.register_s = self.register_x;
    }

    pub fn tya(&mut self) {
        self.register_a = self.register_y;
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    pub fn st_reg(&mut self, reg: &str, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        match reg {
            "a" => {
                self.mem_write(addr, self.register_a);
                self.register_a
            }
            "x" => {
                self.mem_write(addr, self.register_x);
                self.register_x
            }
            "y" => {
                self.mem_write(addr, self.register_y);
                self.register_y
            }
            _ => panic!("Unknown Register: {}", reg),
        };
    }

    pub fn sta(&mut self, mode: AddressingMode) {
        self.st_reg("a", mode);
    }

    pub fn stx(&mut self, mode: AddressingMode) {
        self.st_reg("x", mode);
    }

    pub fn sty(&mut self, mode: AddressingMode) {
        self.st_reg("y", mode);
    }

    /// ************** INC instructions **************
    ///
    pub fn inc_reg(&mut self, reg: &str) {
        let reg = match reg {
            "a" => {
                self.register_a = self.register_a.wrapping_add(1);
                self.register_a
            }
            "x" => {
                self.register_x = self.register_x.wrapping_add(1);
                self.register_x
            }
            "y" => {
                self.register_y = self.register_y.wrapping_add(1);
                self.register_y
            }
            _ => panic!("Unknown Register: {}", reg),
        };

        self.set_flag("Z", reg == 0);
        self.set_flag("N", reg.bit(7));
    }

    pub fn ina(&mut self) {
        self.inc_reg("a");
    }

    pub fn inx(&mut self) {
        self.inc_reg("x");
    }

    pub fn iny(&mut self) {
        self.inc_reg("y");
    }

    /// ************** Status Flags instructions **************
    ///
    pub fn clear_carry_flag(&mut self) {
        self.set_flag("C", false);
    }

    pub fn set_carry_flag(&mut self) {
        self.set_flag("C", true);
    }

    pub fn clear_decimal_flag(&mut self) {
        self.set_flag("D", false);
    }

    pub fn set_decimal_flag(&mut self) {
        self.set_flag("D", true);
    }

    pub fn clear_interrupt_flag(&mut self) {
        self.set_flag("I", false);
    }

    pub fn set_interrupt_flag(&mut self) {
        self.set_flag("I", true);
    }

    pub fn clear_overflow_flag(&mut self) {
        self.set_flag("V", false);
    }

    /// ************** Stacks instructions **************
    ///
    pub fn push(&mut self, val: u8) {
        self.mem_write(self.register_s as u16, val);
        self.register_s -= 1;
    }

    pub fn pull(&mut self) -> u8 {
        let val = self.mem_read(self.register_s as u16);
        self.register_s += 1;
        val
    }

    pub fn pha(&mut self) {
        self.push(self.register_a);
    }

    pub fn php(&mut self) {
        self.push(self.status);
    }

    pub fn pla(&mut self) {
        self.register_a = self.pull();
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    pub fn plp(&mut self) {
        self.status = self.pull();
    }
}
