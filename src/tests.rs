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

#[test]
fn test_asl() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xa9, 0xc0, 0x0a, 0x00]);
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.status, 0x85);
}

#[test]
fn test_bcc() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x18, 0x90, 0x02, 0x00]);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.pc, 0x8006);
}

#[test]
fn test_bcc_backward_jump() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x00, 0x18, 0x90, 0xFC, 0x00]);
    cpu.pc = 0x8001;
    cpu.run();
    assert_eq!(cpu.pc, 0x8001);
}
