use bit::BitIndex;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub pc: u16,
    pub cycles: u16,
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

    pub fn interpret(&mut self, program: &Vec<u8>) {
        self.pc = 0;
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                // BRK
                0x00 => {
                    self.status.set_bit(2, true);
                    self.cycles += 7;
                    return;
                }
                // LDA
                0xA9 => self.lda(program[self.pc as usize]),

                // TAX
                0xAA => self.tax(),

                // INX
                0xE8 => self.inx(),

                _ => todo!("Opcode {:#04X} not implemented yet!", opcode),
            }

            println!("{:#?}", self)
        }
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
        self.register_a += 1;
        self.set_z_flag(self.register_x == 0);
        self.set_neg_flag(self.register_x.bit(7));
    }
}
