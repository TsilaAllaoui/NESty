use bit::BitIndex;

use crate::cpu::{AddressingMode, Cpu};

impl Cpu {
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
                let ptr: u8 = base.wrapping_add(self.register_x);
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
}

impl Cpu {
    /// ************** BRK instructions **************
    ///
    pub fn brk(&mut self) {
        // self.status.set_bit(2, true);
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
        self.register_x = self.pull();
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

    /// ************** Decrements & IncrementsC instructions **************
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

    pub fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, val);
        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    pub fn inx(&mut self) {
        self.inc_reg("x");
    }

    pub fn iny(&mut self) {
        self.inc_reg("y");
    }

    pub fn dec_reg(&mut self, reg: &str) {
        let reg = match reg {
            "a" => {
                self.register_a = self.register_a.wrapping_sub(1);
                self.register_a
            }
            "x" => {
                self.register_x = self.register_x.wrapping_sub(1);
                self.register_x
            }
            "y" => {
                self.register_y = self.register_y.wrapping_sub(1);
                self.register_y
            }
            _ => panic!("Unknown Register: {}", reg),
        };

        self.set_flag("Z", reg == 0);
        self.set_flag("N", reg.bit(7));
    }

    pub fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, val);
        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    pub fn dex(&mut self) {
        self.dec_reg("x");
    }

    pub fn dey(&mut self) {
        self.dec_reg("y");
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
        self.register_s += 1;
        let val = self.mem_read(self.register_s as u16);
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

    /// ************** Logical instructions **************
    ///
    pub fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a &= self.mem_read(addr);
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    pub fn eor(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a ^= self.mem_read(addr);
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    pub fn ora(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a |= self.mem_read(addr);
        self.set_flag("Z", self.register_a == 0);
        self.set_flag("N", self.register_a.bit(7));
    }

    /// ************** Shift & Rotate Instructions **************
    ///
    pub fn asl(&mut self, mode: AddressingMode) {
        let val = match mode {
            AddressingMode::NoneAddressing => {
                self.set_flag("C", self.register_a.bit(7));
                self.register_a = self.register_a << 1;
                self.register_a
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let mut mem_val = self.mem_read(addr);
                self.set_flag("C", mem_val.bit(7));
                mem_val = mem_val << 1;
                self.mem_write(addr, mem_val);
                mem_val
            }
        };

        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    pub fn lsr(&mut self, mode: AddressingMode) {
        let val = match mode {
            AddressingMode::NoneAddressing => {
                self.set_flag("C", self.register_a.bit(0));
                self.register_a = self.register_a >> 1;
                self.register_a
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let mut mem_val = self.mem_read(addr);
                self.set_flag("C", mem_val.bit(0));
                mem_val = mem_val >> 1;
                self.mem_write(addr, mem_val);
                mem_val
            }
        };

        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    pub fn rol(&mut self, mode: AddressingMode) {
        let old_c_flag = self.get_flag("C");
        let val = match mode {
            AddressingMode::NoneAddressing => {
                let old_bit_7 = self.register_a.bit(7);
                self.register_a = self.register_a << 1;
                self.register_a.set_bit(0, old_c_flag);
                self.set_flag("C", old_bit_7);
                self.register_a
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let mut mem_val = self.mem_read(addr);
                let old_bit_7 = mem_val.bit(7);
                mem_val = mem_val << 1;
                mem_val.set_bit(0, old_c_flag);
                self.set_flag("C", old_bit_7);
                self.mem_write(addr, mem_val);
                mem_val
            }
        };

        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    pub fn ror(&mut self, mode: AddressingMode) {
        let old_c_flag = self.get_flag("C");
        let val = match mode {
            AddressingMode::NoneAddressing => {
                let old_bit_0 = self.register_a.bit(0);
                self.register_a = self.register_a >> 1;
                self.register_a.set_bit(7, old_c_flag);
                self.set_flag("C", old_bit_0);
                self.register_a
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let mut mem_val = self.mem_read(addr);
                let old_bit_0 = mem_val.bit(0);
                mem_val = mem_val >> 1;
                mem_val.set_bit(7, old_c_flag);
                self.set_flag("C", old_bit_0);
                self.mem_write(addr, mem_val);
                mem_val
            }
        };

        self.set_flag("Z", val == 0);
        self.set_flag("N", val.bit(7));
    }

    /// ************** Comparisons Instructions **************
    ///
    pub fn cp_reg(&mut self, reg: &str, mode: AddressingMode) {
        let reg = match reg {
            "a" => self.register_a,
            "x" => self.register_x,
            "y" => self.register_y,
            _ => panic!("Unknown register: {}", reg),
        };

        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        if reg >= val {
            if reg == val {
                self.set_flag("Z", true);
            }
            self.set_flag("C", true);
        }
        self.set_flag("N", reg.bit(7));
    }

    pub fn cmp(&mut self, mode: AddressingMode) {
        self.cp_reg("a", mode);
    }

    pub fn cpx(&mut self, mode: AddressingMode) {
        self.cp_reg("x", mode);
    }

    pub fn cpy(&mut self, mode: AddressingMode) {
        self.cp_reg("y", mode);
    }

    /// ************** Conditional Branch Instructions **************
    ///
    pub fn branch(&mut self, flag: &str, condition: bool) {
        let addr = self.get_operand_address(AddressingMode::Immediate);
        let val = self.mem_read(addr) as i8;
        if self.get_flag(flag) == condition {
            if val >= 0 {
                self.pc += val as u16;
            } else {
                self.pc = self.pc.wrapping_add_signed(val as i16);
            }
            self.cycles += 1;
        }
    }

    pub fn bcc(&mut self) {
        self.branch("C", false);
    }

    pub fn bcs(&mut self) {
        self.branch("C", true);
    }

    pub fn beq(&mut self) {
        self.branch("Z", true);
    }

    pub fn bne(&mut self) {
        self.branch("Z", false);
    }

    pub fn bmi(&mut self) {
        self.branch("N", true);
    }

    pub fn bpl(&mut self) {
        self.branch("N", false);
    }

    pub fn bvc(&mut self) {
        self.branch("V", false);
    }

    pub fn bvs(&mut self) {
        self.branch("V", true);
    }

    /// ************** Jumps & Subroutines Instructions **************
    ///
    pub fn jmp(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Absolute {
            let addr = self.get_operand_address(mode);
            self.pc = addr;
        } else if mode == AddressingMode::NoneAddressing {
            let addr = self.mem_read_16(self.pc);
            self.pc = self.mem_read_16(addr);
            self.cycles += 5;
        }
    }

    pub fn jsr(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.push(self.pc as u8);
        self.pc = self.mem_read(addr) as u16;
    }

    pub fn rts(&mut self) {
        self.pc = self.pull() as u16;
        self.cycles += 6;
    }
}
