pub struct Cpu {
    pub register_a: u8,
    pub status: u8,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            status: 0,
            pc: 0,
        }
    }

    pub fn interpret(&mut self, program: &Vec<u8>) {
        self.pc = 0;
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                _ => todo!("Opcode {:#04x} not implemented yet!", opcode),
            }
        }
    }
}
