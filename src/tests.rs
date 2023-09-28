use super::*;

// LDA
#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xA9, 0x42, 0xA9, 0x21]);
    assert_eq!(cpu.register_a, 0x21);
}

#[test]
fn test_lda_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA5, 0x42]);
    cpu.mem_write(0x42, 0x42);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_lda_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x02, 0x42);
    cpu.load_and_run(&vec![0xB5, 0x02]);
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_lda_absolute() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xAD, 0x03, 0x80, 0xA9, 0x21]);
    assert_eq!(cpu.register_a, 0x21);
}

#[test]
fn test_lda_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xA2, 0x02, 0xBD, 0x00, 0x02, 0xA9, 0x21]);
    assert_eq!(cpu.register_a, 0x21);
}

#[test]
fn test_lda_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xA0, 0x02, 0xBD, 0x00, 0x02, 0xA9, 0x21]);
    assert_eq!(cpu.register_a, 0x21);
}

#[test]
fn test_lda_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0xA2, 0x02, 0xA9, 0x02, 0x85, 0x02, 0xA1, 0x00]);
    assert_eq!(cpu.register_a, 0x02);
}

#[test]
fn test_lda_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![
        0xA0, 0x02, 0xA9, 0x02, 0x85, 0x02, 0xA1, 0x00, 0xA9, 0x21,
    ]);
    assert_eq!(cpu.register_a, 0x21);
}

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

// JMP
#[test]
fn test_jmp_absolute() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0x4C, 0x03, 0x80, 0xa9, 0x21, 0x00]);
    assert_eq!(cpu.pc, 0x8006);
}

#[test]
fn test_jmp_indirect() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(&vec![0x6C, 0x03, 0x80, 0x00, 0x90]);
    assert_eq!(cpu.pc, 0x9001);
}
