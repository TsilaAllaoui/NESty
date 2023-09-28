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
    cpu.mem_write(0x8003, 0xFF); // Set the value at address 0x8003 to maximum value 0xFF
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
    cpu.mem_write(0x8005, 0xFF); // Set the value at address 0x8005 to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INC instruction
    assert_eq!(cpu.mem_read(0x8005), 0x00); // Check the value after execution (should wrap around to 0x00)
    assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
    assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
    assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
    assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
}

// DEC

#[test]
fn test_dec_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC6, 0x10]); // DEC instruction with zero page addressing mode
    cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x10), 0x7E); // Check the value after execution
}

#[test]
fn test_dec_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xD6, 0x10]); // DEC instruction with zero page indexed addressing mode
    cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x12), 0x7E); // Check the value after execution
}

#[test]
fn test_dec_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xCE, 0x00, 0x90]); // DEC instruction with absolute addressing mode
    cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x9000), 0x7E); // Check the value after execution
}

#[test]
fn test_dec_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xDE, 0x00, 0x90]); // DEC instruction with absolute indexed addressing mode
    cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x9002), 0x7E); // Check the value after execution
}

#[test]
fn test_dec_zero_page_underflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC6, 0x10]); // DEC instruction with zero page addressing mode
    cpu.mem_write(0x10, 0x00); // Set the value at address 0x10 to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value after execution (should wrap around to 0xFF)
}

#[test]
fn test_dec_absolute_underflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xCE, 0x00, 0x90]); // DEC instruction with absolute addressing mode
    cpu.mem_write(0x9000, 0x00); // Set the value at address 0x9000 to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEC instruction
    assert_eq!(cpu.mem_read(0x9000), 0xFF); // Check the value after execution (should wrap around to 0xFF)
}

// INX, DEX, INY, DEY

#[test]
fn test_inx() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xE8]); // INX instruction
    cpu.register_x = 0x7F; // Set X register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INX instruction
    assert_eq!(cpu.register_x, 0x80); // Check the value after execution

    // Edge Case: Incrementing 0xFF should wrap around to 0x00
    cpu.load(&vec![0xE8]); // INX instruction
    cpu.register_x = 0xFF; // Set X register to 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INX instruction
    assert_eq!(cpu.register_x, 0x00); // Check the value after execution
}

#[test]
fn test_iny() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC8]); // INY instruction
    cpu.register_y = 0x7F; // Set Y register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INY instruction
    assert_eq!(cpu.register_y, 0x80); // Check the value after execution

    // Edge Case: Incrementing 0xFF should wrap around to 0x00
    cpu.load(&vec![0xC8]); // INY instruction
    cpu.register_y = 0xFF; // Set Y register to 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the INY instruction
    assert_eq!(cpu.register_y, 0x00); // Check the value after execution
}

#[test]
fn test_dex() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xCA]); // DEX instruction
    cpu.register_x = 0x80; // Set X register to 0x80
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value after execution

    // Edge Case: Decrementing 0x00 should wrap around to 0xFF
    cpu.load(&vec![0xCA]); // DEX instruction
    cpu.register_x = 0x00; // Set X register to 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEX instruction
    assert_eq!(cpu.register_x, 0xFF); // Check the value after execution
}

#[test]
fn test_dey() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x88]); // DEY instruction
    cpu.register_y = 0x80; // Set Y register to 0x80
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value after execution

    // Edge Case: Decrementing 0x00 should wrap around to 0xFF
    cpu.load(&vec![0x88]); // DEY instruction
    cpu.register_y = 0x00; // Set Y register to 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the DEY instruction
    assert_eq!(cpu.register_y, 0xFF); // Check the value after execution
}

// LDX

#[test]
fn test_ldx_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA2, 0x42]); // LDX instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x42); // Check the value after execution
}

#[test]
fn test_ldx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA6, 0x10]); // LDX instruction with zero page addressing mode
    cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
}

#[test]
fn test_ldx_zero_page_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xB6, 0x10]); // LDX instruction with zero page indexed addressing mode (Y register)
    cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
    cpu.register_y = 0x02; // Set Y register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
}

#[test]
fn test_ldx_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xAE, 0x00, 0x90]); // LDX instruction with absolute addressing mode
    cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
}

#[test]
fn test_ldx_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xBE, 0x00, 0x90]); // LDX instruction with absolute indexed addressing mode (Y register)
    cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
    cpu.register_y = 0x02; // Set Y register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
}

#[test]
fn test_ldx_edge_cases() {
    let mut cpu = Cpu::new();

    // Edge Case 1: Loading 0x00
    cpu.load(&vec![0xA2, 0x00]); // LDX instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0x00); // Check the value after execution

    // Edge Case 2: Loading 0xFF
    cpu.load(&vec![0xA2, 0xFF]); // LDX instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDX instruction
    assert_eq!(cpu.register_x, 0xFF); // Check the value after execution
}

// LDY

#[test]
fn test_ldy_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA0, 0x42]); // LDY instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x42); // Check the value after execution
}

#[test]
fn test_ldy_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA4, 0x10]); // LDY instruction with zero page addressing mode
    cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
}

#[test]
fn test_ldy_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xB4, 0x10]); // LDY instruction with zero page indexed addressing mode (X register)
    cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
}

#[test]
fn test_ldy_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xAC, 0x00, 0x90]); // LDY instruction with absolute addressing mode
    cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
}

#[test]
fn test_ldy_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xBC, 0x00, 0x90]); // LDY instruction with absolute indexed addressing mode (X register)
    cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
}

#[test]
fn test_ldy_edge_cases() {
    let mut cpu = Cpu::new();

    // Edge Case 1: Loading 0x00
    cpu.load(&vec![0xA0, 0x00]); // LDY instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0x00); // Check the value after execution

    // Edge Case 2: Loading 0xFF
    cpu.load(&vec![0xA0, 0xFF]); // LDY instruction with immediate addressing mode
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the LDY instruction
    assert_eq!(cpu.register_y, 0xFF); // Check the value after execution
}

// STA

#[test]
fn test_sta_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x85, 0x10]); // STA instruction with zero page addressing mode
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
}

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x95, 0x10]); // STA instruction with zero page indexed addressing mode (X register)
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
}

#[test]
fn test_sta_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x8D, 0x00, 0x90]); // STA instruction with absolute addressing mode
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
}

#[test]
fn test_sta_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x9D, 0x00, 0x90]); // STA instruction with absolute indexed addressing mode (X register)
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x9002), 0x7F); // Check the value at address 0x9002 after execution
}

#[test]
fn test_sta_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x99, 0x00, 0x90]); // STA instruction with absolute indexed addressing mode (Y register)
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.register_y = 0x02; // Set Y register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x9002), 0x7F); // Check the value at address 0x9002 after execution
}

#[test]
fn test_sta_zero_page_underflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x85, 0x10]); // STA instruction with zero page addressing mode
    cpu.register_a = 0x00; // Set A register to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution
}

#[test]
fn test_sta_absolute_underflow() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x8D, 0x00, 0x90]); // STA instruction with absolute addressing mode
    cpu.register_a = 0x00; // Set A register to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STA instruction
    assert_eq!(cpu.mem_read(0x9000), 0x00); // Check the value at address 0x9000 after execution
}

// STX

#[test]
fn test_stx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
    cpu.register_x = 0x7F; // Set X register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STX instruction
    assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x96, 0x10]); // STX instruction with zero page indexed addressing mode (Y register)
    cpu.register_x = 0x7F; // Set X register to 0x7F
    cpu.register_y = 0x02; // Set Y register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STX instruction
    assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
}

#[test]
fn test_stx_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x8E, 0x00, 0x90]); // STX instruction with absolute addressing mode
    cpu.register_x = 0x7F; // Set X register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STX instruction
    assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
}

#[test]
fn test_stx_edge_cases() {
    let mut cpu = Cpu::new();

    // Edge Case 1: Storing 0x00
    cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
    cpu.register_x = 0x00; // Set X register to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STX instruction
    assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution

    // Edge Case 2: Storing 0xFF
    cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
    cpu.register_x = 0xFF; // Set X register to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STX instruction
    assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value at address 0x10 after execution
}

// STY

#[test]
fn test_sty_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
    cpu.register_y = 0x7F; // Set Y register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STY instruction
    assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
}

#[test]
fn test_sty_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x94, 0x10]); // STY instruction with zero page indexed addressing mode (X register)
    cpu.register_y = 0x7F; // Set Y register to 0x7F
    cpu.register_x = 0x02; // Set X register
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STY instruction
    assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
}

#[test]
fn test_sty_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x8C, 0x00, 0x90]); // STY instruction with absolute addressing mode
    cpu.register_y = 0x7F; // Set Y register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STY instruction
    assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
}

#[test]
fn test_sty_edge_cases() {
    let mut cpu = Cpu::new();

    // Edge Case 1: Storing 0x00
    cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
    cpu.register_y = 0x00; // Set Y register to minimum value 0x00
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STY instruction
    assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution

    // Edge Case 2: Storing 0xFF
    cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
    cpu.register_y = 0xFF; // Set Y register to maximum value 0xFF
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the STY instruction
    assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value at address 0x10 after execution
}

// TAX, TAY, TYA

#[test]
fn test_tax() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xAA]); // TAX instruction
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the TAX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value of X register after execution
}

#[test]
fn test_tay() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA8]); // TAY instruction
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the TAY instruction
    assert_eq!(cpu.register_y, 0x7F); // Check the value of Y register after execution
}

#[test]
fn test_tsx() {
    let mut cpu = Cpu::new();
    cpu.register_s = 0xFC;
    cpu.load(&vec![0xBA]); // TSX instruction
    cpu.push(0x7F); // Push 0x7F onto the stack
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the TSX instruction
    assert_eq!(cpu.register_x, 0x7F); // Check the value of X register after execution
}

// PHA, PHP, PLA, PLP

#[test]
fn test_pha() {
    let mut cpu = Cpu::new();
    cpu.register_s = 0xFC;
    cpu.load(&vec![0x48]); // PHA instruction
    cpu.register_a = 0x7F; // Set A register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the PHA instruction
    assert_eq!(cpu.pull(), 0x7F); // Check the value on the stack after execution
}

#[test]
fn test_php() {
    let mut cpu = Cpu::new();
    cpu.register_s = 0xFC;
    cpu.load(&vec![0x08]); // PHP instruction
    cpu.status = 0x7F; // Set status register to 0x7F
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the PHP instruction
    assert_eq!(cpu.pull(), 0x7F); // Check the value on the stack after execution
}

#[test]
fn test_pla() {
    let mut cpu = Cpu::new();
    cpu.register_s = 0xFC;
    cpu.load(&vec![0x68]); // PLA instruction
    cpu.push(0x7F); // Push 0x7F onto the stack
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the PLA instruction
    assert_eq!(cpu.register_a, 0x7F); // Check the value in the A register after execution
}

#[test]
fn test_plp() {
    let mut cpu = Cpu::new();
    cpu.register_s = 0xFC;
    cpu.load(&vec![0x28]); // PLP instruction
    cpu.push(0x7F); // Push 0x7F onto the stack
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the PLP instruction
    assert_eq!(cpu.status, 0x7F); // Check the value in the status register after execution
}

// CLC, CLD, CLI, CLV, SEC, SED, and SEI

#[test]
fn test_clc() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x18]); // CLC instruction
    cpu.status = 0x01; // Set Carry flag to 1 (true)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the CLC instruction
    assert_eq!(cpu.get_flag("C"), false); // Check if Carry flag is cleared (false)
}

#[test]
fn test_cld() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xD8]); // CLD instruction
    cpu.status = 0x08; // Set Decimal mode flag to 1 (true)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the CLD instruction
    assert_eq!(cpu.get_flag("D"), false); // Check if Decimal mode flag is cleared (false)
}

#[test]
fn test_cli() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x58]); // CLI instruction
    cpu.status = 0x04; // Set Interrupt disable flag to 1 (true)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the CLI instruction
    let val = cpu.get_flag("I");
    assert_eq!(val, false); // Check if Interrupt disable flag is cleared (false)
}

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xB8]); // CLV instruction
    cpu.status = 0x40; // Set Overflow flag to 1 (true)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the CLV instruction
    assert_eq!(cpu.get_flag("V"), false); // Check if Overflow flag is cleared (false)
}

#[test]
fn test_sec() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x38]); // SEC instruction
    cpu.status = 0x00; // Clear Carry flag (false)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the SEC instruction
    assert_eq!(cpu.get_flag("C"), true); // Check if Carry flag is set (true)
}

#[test]
fn test_sed() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xF8]); // SED instruction
    cpu.status = 0x00; // Clear Decimal mode flag (false)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the SED instruction
    assert_eq!(cpu.get_flag("D"), true); // Check if Decimal mode flag is set (true)
}

#[test]
fn test_sei() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x78]); // SEI instruction
    cpu.status = 0x00; // Clear Interrupt disable flag (false)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the SEI instruction
    assert_eq!(cpu.get_flag("I"), true); // Check if Interrupt disable flag is set (true)
}

//AND

#[test]
fn test_and_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA9, 0x0F, 0x29, 0x07]); // LDA, AND with immediate values
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0x07); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA5, 0x42, 0x25, 0x42]); // LDA, AND with zero page addressing mode
    cpu.mem_write(0x42, 0b11001100); // Set value at address 0x42
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0b11001100); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA2, 0x02, 0xA9, 0b11110000, 0x95, 0x00]); // LDX, LDA, AND with zero page indexed addressing mode (X register)
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0b11110000); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xAD, 0x00, 0x90, 0x2D, 0x00, 0x90]); // LDA, AND with absolute addressing mode
    cpu.mem_write(0x9000, 0b10101010); // Set value at address 0x9000
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0b10101010); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA2, 0x02, 0xBD, 0x00, 0x02]); // LDX, LDA, AND with absolute indexed addressing mode (X register)
    cpu.mem_write(0x0202, 0b11001100); // Set value at address 0x0202
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0b11001100); // Check if A register has correct value after AND operation
}

// Edge Case: Zero result
#[test]
fn test_and_zero_result() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA9, 0b11001100, 0x29, 0b00110011]); // LDA, AND with immediate values
    cpu.pc = 0x8000; // Set program counter to 0x8000
    cpu.run(); // Execute the instructions
    assert_eq!(cpu.register_a, 0b00000000); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA2, 0x01, 0x86, 0x01, 0xA9, 0xCC, 0x21, 0x00]); // LDX, STX, LDA, AND with indirect X addressing mode
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0); // Check if A register has correct value after AND operation
}

#[test]
fn test_and_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x0000, 0x01);
    cpu.load(&vec![
        0xA0, 0x02, 0x86, 0x02, 0x85, 0x03, 0xA9, 0b00110011, 0x21, 0x00,
    ]); // LDY, STX, STY, LDA, AND with indirect Y addressing mode
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0b00000000); // Check if A register has correct value after AND operation
}

// EOR

#[test]
fn test_eor_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA9, 0b11001100, 0x49, 0b10101010]);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0b01100110);
}

#[test]
fn test_eor_immediate_zero_result() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA9, 0b11001100, 0x49, 0b11001100]);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0b00000000);
}

#[test]
fn test_eor_zero_page() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x02;
    cpu.mem_write(0x10, 0xAB);
    cpu.load(&vec![0xA9, 0b10101010, 0x45, 0x10]);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_eor_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x02;
    cpu.mem_write(0x12, 0xAB);
    cpu.load(&vec![0xA9, 0b10101010, 0x55, 0x10]);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_eor_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xA9, 0b10101010, 0x4D, 0x00, 0x10]);
    cpu.mem_write(0x1000, 0xAB);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x1);
}

#[test]
fn test_eor_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x02;
    cpu.load(&vec![0xA9, 0b10101010, 0x5D, 0x00, 0x10]);
    cpu.mem_write(0x1002, 0xAB);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x1);
}

#[test]
fn test_eor_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.register_y = 0x02;
    cpu.load(&vec![0xA9, 0b10101010, 0x59, 0x00, 0x10]);
    cpu.mem_write(0x1002, 0xAB);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x1);
}

#[test]
fn test_eor_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.register_x = 0x01;
    cpu.register_a = 0b10101010;
    cpu.load(&vec![0x41, 0xF0]);
    cpu.mem_write(0x00F1, 0x00);
    cpu.mem_write(0x00F2, 0x90);
    cpu.mem_write(0x9000, 0xAB);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_eor_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.register_y = 0x01;
    cpu.register_a = 0b10101010;
    cpu.load(&vec![0x51, 0xF0]);
    cpu.mem_write(0x00F0, 0x00);
    cpu.mem_write(0x00F1, 0x90);
    cpu.mem_write(0x9001, 0xAB);
    cpu.pc = 0x8000;
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
}

// ORA

#[test]
fn test_ora_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x09, 0x3C]); // ORA immediate with value 0x3C
    cpu.register_a = 0x24; // Set A register to 0x24
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x3C);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x05, 0x10]); // ORA zero page with address 0x10
    cpu.mem_write(0x0010, 0x2A); // Set value at address 0x10 to 0x2A
    cpu.register_a = 0x05; // Set A register to 0x05
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x2F); // 0x05 | 0x2A = 0x2F
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x15, 0x10]); // ORA zero page,X with address 0x10
    cpu.register_x = 0x03; // Set X register to 0x03
    cpu.mem_write(0x0013, 0x0F); // Set value at address 0x13 (0x10 + 0x03) to 0x0F
    cpu.register_a = 0x21; // Set A register to 0x21
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x2F); // 0x21 | 0x0F = 0x2F
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x0D, 0x34, 0x12]); // ORA absolute with address 0x1234
    cpu.mem_write(0x1234, 0x42); // Set value at address 0x1234 to 0x42
    cpu.register_a = 0x31; // Set A register to 0x31
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x73); // 0x31 | 0x42 = 0x73
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x1D, 0x34, 0x12]); // ORA absolute,X with address 0x1234
    cpu.register_x = 0x02; // Set X register to 0x02
    cpu.mem_write(0x1236, 0x1E); // Set value at address 0x1236 (0x1234 + 0x02) to 0x1E
    cpu.register_a = 0x37; // Set A register to 0x37
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x3F); // 0x37 | 0x1E = 0x3F
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x19, 0x34, 0x12]); // ORA absolute,Y with address 0x1234
    cpu.register_y = 0x01; // Set Y register to 0x01
    cpu.mem_write(0x1235, 0x3C); // Set value at address 0x1235 (0x1234 + 0x01) to 0x3C
    cpu.register_a = 0x27; // Set A register to 0x27
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x3F); // 0x27 | 0x3C = 0x3F
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x01, 0x10]); // ORA (zero page,X) with address 0x10
    cpu.register_x = 0x02; // Set X register to 0x02
    cpu.mem_write(0x0012, 0x2F); // Set value at address 0x12 (0x10 + 0x02) to 0x2F
    cpu.mem_write(0x2F2F, 0x0A); // Set value at address 0x2F2F to 0x0A
    cpu.register_a = 0x1E; // Set A register to 0x1E
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x1E | 0x0A); // 0x1E | 0x0A = 0x1E
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_ora_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x11, 0x10]); // ORA (zero page),Y with address 0x10
    cpu.register_y = 0x01; // Set Y register to 0x01
    cpu.mem_write(0x0010, 0x34); // Set value at address 0x10 to 0x34
    cpu.mem_write(0x0035, 0x07); // Set value at address 0x35 (0x34 + 0x01) to 0x07
    cpu.register_a = 0x3A; // Set A register to 0x3A
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x3A | 0x07); // 0x3A | 0x07 = 0x3F
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

// ASL

#[test]
fn test_asl_accumulator() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x0A]); // ASL accumulator
    cpu.register_a = 0x85; // Set A register to 0x85
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x0A); // 0x85 << 1 = 0x0A
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x06, 0x10]); // ASL zero page with address 0x10
    cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x0010), 0xFE); // 0x7F << 1 = 0xFE
    assert_eq!(cpu.get_flag("C"), false);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), true);
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x16, 0x10]); // ASL zero page,X with address 0x10
    cpu.register_x = 0x03; // Set X register to 0x03
    cpu.mem_write(0x0013, 0x3E); // Set value at address 0x13 (0x10 + 0x03) to 0x3E
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x0013), 0x7C); // 0x3E << 1 = 0x7C
    assert_eq!(cpu.get_flag("C"), false);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x0E, 0x34, 0x12]); // ASL absolute with address 0x1234
    cpu.mem_write(0x1234, 0xB3); // Set value at address 0x1234 to 0xB3
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x1234), 0x66); // 0xB3 << 1 = 0x66
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_asl_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x1E, 0x34, 0x12]); // ASL absolute,X with address 0x1234
    cpu.register_x = 0x02; // Set X register to 0x02
    cpu.mem_write(0x1236, 0x19); // Set value at address 0x1236 (0x1234 + 0x02) to 0x19
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x1236), 0x32); // 0x19 << 1 = 0x32
    assert_eq!(cpu.get_flag("C"), false);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

// LSR

#[test]
fn test_lsr_accumulator() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x4A]); // LSR accumulator
    cpu.register_a = 0x85; // Set A register to 0x85
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.register_a, 0x85 >> 1); // 0x85 >> 1 = 0x42
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_lsr_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x46, 0x10]); // LSR zero page with address 0x10
    cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x0010), 0x3F); // 0x7F >> 1 = 0x3F
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_lsr_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x56, 0x10]); // LSR zero page,X with address 0x10
    cpu.register_x = 0x03; // Set X register to 0x03
    cpu.mem_write(0x0013, 0x3E); // Set value at address 0x13 (0x10 + 0x03) to 0x3E
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x0013), 0x1F); // 0x3E >> 1 = 0x1F
    assert_eq!(cpu.get_flag("C"), false);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_lsr_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x4E, 0x34, 0x12]); // LSR absolute with address 0x1234
    cpu.mem_write(0x1234, 0xB3); // Set value at address 0x1234 to 0xB3
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x1234), 0x59); // 0xB3 >> 1 = 0x59
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_lsr_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0x5E, 0x34, 0x12]); // LSR absolute,X with address 0x1234
    cpu.register_x = 0x02; // Set X register to 0x02
    cpu.mem_write(0x1236, 0x19); // Set value at address 0x1236 (0x1234 + 0x02) to 0x19
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.mem_read(0x1236), 0x0C); // 0x19 >> 1 = 0x0C
    assert_eq!(cpu.get_flag("C"), true);
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}
