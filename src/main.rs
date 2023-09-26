use cpu::Cpu;

mod cpu;
mod opcode;

fn main() {
    let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut cpu = Cpu::new();
    cpu.load_and_run(&program);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tax() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(&vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_tay() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(&vec![0xa9, 0xfe, 0xa8, 0x00]);
        assert_eq!(cpu.register_y, 0xfe);
    }

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(&vec![0xa2, 0xff, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(&vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }
}
