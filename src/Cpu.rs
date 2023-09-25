use bit::BitIndex;
use std::process;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub pc: u16,
    cycles: u16,
    memory: [u8; 0xFFFF],
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

    pub fn mem_write(&mut self, val: u8, addr: u16) {
        self.memory[addr as usize] = val;
    }

    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x8000;
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.pc as usize];
            self.pc += 1;

            match opcode {
                // BRK
                0x00 => {
                    self.status.set_bit(2, true);
                    self.cycles += 7;
                    process::exit(1);
                }
                // LDA
                0xA9 => self.lda(self.memory[self.pc as usize]),

                // TAX
                0xAA => self.tax(),

                // INX
                0xE8 => self.inx(),

                _ => todo!("Opcode {:#04X} not implemented yet!", opcode),
            }

            // println!("{:#?}", self)
        }
    }

    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load(program);
        self.run();
    }

    pub fn lda(&mut self, param: u8) {
        self.register_a = param;
        self.pc += 1;
        self.set_z_flag(self.register_a == 0);
        self.set_neg_flag(self.register_a.bit(7));
        self.cycles += 2;
    }

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_z_flag(self.register_x == 0);
        self.set_neg_flag(self.register_x.bit(7));
        self.cycles += 2;
    }

    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.set_z_flag(self.register_x == 0);
        self.set_neg_flag(self.register_x.bit(7));
    }
}
