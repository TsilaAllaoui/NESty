use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub register_s: u8,
    pub status: u8,
    pub pc: u16,
    pub opcodes: Vec<Opcode>,
    pub cycles: u16,
    pub memory: [u8; 0xFFFF],
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

                    // Decrements & Increments
                    Opcode::new(0xE6, String::from("INC"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0xF6, String::from("INC"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0xEE, String::from("INC"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0xFE, String::from("INC"), 3, 7, AddressingMode::Absolute_X),

                    Opcode::new(0xC6, String::from("DEC"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0xD6, String::from("DEC"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0xCE, String::from("DEC"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0xDE, String::from("DEC"), 3, 7, AddressingMode::Absolute_X),

                    Opcode::new( 0xE8, String::from("INX"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0xC8, String::from("INY"), 1, 2, AddressingMode::NoneAddressing),

                    Opcode::new( 0xCA, String::from("DEX"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new( 0x88, String::from("DEY"), 1, 2, AddressingMode::NoneAddressing),

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

                    // Logical Operations
                    Opcode::new(0x29, String::from("AND"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0x25, String::from("AND"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x35, String::from("AND"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x2D, String::from("AND"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0x3D, String::from("AND"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0x39, String::from("AND"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0x21, String::from("AND"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0x31, String::from("AND"), 2, 5, AddressingMode::Indirect_Y),

                    Opcode::new(0x49, String::from("EOR"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0x45, String::from("EOR"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x55, String::from("EOR"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x4D, String::from("EOR"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0x5D, String::from("EOR"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0x59, String::from("EOR"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0x41, String::from("EOR"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0x51, String::from("EOR"), 2, 5, AddressingMode::Indirect_Y),

                    Opcode::new(0x09, String::from("ORA"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0x05, String::from("ORA"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0x15, String::from("ORA"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0x0D, String::from("ORA"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0x1D, String::from("ORA"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0x19, String::from("ORA"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0x01, String::from("ORA"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0x11, String::from("ORA"), 2, 5, AddressingMode::Indirect_Y),

                    // Shift & Rotate Instructions
                    Opcode::new(0x0A, String::from("ASL"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x06, String::from("ASL"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0x16, String::from("ASL"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0x0E, String::from("ASL"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0x1E, String::from("ASL"), 3, 7, AddressingMode::Absolute_X),

                    Opcode::new(0x4A, String::from("LSR"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x46, String::from("LSR"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0x56, String::from("LSR"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0x4E, String::from("LSR"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0x5E, String::from("LSR"), 3, 7, AddressingMode::Absolute_X),

                    Opcode::new(0x2A, String::from("ROL"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x26, String::from("ROL"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0x36, String::from("ROL"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0x2E, String::from("ROL"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0x3E, String::from("ROL"), 3, 7, AddressingMode::Absolute_X),

                    Opcode::new(0x6A, String::from("ROR"), 1, 2, AddressingMode::NoneAddressing),
                    Opcode::new(0x66, String::from("ROR"), 2, 5, AddressingMode::ZeroPage),
                    Opcode::new(0x76, String::from("ROR"), 2, 6, AddressingMode::ZeroPage_X),
                    Opcode::new(0x6E, String::from("ROR"), 3, 6, AddressingMode::Absolute),
                    Opcode::new(0x7E, String::from("ROR"), 3, 7, AddressingMode::Absolute_X),

                    // Comparisons instructions
                    Opcode::new(0xC9, String::from("CMP"), 2, 2, AddressingMode::Immediate),
                    Opcode::new(0xC5, String::from("CMP"), 2, 3, AddressingMode::ZeroPage),
                    Opcode::new(0xD5, String::from("CMP"), 2, 4, AddressingMode::ZeroPage_X),
                    Opcode::new(0xCD, String::from("CMP"), 3, 4, AddressingMode::Absolute),
                    Opcode::new(0xDD, String::from("CMP"), 3, 4, AddressingMode::Absolute_X),
                    Opcode::new(0xD9, String::from("CMP"), 3, 4, AddressingMode::Absolute_Y),
                    Opcode::new(0xC1, String::from("CMP"), 2, 6, AddressingMode::Indirect_X),
                    Opcode::new(0xD1, String::from("CMP"), 2, 5, AddressingMode::Indirect_Y),
                ]
            },
        }
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

                // Decrements & Increments
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                0xCA => self.dex(),
                0x88 => self.dey(),
                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(opcode.mode),
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(opcode.mode),

                // Logical Operations
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(opcode.mode),
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(opcode.mode),
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(opcode.mode),

                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(opcode.mode),

                // Status flags Instructions
                0x18 => self.clear_carry_flag(),
                0xD8 => self.clear_decimal_flag(),
                0x58 => self.clear_interrupt_flag(),
                0xB8 => self.clear_overflow_flag(),
                0x38 => self.set_carry_flag(),
                0xF8 => self.set_decimal_flag(),
                0x78 => self.set_interrupt_flag(),

                // Shift & Rotate Instructions
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(opcode.mode),
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(opcode.mode),
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(opcode.mode),
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(opcode.mode),

                // Comparisons Instructions
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(opcode.mode),

                _ => {
                    self.brk();
                    break;
                }
            }
        }
    }
}
