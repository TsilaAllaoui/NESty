use crate::cpu::AddressingMode;
use core::fmt;

#[derive(Clone)]
pub struct Opcode {
    pub code: u8,
    pub name: String,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
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
