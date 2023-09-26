use core::fmt;
use std::rc::Rc;

use bit::BitIndex;

#[derive(Clone)]
pub struct Opcode {
    code: u8,
    name: String,
    bytes: u8,
    cycles: u8,
    mode: AddressingMode,
}

impl Opcode {
    pub fn new(code: u8, name: String, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        Opcode {
            code,
            name,
            bytes,
            cycles,
            mode,
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Code: {}\nName: {}\nBytes: {}\nCycles: {}\nMode: {}",
            self.code,
            self.name,
            self.bytes,
            self.cycles,
            match self.mode {
                AddressingMode::Immediate => "IMM",
                AddressingMode::ZeroPage => "ZP",
                AddressingMode::ZeroPage_X => "ZPX",
                AddressingMode::ZeroPage_Y => "ZPY",
                AddressingMode::Absolute => "A",
                AddressingMode::Absolute_X => "AX",
                AddressingMode::Absolute_Y => "AY",
                AddressingMode::Indirect_X => "IX",
                AddressingMode::Indirect_Y => "IY",
                AddressingMode::NoneAddressing => "IMP",
            }
        )
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
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
            status: 0,
            pc: 0,
            cycles: 0,
            memory: [0; 0xFFFF],
            #[rustfmt::skip]
            opcodes: {
                vec![
                    // BRK
                    Opcode::new( 0x00, String::from("BRK"), 1, 7, AddressingMode::NoneAddressing),

                    // TAX
                    Opcode::new( 0xAA, String::from("TAX"), 1, 2, AddressingMode::NoneAddressing),

                    //TAY
                    Opcode::new( 0xA8, String::from("TAY"), 1, 2, AddressingMode::NoneAddressing),

                    // INX
                    Opcode::new( 0xE8, String::from("INX"), 1, 2, AddressingMode::NoneAddressing),

                    // INY
                    Opcode::new( 0xC8, String::from("INY"), 1, 2, AddressingMode::NoneAddressing),

                    // LDA
                    Opcode::new(0xA9, String::from("LDA"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA5, String::from("LDA"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB5, String::from("LDA"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0xAD, String::from("LDA"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBD, String::from("LDA"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0xB9, String::from("LDA"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0xA1, String::from("LDA"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0xB1, String::from("LDA"), 2, 5, AddressingMode::Indirect_Y),

                    // LDX
                    Opcode::new(0xA2, String::from("LDX"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA6, String::from("LDX"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB6, String::from("LDX"), 2, 4, AddressingMode::ZeroPage_Y),
                    Opcode::new(0xAE, String::from("LDX"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBE, String::from("LDX"), 3, 4, AddressingMode::Absolute_Y),

                    // LDY
                    Opcode::new(0xA0, String::from("LDY"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xA4, String::from("LDY"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xB4, String::from("LDY"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0xAC, String::from("LDY"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xBC, String::from("LDY"), 3, 4, AddressingMode::Absolute_X),

                    // STA
                    Opcode::new(0x85, String::from("STA"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x95, String::from("STA"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x8D, String::from("STA"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0x9D, String::from("STA"), 3, 5, AddressingMode::Absolute_X),
                    Opcode::new(0x99, String::from("STA"), 3, 5, AddressingMode::Absolute_Y),
                    Opcode::new(0x81, String::from("STA"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0x91, String::from("STA"), 2, 6, AddressingMode::Indirect_Y),
                ]
            },
        }
    }

    pub fn set_z_flag(&mut self, val: bool) {
        self.status.set_bit(1, val);
    }

    pub fn set_neg_flag(&mut self, val: bool) {
        self.status.set_bit(7, val);
    }

    pub fn get_bit_at(self, i: u32) -> bool {
        self.status.bit(i as usize)
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

                // TAX
                0xAA => self.tax(),

                //     // INX
                0xE8 => self.inx(),

                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode.mode),

                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode.mode),

                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(opcode.mode),

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

    /// ************** BRK instructions **************
    ///
    pub fn brk(&mut self) {
        self.status.set_bit(2, true);
        self.cycles += 7;
        return;
    }

    /// ************** LD instructions **************
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

        self.set_z_flag(reg == 0);
        self.set_neg_flag(reg.bit(7));
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

    /// ************** TAX instruction **************
    ///
    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_z_flag(self.register_x == 0);
        self.set_neg_flag(self.register_x.bit(7));
        self.cycles += 2;
    }

    /// ************** TAY instruction **************
    ///
    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.set_z_flag(self.register_y == 0);
        self.set_neg_flag(self.register_y.bit(7));
        self.cycles += 2;
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
        self.set_z_flag(reg == 0);
        self.set_neg_flag(reg.bit(7));
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

    /// ************** ST instructions **************
    ///
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
}
