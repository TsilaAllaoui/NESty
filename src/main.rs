use nesty::cpu::Cpu;
fn main() {
    let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut cpu = Cpu::new();
    cpu.interpret(&program);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = Cpu::new();
        cpu.interpret(&vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(&vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.register_a = 10;
        cpu.interpret(&vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }
}
