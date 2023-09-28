use super::*;

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

// INC
#[test]
fn test_inc_zero_page() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x42, 0x7F); // Set the value at address 0x42 to 0x7F
    cpu.load_and_run(&vec![0xE6, 0x42]);
    assert_eq!(cpu.mem_read(0x42), 0x80);
}

#[test]
fn test_inc_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xF6, 0x02]);
    cpu.mem_write(0x04, 0x7D);
    cpu.register_x = 0x02;
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.mem_read(0x04), 0x7E);
}

#[test]
fn test_inc_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xEE, 0x00, 0x80]);
    cpu.mem_write(0x8000, 0xEE); // Set the value at address 0x8000 to 0xEE
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.mem_read(0x8000), 0xEF); // Check the value after execution
}

#[test]
fn test_inc_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x02;
    cpu.mem_write(0x8002, 0x7F); // Set the value at address 0x8002 to 0x7F
    cpu.load_and_run(&vec![0xFE, 0x00, 0x80]);
    assert_eq!(cpu.mem_read(0x8002), 0x80);
}

#[test]
fn test_inc_zero_page_overflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xE6, 0x02]); // INC instruction with zero page indexed addressing mode
    cpu.mem_write(0x02, 0xFF); // Set the value at address 0x02 to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INC instruction
    assert_eq!(cpu.mem_read(0x02), 0x00); // Check the value after execution (should wrap around to 0x00)
    assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
    assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
    assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
    assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
}

#[test]
fn test_inc_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0xFF; // Set X register to a large value (will wrap around to 0x02)
    cpu.load(&vec![0xF6, 0x02]); // INC instruction with zero page indexed addressing mode
    cpu.mem_write(0x01, 0xFF); // Set the value at address 0x04 to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INC instruction
    assert_eq!(cpu.mem_read(0x01), 0x00); // Check the value after execution (should wrap around to 0x00)
    assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
    assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
    assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
    assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
}

#[test]
fn test_inc_absolute_overflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xEE, 0x03, 0x80]); // INC instruction with absolute addressing mode
    cpu.mem_write(0x8003, 0xFF); // Set the value at address 0x8000 to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INC instruction
    assert_eq!(cpu.mem_read(0x8003), 0x00); // Check the value after execution (should wrap around to 0x00)
    assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
    assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
    assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
    assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
}

#[test]
fn test_inc_absolute_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x02; // Set X register
    cpu.load(&vec![0xFE, 0x03, 0x80]); // INC instruction with absolute indexed addressing mode
    cpu.mem_write(0x8005, 0xFF); // Set the value at address 0x8002 to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INC instruction
    assert_eq!(cpu.mem_read(0x8005), 0x00); // Check the value after execution (should wrap around to 0x00)
    assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
    assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
    assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
    assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
}
