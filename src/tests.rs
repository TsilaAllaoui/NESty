use super::*;

// // JMP
// #[test]
// fn test_jmp_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0x4C, 0x03, 0x80, 0xa9, 0x21, 0x00]);
//     assert_eq!(cpu.pc, 0x8006);
// }

// #[test]
// fn test_jmp_indirect() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0x6C, 0x03, 0x80, 0x00, 0x90]);
//     assert_eq!(cpu.pc, 0x9001);
// }

// // LDA
// #[test]
// fn test_lda_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0xA9, 0x42, 0xA9, 0x21]);
//     assert_eq!(cpu.register_a, 0x21);
// }

// #[test]
// fn test_lda_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA5, 0x42]);
//     cpu.mem_write(0x42, 0x42);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x42);
// }

// #[test]
// fn test_lda_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.mem_write(0x02, 0x42);
//     cpu.load_and_run(&vec![0xB5, 0x02]);
//     assert_eq!(cpu.register_a, 0x42);
// }

// #[test]
// fn test_lda_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0xAD, 0x03, 0x80, 0xA9, 0x21]);
//     assert_eq!(cpu.register_a, 0x21);
// }

// #[test]
// fn test_lda_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0xA2, 0x02, 0xBD, 0x00, 0x02, 0xA9, 0x21]);
//     assert_eq!(cpu.register_a, 0x21);
// }

// #[test]
// fn test_lda_absolute_y() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0xA0, 0x02, 0xBD, 0x00, 0x02, 0xA9, 0x21]);
//     assert_eq!(cpu.register_a, 0x21);
// }

// #[test]
// fn test_lda_indirect_x() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![0xA2, 0x02, 0xA9, 0x02, 0x85, 0x02, 0xA1, 0x00]);
//     assert_eq!(cpu.register_a, 0x02);
// }

// #[test]
// fn test_lda_indirect_y() {
//     let mut cpu = Cpu::new();
//     cpu.load_and_run(&vec![
//         0xA0, 0x02, 0xA9, 0x02, 0x85, 0x02, 0xA1, 0x00, 0xA9, 0x21,
//     ]);
//     assert_eq!(cpu.register_a, 0x21);
// }

// // INC
// #[test]
// fn test_inc_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.mem_write(0x42, 0x7F); // Set the value at address 0x42 to 0x7F
//     cpu.load_and_run(&vec![0xE6, 0x42]);
//     assert_eq!(cpu.mem_read(0x42), 0x80);
// }

// #[test]
// fn test_inc_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xF6, 0x02]);
//     cpu.mem_write(0x04, 0x7D);
//     cpu.register_x = 0x02;
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x04), 0x7E);
// }

// #[test]
// fn test_inc_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xEE, 0x00, 0x80]);
//     cpu.mem_write(0x8000, 0xEE); // Set the value at address 0x8000 to 0xEE
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x8000), 0xEF); // Check the value after execution
// }

// #[test]
// fn test_inc_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x02;
//     cpu.mem_write(0x8002, 0x7F); // Set the value at address 0x8002 to 0x7F
//     cpu.load_and_run(&vec![0xFE, 0x00, 0x80]);
//     assert_eq!(cpu.mem_read(0x8002), 0x80);
// }

// #[test]
// fn test_inc_zero_page_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xE6, 0x02]); // INC instruction with zero page indexed addressing mode
//     cpu.mem_write(0x02, 0xFF); // Set the value at address 0x02 to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INC instruction
//     assert_eq!(cpu.mem_read(0x02), 0x00); // Check the value after execution (should wrap around to 0x00)
//     assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
//     assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
//     assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
//     assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
// }

// #[test]
// fn test_inc_zero_page_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0xFF; // Set X register to a large value (will wrap around to 0x02)
//     cpu.load(&vec![0xF6, 0x02]); // INC instruction with zero page indexed addressing mode
//     cpu.mem_write(0x01, 0xFF); // Set the value at address 0x04 to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INC instruction
//     assert_eq!(cpu.mem_read(0x01), 0x00); // Check the value after execution (should wrap around to 0x00)
//     assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
//     assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
//     assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
//     assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
// }

// #[test]
// fn test_inc_absolute_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xEE, 0x03, 0x80]); // INC instruction with absolute addressing mode
//     cpu.mem_write(0x8003, 0xFF); // Set the value at address 0x8003 to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INC instruction
//     assert_eq!(cpu.mem_read(0x8003), 0x00); // Check the value after execution (should wrap around to 0x00)
//     assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
//     assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
//     assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
//     assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
// }

// #[test]
// fn test_inc_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x02; // Set X register
//     cpu.load(&vec![0xFE, 0x03, 0x80]); // INC instruction with absolute indexed addressing mode
//     cpu.mem_write(0x8005, 0xFF); // Set the value at address 0x8005 to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INC instruction
//     assert_eq!(cpu.mem_read(0x8005), 0x00); // Check the value after execution (should wrap around to 0x00)
//     assert_eq!(cpu.get_flag("C"), false); // Check if carry flag is clear
//     assert_eq!(cpu.get_flag("Z"), true); // Check if zero flag is set
//     assert_eq!(cpu.get_flag("N"), false); // Check if negative flag is clear
//     assert_eq!(cpu.get_flag("V"), false); // Check if overflow flag is clear
// }

// // DEC

// #[test]
// fn test_dec_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xC6, 0x10]); // DEC instruction with zero page addressing mode
//     cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x10), 0x7E); // Check the value after execution
// }

// #[test]
// fn test_dec_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xD6, 0x10]); // DEC instruction with zero page indexed addressing mode
//     cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x12), 0x7E); // Check the value after execution
// }

// #[test]
// fn test_dec_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xCE, 0x00, 0x90]); // DEC instruction with absolute addressing mode
//     cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x9000), 0x7E); // Check the value after execution
// }

// #[test]
// fn test_dec_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xDE, 0x00, 0x90]); // DEC instruction with absolute indexed addressing mode
//     cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x9002), 0x7E); // Check the value after execution
// }

// #[test]
// fn test_dec_zero_page_underflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xC6, 0x10]); // DEC instruction with zero page addressing mode
//     cpu.mem_write(0x10, 0x00); // Set the value at address 0x10 to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value after execution (should wrap around to 0xFF)
// }

// #[test]
// fn test_dec_absolute_underflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xCE, 0x00, 0x90]); // DEC instruction with absolute addressing mode
//     cpu.mem_write(0x9000, 0x00); // Set the value at address 0x9000 to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEC instruction
//     assert_eq!(cpu.mem_read(0x9000), 0xFF); // Check the value after execution (should wrap around to 0xFF)
// }

// // INX, DEX, INY, DEY

// #[test]
// fn test_inx() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xE8]); // INX instruction
//     cpu.register_x = 0x7F; // Set X register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INX instruction
//     assert_eq!(cpu.register_x, 0x80); // Check the value after execution

//     // Edge Case: Incrementing 0xFF should wrap around to 0x00
//     cpu.load(&vec![0xE8]); // INX instruction
//     cpu.register_x = 0xFF; // Set X register to 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INX instruction
//     assert_eq!(cpu.register_x, 0x00); // Check the value after execution
// }

// #[test]
// fn test_iny() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xC8]); // INY instruction
//     cpu.register_y = 0x7F; // Set Y register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INY instruction
//     assert_eq!(cpu.register_y, 0x80); // Check the value after execution

//     // Edge Case: Incrementing 0xFF should wrap around to 0x00
//     cpu.load(&vec![0xC8]); // INY instruction
//     cpu.register_y = 0xFF; // Set Y register to 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the INY instruction
//     assert_eq!(cpu.register_y, 0x00); // Check the value after execution
// }

// #[test]
// fn test_dex() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xCA]); // DEX instruction
//     cpu.register_x = 0x80; // Set X register to 0x80
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value after execution

//     // Edge Case: Decrementing 0x00 should wrap around to 0xFF
//     cpu.load(&vec![0xCA]); // DEX instruction
//     cpu.register_x = 0x00; // Set X register to 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEX instruction
//     assert_eq!(cpu.register_x, 0xFF); // Check the value after execution
// }

// #[test]
// fn test_dey() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x88]); // DEY instruction
//     cpu.register_y = 0x80; // Set Y register to 0x80
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value after execution

//     // Edge Case: Decrementing 0x00 should wrap around to 0xFF
//     cpu.load(&vec![0x88]); // DEY instruction
//     cpu.register_y = 0x00; // Set Y register to 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the DEY instruction
//     assert_eq!(cpu.register_y, 0xFF); // Check the value after execution
// }

// // LDX

// #[test]
// fn test_ldx_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA2, 0x42]); // LDX instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x42); // Check the value after execution
// }

// #[test]
// fn test_ldx_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA6, 0x10]); // LDX instruction with zero page addressing mode
//     cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldx_zero_page_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xB6, 0x10]); // LDX instruction with zero page indexed addressing mode (Y register)
//     cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
//     cpu.register_y = 0x02; // Set Y register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldx_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xAE, 0x00, 0x90]); // LDX instruction with absolute addressing mode
//     cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldx_absolute_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xBE, 0x00, 0x90]); // LDX instruction with absolute indexed addressing mode (Y register)
//     cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
//     cpu.register_y = 0x02; // Set Y register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldx_edge_cases() {
//     let mut cpu = Cpu::new();

//     // Edge Case 1: Loading 0x00
//     cpu.load(&vec![0xA2, 0x00]); // LDX instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0x00); // Check the value after execution

//     // Edge Case 2: Loading 0xFF
//     cpu.load(&vec![0xA2, 0xFF]); // LDX instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDX instruction
//     assert_eq!(cpu.register_x, 0xFF); // Check the value after execution
// }

// // LDY

// #[test]
// fn test_ldy_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA0, 0x42]); // LDY instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x42); // Check the value after execution
// }

// #[test]
// fn test_ldy_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA4, 0x10]); // LDY instruction with zero page addressing mode
//     cpu.mem_write(0x10, 0x7F); // Set the value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldy_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xB4, 0x10]); // LDY instruction with zero page indexed addressing mode (X register)
//     cpu.mem_write(0x12, 0x7F); // Set the value at address 0x12 to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldy_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xAC, 0x00, 0x90]); // LDY instruction with absolute addressing mode
//     cpu.mem_write(0x9000, 0x7F); // Set the value at address 0x9000 to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldy_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xBC, 0x00, 0x90]); // LDY instruction with absolute indexed addressing mode (X register)
//     cpu.mem_write(0x9002, 0x7F); // Set the value at address 0x9002 to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value after execution
// }

// #[test]
// fn test_ldy_edge_cases() {
//     let mut cpu = Cpu::new();

//     // Edge Case 1: Loading 0x00
//     cpu.load(&vec![0xA0, 0x00]); // LDY instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0x00); // Check the value after execution

//     // Edge Case 2: Loading 0xFF
//     cpu.load(&vec![0xA0, 0xFF]); // LDY instruction with immediate addressing mode
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the LDY instruction
//     assert_eq!(cpu.register_y, 0xFF); // Check the value after execution
// }

// // STA

// #[test]
// fn test_sta_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x85, 0x10]); // STA instruction with zero page addressing mode
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
// }

// #[test]
// fn test_sta_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x95, 0x10]); // STA instruction with zero page indexed addressing mode (X register)
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
// }

// #[test]
// fn test_sta_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x8D, 0x00, 0x90]); // STA instruction with absolute addressing mode
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
// }

// #[test]
// fn test_sta_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x9D, 0x00, 0x90]); // STA instruction with absolute indexed addressing mode (X register)
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x9002), 0x7F); // Check the value at address 0x9002 after execution
// }

// #[test]
// fn test_sta_absolute_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x99, 0x00, 0x90]); // STA instruction with absolute indexed addressing mode (Y register)
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.register_y = 0x02; // Set Y register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x9002), 0x7F); // Check the value at address 0x9002 after execution
// }

// #[test]
// fn test_sta_zero_page_underflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x85, 0x10]); // STA instruction with zero page addressing mode
//     cpu.register_a = 0x00; // Set A register to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution
// }

// #[test]
// fn test_sta_absolute_underflow() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x8D, 0x00, 0x90]); // STA instruction with absolute addressing mode
//     cpu.register_a = 0x00; // Set A register to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STA instruction
//     assert_eq!(cpu.mem_read(0x9000), 0x00); // Check the value at address 0x9000 after execution
// }

// // STX

// #[test]
// fn test_stx_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
//     cpu.register_x = 0x7F; // Set X register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STX instruction
//     assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
// }

// #[test]
// fn test_stx_zero_page_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x96, 0x10]); // STX instruction with zero page indexed addressing mode (Y register)
//     cpu.register_x = 0x7F; // Set X register to 0x7F
//     cpu.register_y = 0x02; // Set Y register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STX instruction
//     assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
// }

// #[test]
// fn test_stx_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x8E, 0x00, 0x90]); // STX instruction with absolute addressing mode
//     cpu.register_x = 0x7F; // Set X register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STX instruction
//     assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
// }

// #[test]
// fn test_stx_edge_cases() {
//     let mut cpu = Cpu::new();

//     // Edge Case 1: Storing 0x00
//     cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
//     cpu.register_x = 0x00; // Set X register to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STX instruction
//     assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution

//     // Edge Case 2: Storing 0xFF
//     cpu.load(&vec![0x86, 0x10]); // STX instruction with zero page addressing mode
//     cpu.register_x = 0xFF; // Set X register to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STX instruction
//     assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value at address 0x10 after execution
// }

// // STY

// #[test]
// fn test_sty_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
//     cpu.register_y = 0x7F; // Set Y register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STY instruction
//     assert_eq!(cpu.mem_read(0x10), 0x7F); // Check the value at address 0x10 after execution
// }

// #[test]
// fn test_sty_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x94, 0x10]); // STY instruction with zero page indexed addressing mode (X register)
//     cpu.register_y = 0x7F; // Set Y register to 0x7F
//     cpu.register_x = 0x02; // Set X register
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STY instruction
//     assert_eq!(cpu.mem_read(0x12), 0x7F); // Check the value at address 0x12 after execution
// }

// #[test]
// fn test_sty_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x8C, 0x00, 0x90]); // STY instruction with absolute addressing mode
//     cpu.register_y = 0x7F; // Set Y register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STY instruction
//     assert_eq!(cpu.mem_read(0x9000), 0x7F); // Check the value at address 0x9000 after execution
// }

// #[test]
// fn test_sty_edge_cases() {
//     let mut cpu = Cpu::new();

//     // Edge Case 1: Storing 0x00
//     cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
//     cpu.register_y = 0x00; // Set Y register to minimum value 0x00
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STY instruction
//     assert_eq!(cpu.mem_read(0x10), 0x00); // Check the value at address 0x10 after execution

//     // Edge Case 2: Storing 0xFF
//     cpu.load(&vec![0x84, 0x10]); // STY instruction with zero page addressing mode
//     cpu.register_y = 0xFF; // Set Y register to maximum value 0xFF
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the STY instruction
//     assert_eq!(cpu.mem_read(0x10), 0xFF); // Check the value at address 0x10 after execution
// }

// // TAX, TAY, TYA

// #[test]
// fn test_tax() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xAA]); // TAX instruction
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the TAX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value of X register after execution
// }

// #[test]
// fn test_tay() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA8]); // TAY instruction
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the TAY instruction
//     assert_eq!(cpu.register_y, 0x7F); // Check the value of Y register after execution
// }

// #[test]
// fn test_tsx() {
//     let mut cpu = Cpu::new();
//     cpu.register_s = 0xFC;
//     cpu.load(&vec![0xBA]); // TSX instruction
//     cpu.push(0x7F); // Push 0x7F onto the stack
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the TSX instruction
//     assert_eq!(cpu.register_x, 0x7F); // Check the value of X register after execution
// }

// // PHA, PHP, PLA, PLP

// #[test]
// fn test_pha() {
//     let mut cpu = Cpu::new();
//     cpu.register_s = 0xFC;
//     cpu.load(&vec![0x48]); // PHA instruction
//     cpu.register_a = 0x7F; // Set A register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the PHA instruction
//     assert_eq!(cpu.pull(), 0x7F); // Check the value on the stack after execution
// }

// #[test]
// fn test_php() {
//     let mut cpu = Cpu::new();
//     cpu.register_s = 0xFC;
//     cpu.load(&vec![0x08]); // PHP instruction
//     cpu.status = 0x7F; // Set status register to 0x7F
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the PHP instruction
//     assert_eq!(cpu.pull(), 0x7F); // Check the value on the stack after execution
// }

// #[test]
// fn test_pla() {
//     let mut cpu = Cpu::new();
//     cpu.register_s = 0xFC;
//     cpu.load(&vec![0x68]); // PLA instruction
//     cpu.push(0x7F); // Push 0x7F onto the stack
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the PLA instruction
//     assert_eq!(cpu.register_a, 0x7F); // Check the value in the A register after execution
// }

// #[test]
// fn test_plp() {
//     let mut cpu = Cpu::new();
//     cpu.register_s = 0xFC;
//     cpu.load(&vec![0x28]); // PLP instruction
//     cpu.push(0x7F); // Push 0x7F onto the stack
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the PLP instruction
//     assert_eq!(cpu.status, 0x7F); // Check the value in the status register after execution
// }

// // CLC, CLD, CLI, CLV, SEC, SED, and SEI

// #[test]
// fn test_clc() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x18]); // CLC instruction
//     cpu.status = 0x01; // Set Carry flag to 1 (true)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the CLC instruction
//     assert_eq!(cpu.get_flag("C"), false); // Check if Carry flag is cleared (false)
// }

// #[test]
// fn test_cld() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xD8]); // CLD instruction
//     cpu.status = 0x08; // Set Decimal mode flag to 1 (true)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the CLD instruction
//     assert_eq!(cpu.get_flag("D"), false); // Check if Decimal mode flag is cleared (false)
// }

// #[test]
// fn test_cli() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x58]); // CLI instruction
//     cpu.status = 0x04; // Set Interrupt disable flag to 1 (true)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the CLI instruction
//     let val = cpu.get_flag("I");
//     assert_eq!(val, false); // Check if Interrupt disable flag is cleared (false)
// }

// #[test]
// fn test_clv() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xB8]); // CLV instruction
//     cpu.status = 0x40; // Set Overflow flag to 1 (true)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the CLV instruction
//     assert_eq!(cpu.get_flag("V"), false); // Check if Overflow flag is cleared (false)
// }

// #[test]
// fn test_sec() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x38]); // SEC instruction
//     cpu.status = 0x00; // Clear Carry flag (false)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the SEC instruction
//     assert_eq!(cpu.get_flag("C"), true); // Check if Carry flag is set (true)
// }

// #[test]
// fn test_sed() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xF8]); // SED instruction
//     cpu.status = 0x00; // Clear Decimal mode flag (false)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the SED instruction
//     assert_eq!(cpu.get_flag("D"), true); // Check if Decimal mode flag is set (true)
// }

// #[test]
// fn test_sei() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x78]); // SEI instruction
//     cpu.status = 0x00; // Clear Interrupt disable flag (false)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the SEI instruction
//     assert_eq!(cpu.get_flag("I"), true); // Check if Interrupt disable flag is set (true)
// }

// //AND

// #[test]
// fn test_and_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA9, 0x0F, 0x29, 0x07]); // LDA, AND with immediate values
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0x07); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA5, 0x42, 0x25, 0x42]); // LDA, AND with zero page addressing mode
//     cpu.mem_write(0x42, 0b11001100); // Set value at address 0x42
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0b11001100); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA2, 0x02, 0xA9, 0b11110000, 0x95, 0x00]); // LDX, LDA, AND with zero page indexed addressing mode (X register)
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0b11110000); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xAD, 0x00, 0x90, 0x2D, 0x00, 0x90]); // LDA, AND with absolute addressing mode
//     cpu.mem_write(0x9000, 0b10101010); // Set value at address 0x9000
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0b10101010); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA2, 0x02, 0xBD, 0x00, 0x02]); // LDX, LDA, AND with absolute indexed addressing mode (X register)
//     cpu.mem_write(0x0202, 0b11001100); // Set value at address 0x0202
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0b11001100); // Check if A register has correct value after AND operation
// }

// // Edge Case: Zero result
// #[test]
// fn test_and_zero_result() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA9, 0b11001100, 0x29, 0b00110011]); // LDA, AND with immediate values
//     cpu.pc = 0x8000; // Set program counter to 0x8000
//     cpu.run(); // Execute the instructions
//     assert_eq!(cpu.register_a, 0b00000000); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_indirect_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA2, 0x01, 0x86, 0x01, 0xA9, 0xCC, 0x21, 0x00]); // LDX, STX, LDA, AND with indirect X addressing mode
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0); // Check if A register has correct value after AND operation
// }

// #[test]
// fn test_and_indirect_y() {
//     let mut cpu = Cpu::new();
//     cpu.mem_write(0x0000, 0x01);
//     cpu.load(&vec![
//         0xA0, 0x02, 0x86, 0x02, 0x85, 0x03, 0xA9, 0b00110011, 0x21, 0x00,
//     ]); // LDY, STX, STY, LDA, AND with indirect Y addressing mode
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0b00000000); // Check if A register has correct value after AND operation
// }

// // EOR

// #[test]
// fn test_eor_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA9, 0b11001100, 0x49, 0b10101010]);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0b01100110);
// }

// #[test]
// fn test_eor_immediate_zero_result() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA9, 0b11001100, 0x49, 0b11001100]);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0b00000000);
// }

// #[test]
// fn test_eor_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x02;
//     cpu.mem_write(0x10, 0xAB);
//     cpu.load(&vec![0xA9, 0b10101010, 0x45, 0x10]);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x01);
// }

// #[test]
// fn test_eor_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x02;
//     cpu.mem_write(0x12, 0xAB);
//     cpu.load(&vec![0xA9, 0b10101010, 0x55, 0x10]);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x01);
// }

// #[test]
// fn test_eor_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0xA9, 0b10101010, 0x4D, 0x00, 0x10]);
//     cpu.mem_write(0x1000, 0xAB);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x1);
// }

// #[test]
// fn test_eor_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x02;
//     cpu.load(&vec![0xA9, 0b10101010, 0x5D, 0x00, 0x10]);
//     cpu.mem_write(0x1002, 0xAB);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x1);
// }

// #[test]
// fn test_eor_absolute_y() {
//     let mut cpu = Cpu::new();
//     cpu.register_y = 0x02;
//     cpu.load(&vec![0xA9, 0b10101010, 0x59, 0x00, 0x10]);
//     cpu.mem_write(0x1002, 0xAB);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x1);
// }

// #[test]
// fn test_eor_indirect_x() {
//     let mut cpu = Cpu::new();
//     cpu.register_x = 0x01;
//     cpu.register_a = 0b10101010;
//     cpu.load(&vec![0x41, 0xF0]);
//     cpu.mem_write(0x00F1, 0x00);
//     cpu.mem_write(0x00F2, 0x90);
//     cpu.mem_write(0x9000, 0xAB);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x01);
// }

// #[test]
// fn test_eor_indirect_y() {
//     let mut cpu = Cpu::new();
//     cpu.register_y = 0x01;
//     cpu.register_a = 0b10101010;
//     cpu.load(&vec![0x51, 0xF0]);
//     cpu.mem_write(0x00F0, 0x00);
//     cpu.mem_write(0x00F1, 0x90);
//     cpu.mem_write(0x9001, 0xAB);
//     cpu.pc = 0x8000;
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x01);
// }

// // ORA

// #[test]
// fn test_ora_immediate() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x09, 0x3C]); // ORA immediate with value 0x3C
//     cpu.register_a = 0x24; // Set A register to 0x24
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x3C);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x05, 0x10]); // ORA zero page with address 0x10
//     cpu.mem_write(0x0010, 0x2A); // Set value at address 0x10 to 0x2A
//     cpu.register_a = 0x05; // Set A register to 0x05
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2F); // 0x05 | 0x2A = 0x2F
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x15, 0x10]); // ORA zero page,X with address 0x10
//     cpu.register_x = 0x03; // Set X register to 0x03
//     cpu.mem_write(0x0013, 0x0F); // Set value at address 0x13 (0x10 + 0x03) to 0x0F
//     cpu.register_a = 0x21; // Set A register to 0x21
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x2F); // 0x21 | 0x0F = 0x2F
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x0D, 0x34, 0x12]); // ORA absolute with address 0x1234
//     cpu.mem_write(0x1234, 0x42); // Set value at address 0x1234 to 0x42
//     cpu.register_a = 0x31; // Set A register to 0x31
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x73); // 0x31 | 0x42 = 0x73
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x1D, 0x34, 0x12]); // ORA absolute,X with address 0x1234
//     cpu.register_x = 0x02; // Set X register to 0x02
//     cpu.mem_write(0x1236, 0x1E); // Set value at address 0x1236 (0x1234 + 0x02) to 0x1E
//     cpu.register_a = 0x37; // Set A register to 0x37
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x3F); // 0x37 | 0x1E = 0x3F
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_absolute_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x19, 0x34, 0x12]); // ORA absolute,Y with address 0x1234
//     cpu.register_y = 0x01; // Set Y register to 0x01
//     cpu.mem_write(0x1235, 0x3C); // Set value at address 0x1235 (0x1234 + 0x01) to 0x3C
//     cpu.register_a = 0x27; // Set A register to 0x27
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x3F); // 0x27 | 0x3C = 0x3F
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_indirect_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x01, 0x10]); // ORA (zero page,X) with address 0x10
//     cpu.register_x = 0x02; // Set X register to 0x02
//     cpu.mem_write(0x0012, 0x2F); // Set value at address 0x12 (0x10 + 0x02) to 0x2F
//     cpu.mem_write(0x2F2F, 0x0A); // Set value at address 0x2F2F to 0x0A
//     cpu.register_a = 0x1E; // Set A register to 0x1E
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x1E | 0x0A); // 0x1E | 0x0A = 0x1E
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ora_indirect_y() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x11, 0x10]); // ORA (zero page),Y with address 0x10
//     cpu.register_y = 0x01; // Set Y register to 0x01
//     cpu.mem_write(0x0010, 0x34); // Set value at address 0x10 to 0x34
//     cpu.mem_write(0x0035, 0x07); // Set value at address 0x35 (0x34 + 0x01) to 0x07
//     cpu.register_a = 0x3A; // Set A register to 0x3A
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x3A | 0x07); // 0x3A | 0x07 = 0x3F
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// // ASL

// #[test]
// fn test_asl_accumulator() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x0A]); // ASL accumulator
//     cpu.register_a = 0x85; // Set A register to 0x85
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x0A); // 0x85 << 1 = 0x0A
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_asl_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x06, 0x10]); // ASL zero page with address 0x10
//     cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0010), 0xFE); // 0x7F << 1 = 0xFE
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// #[test]
// fn test_asl_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x16, 0x10]); // ASL zero page,X with address 0x10
//     cpu.register_x = 0x03; // Set X register to 0x03
//     cpu.mem_write(0x0013, 0x3E); // Set value at address 0x13 (0x10 + 0x03) to 0x3E
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0013), 0x7C); // 0x3E << 1 = 0x7C
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_asl_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x0E, 0x34, 0x12]); // ASL absolute with address 0x1234
//     cpu.mem_write(0x1234, 0xB3); // Set value at address 0x1234 to 0xB3
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1234), 0x66); // 0xB3 << 1 = 0x66
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_asl_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x1E, 0x34, 0x12]); // ASL absolute,X with address 0x1234
//     cpu.register_x = 0x02; // Set X register to 0x02
//     cpu.mem_write(0x1236, 0x19); // Set value at address 0x1236 (0x1234 + 0x02) to 0x19
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1236), 0x32); // 0x19 << 1 = 0x32
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// // LSR

// #[test]
// fn test_lsr_accumulator() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x4A]); // LSR accumulator
//     cpu.register_a = 0x85; // Set A register to 0x85
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x85 >> 1); // 0x85 >> 1 = 0x42
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_lsr_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x46, 0x10]); // LSR zero page with address 0x10
//     cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0010), 0x3F); // 0x7F >> 1 = 0x3F
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_lsr_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x56, 0x10]); // LSR zero page,X with address 0x10
//     cpu.register_x = 0x03; // Set X register to 0x03
//     cpu.mem_write(0x0013, 0x3E); // Set value at address 0x13 (0x10 + 0x03) to 0x3E
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0013), 0x1F); // 0x3E >> 1 = 0x1F
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_lsr_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x4E, 0x34, 0x12]); // LSR absolute with address 0x1234
//     cpu.mem_write(0x1234, 0xB3); // Set value at address 0x1234 to 0xB3
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1234), 0x59); // 0xB3 >> 1 = 0x59
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_lsr_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x5E, 0x34, 0x12]); // LSR absolute,X with address 0x1234
//     cpu.register_x = 0x02; // Set X register to 0x02
//     cpu.mem_write(0x1236, 0x19); // Set value at address 0x1236 (0x1234 + 0x02) to 0x19
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1236), 0x0C); // 0x19 >> 1 = 0x0C
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// // ROR , ROL

// #[test]
// fn test_ror_accumulator() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x6A]); // ROR accumulator
//     cpu.register_a = 0x85; // Set A register to 0x85
//     cpu.pc = 0x8000; // Set program counter
//     cpu.set_flag("C", true);
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x85 >> 1 | (1 << 7)); // 0x85 >> 1 = 0x42, 0x85 << 7 = 0x00, so result is 0x42
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// #[test]
// fn test_rol_accumulator() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x2A]); // ROL accumulator
//     cpu.register_a = 0x85; // Set A register to 0x85
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.register_a, 0x85 << 1); // 0x85 << 1 = 0x0A, with carry set, result is 0x0B
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_ror_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x66, 0x10]); // ROR zero page with address 0x10
//     cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0010), 0x7F >> 1); // 0x7F >> 1 = 0x3F, 0x7F << 7 = 0xFE, so result is 0xFE
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_rol_zero_page() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x26, 0x10]); // ROL zero page with address 0x10
//     cpu.mem_write(0x0010, 0x7F); // Set value at address 0x10 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0010), 0x7F << 1); // 0x7F << 1 = 0xFE, with carry set, result is 0xFF
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// #[test]
// fn test_ror_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x76, 0x10]); // ROR zero page,X with address 0x10
//     cpu.register_x = 0x01; // Set X register to 0x01
//     cpu.mem_write(0x0011, 0x3E); // Set value at address 0x11 (0x10 + 0x01) to 0x3E
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0011), 0x3E >> 1); // 0x3E >> 1 = 0x1F, 0x3E << 7 = 0xFE, so result is 0xFE
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_rol_zero_page_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x36, 0x10]); // ROL zero page,X with address 0x10
//     cpu.register_x = 0x01; // Set X register to 0x01
//     cpu.mem_write(0x0011, 0x7F); // Set value at address 0x11 (0x10 + 0x01) to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x0011), 0x7F << 1); // 0x7F << 1 = 0xFE, with carry set, result is 0xFF
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// #[test]
// fn test_ror_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x6E, 0x34, 0x12]); // ROR absolute with address 0x1234
//     cpu.mem_write(0x1234, 0x7F); // Set value at address 0x1234 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1234), 0x7F >> 1); // 0x7F >> 1 = 0x3F, 0x7F << 7 = 0xFE, so result is 0xFE
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_rol_absolute() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x2E, 0x34, 0x12]); // ROL absolute with address 0x1234
//     cpu.mem_write(0x1234, 0x7F); // Set value at address 0x1234 to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1234), 0x7F << 1); // 0x7F << 1 = 0xFE, with carry set, result is 0xFF
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// #[test]
// fn test_ror_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x7E, 0x34, 0x12]); // ROR absolute,X with address 0x1234
//     cpu.register_x = 0x01; // Set X register to 0x01
//     cpu.mem_write(0x1235, 0x7F); // Set value at address 0x1235 (0x1234 + 0x01) to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1235), 0x7F >> 1); // 0x7F >> 1 = 0x3F, 0x7F << 7 = 0xFE, so result is 0xFE
//     assert_eq!(cpu.get_flag("C"), true);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), false);
// }

// #[test]
// fn test_rol_absolute_x() {
//     let mut cpu = Cpu::new();
//     cpu.load(&vec![0x3E, 0x34, 0x12]); // ROL absolute,X with address 0x1234
//     cpu.register_x = 0x01; // Set X register to 0x01
//     cpu.mem_write(0x1235, 0x7F); // Set value at address 0x1235 (0x1234 + 0x01) to 0x7F
//     cpu.pc = 0x8000; // Set program counter
//     cpu.run();
//     assert_eq!(cpu.mem_read(0x1235), 0x7F << 1); // 0x7F << 1 = 0xFE, with carry set, result is 0xFF
//     assert_eq!(cpu.get_flag("C"), false);
//     assert_eq!(cpu.get_flag("Z"), false);
//     assert_eq!(cpu.get_flag("N"), true);
// }

// // CMP

#[test]
fn test_a9_c3_0() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc3]);
    cpu.register_a = 22;
    cpu.status = 162;
    cpu.pc = 0x8000;
    cpu.register_s = 215;
    cpu.register_x = 214;
    cpu.register_y = 9;

    cpu.run();

    assert_eq!(cpu.register_a, 195);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 215);
    assert_eq!(cpu.register_x, 214);
    assert_eq!(cpu.register_y, 9);
}

#[test]
fn test_a9_5d_1() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5d]);
    cpu.register_a = 24;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 119;
    cpu.register_x = 252;
    cpu.register_y = 141;

    cpu.run();

    assert_eq!(cpu.register_a, 93);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 119);
    assert_eq!(cpu.register_x, 252);
    assert_eq!(cpu.register_y, 141);
}

#[test]
fn test_a9_ed_2() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xed]);
    cpu.register_a = 73;
    cpu.status = 239;
    cpu.pc = 0x8000;
    cpu.register_s = 227;
    cpu.register_x = 94;
    cpu.register_y = 112;

    cpu.run();

    assert_eq!(cpu.register_a, 237);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 227);
    assert_eq!(cpu.register_x, 94);
    assert_eq!(cpu.register_y, 112);
}

#[test]
fn test_a9_01_3() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x01]);
    cpu.register_a = 70;
    cpu.status = 39;
    cpu.pc = 0x8000;
    cpu.register_s = 4;
    cpu.register_x = 116;
    cpu.register_y = 7;

    cpu.run();

    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 4);
    assert_eq!(cpu.register_x, 116);
    assert_eq!(cpu.register_y, 7);
}

#[test]
fn test_a9_06_4() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x06]);
    cpu.register_a = 240;
    cpu.status = 224;
    cpu.pc = 0x8000;
    cpu.register_s = 121;
    cpu.register_x = 232;
    cpu.register_y = 182;

    cpu.run();

    assert_eq!(cpu.register_a, 6);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 121);
    assert_eq!(cpu.register_x, 232);
    assert_eq!(cpu.register_y, 182);
}

#[test]
fn test_a9_0e_5() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x0e]);
    cpu.register_a = 30;
    cpu.status = 169;
    cpu.pc = 0x8000;
    cpu.register_s = 185;
    cpu.register_x = 103;
    cpu.register_y = 195;

    cpu.run();

    assert_eq!(cpu.register_a, 14);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 185);
    assert_eq!(cpu.register_x, 103);
    assert_eq!(cpu.register_y, 195);
}

#[test]
fn test_a9_e3_6() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe3]);
    cpu.register_a = 235;
    cpu.status = 170;
    cpu.pc = 0x8000;
    cpu.register_s = 93;
    cpu.register_x = 220;
    cpu.register_y = 135;

    cpu.run();

    assert_eq!(cpu.register_a, 227);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 93);
    assert_eq!(cpu.register_x, 220);
    assert_eq!(cpu.register_y, 135);
}

#[test]
fn test_a9_61_7() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x61]);
    cpu.register_a = 220;
    cpu.status = 98;
    cpu.pc = 0x8000;
    cpu.register_s = 118;
    cpu.register_x = 205;
    cpu.register_y = 35;

    cpu.run();

    assert_eq!(cpu.register_a, 97);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 118);
    assert_eq!(cpu.register_x, 205);
    assert_eq!(cpu.register_y, 35);
}

#[test]
fn test_a9_99_8() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x99]);
    cpu.register_a = 231;
    cpu.status = 172;
    cpu.pc = 0x8000;
    cpu.register_s = 198;
    cpu.register_x = 145;
    cpu.register_y = 136;

    cpu.run();

    assert_eq!(cpu.register_a, 153);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 198);
    assert_eq!(cpu.register_x, 145);
    assert_eq!(cpu.register_y, 136);
}

#[test]
fn test_a9_b4_9() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb4]);
    cpu.register_a = 190;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 225;
    cpu.register_x = 44;
    cpu.register_y = 65;

    cpu.run();

    assert_eq!(cpu.register_a, 180);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 225);
    assert_eq!(cpu.register_x, 44);
    assert_eq!(cpu.register_y, 65);
}

#[test]
fn test_a9_47_10() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x47]);
    cpu.register_a = 180;
    cpu.status = 106;
    cpu.pc = 0x8000;
    cpu.register_s = 21;
    cpu.register_x = 51;
    cpu.register_y = 176;

    cpu.run();

    assert_eq!(cpu.register_a, 71);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 21);
    assert_eq!(cpu.register_x, 51);
    assert_eq!(cpu.register_y, 176);
}

#[test]
fn test_a9_5e_11() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5e]);
    cpu.register_a = 251;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 223;
    cpu.register_x = 37;
    cpu.register_y = 136;

    cpu.run();

    assert_eq!(cpu.register_a, 94);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 223);
    assert_eq!(cpu.register_x, 37);
    assert_eq!(cpu.register_y, 136);
}

#[test]
fn test_a9_bb_12() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xbb]);
    cpu.register_a = 172;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 41;
    cpu.register_x = 223;
    cpu.register_y = 248;

    cpu.run();

    assert_eq!(cpu.register_a, 187);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 41);
    assert_eq!(cpu.register_x, 223);
    assert_eq!(cpu.register_y, 248);
}

#[test]
fn test_a9_48_13() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x48]);
    cpu.register_a = 30;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 176;
    cpu.register_x = 2;
    cpu.register_y = 106;

    cpu.run();

    assert_eq!(cpu.register_a, 72);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 176);
    assert_eq!(cpu.register_x, 2);
    assert_eq!(cpu.register_y, 106);
}

#[test]
fn test_a9_0f_14() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x0f]);
    cpu.register_a = 43;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 22;
    cpu.register_x = 161;
    cpu.register_y = 119;

    cpu.run();

    assert_eq!(cpu.register_a, 15);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 22);
    assert_eq!(cpu.register_x, 161);
    assert_eq!(cpu.register_y, 119);
}

#[test]
fn test_a9_93_15() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x93]);
    cpu.register_a = 203;
    cpu.status = 110;
    cpu.pc = 0x8000;
    cpu.register_s = 14;
    cpu.register_x = 221;
    cpu.register_y = 36;

    cpu.run();

    assert_eq!(cpu.register_a, 147);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 14);
    assert_eq!(cpu.register_x, 221);
    assert_eq!(cpu.register_y, 36);
}

#[test]
fn test_a9_bb_16() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xbb]);
    cpu.register_a = 29;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 239;
    cpu.register_x = 4;
    cpu.register_y = 224;

    cpu.run();

    assert_eq!(cpu.register_a, 187);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 239);
    assert_eq!(cpu.register_x, 4);
    assert_eq!(cpu.register_y, 224);
}

#[test]
fn test_a9_75_17() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x75]);
    cpu.register_a = 187;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 51;
    cpu.register_x = 56;
    cpu.register_y = 178;

    cpu.run();

    assert_eq!(cpu.register_a, 117);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 51);
    assert_eq!(cpu.register_x, 56);
    assert_eq!(cpu.register_y, 178);
}

#[test]
fn test_a9_6b_18() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x6b]);
    cpu.register_a = 32;
    cpu.status = 174;
    cpu.pc = 0x8000;
    cpu.register_s = 52;
    cpu.register_x = 13;
    cpu.register_y = 81;

    cpu.run();

    assert_eq!(cpu.register_a, 107);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 52);
    assert_eq!(cpu.register_x, 13);
    assert_eq!(cpu.register_y, 81);
}

#[test]
fn test_a9_03_19() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x03]);
    cpu.register_a = 121;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 132;
    cpu.register_x = 250;
    cpu.register_y = 44;

    cpu.run();

    assert_eq!(cpu.register_a, 3);
    assert_eq!(cpu.status, 32);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 132);
    assert_eq!(cpu.register_x, 250);
    assert_eq!(cpu.register_y, 44);
}

#[test]
fn test_a9_1b_20() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1b]);
    cpu.register_a = 200;
    cpu.status = 174;
    cpu.pc = 0x8000;
    cpu.register_s = 13;
    cpu.register_x = 10;
    cpu.register_y = 108;

    cpu.run();

    assert_eq!(cpu.register_a, 27);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 13);
    assert_eq!(cpu.register_x, 10);
    assert_eq!(cpu.register_y, 108);
}

#[test]
fn test_a9_cf_21() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcf]);
    cpu.register_a = 150;
    cpu.status = 36;
    cpu.pc = 0x8000;
    cpu.register_s = 171;
    cpu.register_x = 176;
    cpu.register_y = 102;

    cpu.run();

    assert_eq!(cpu.register_a, 207);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 171);
    assert_eq!(cpu.register_x, 176);
    assert_eq!(cpu.register_y, 102);
}

#[test]
fn test_a9_89_22() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x89]);
    cpu.register_a = 169;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 28;
    cpu.register_x = 151;
    cpu.register_y = 217;

    cpu.run();

    assert_eq!(cpu.register_a, 137);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 28);
    assert_eq!(cpu.register_x, 151);
    assert_eq!(cpu.register_y, 217);
}

#[test]
fn test_a9_36_23() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x36]);
    cpu.register_a = 151;
    cpu.status = 163;
    cpu.pc = 0x8000;
    cpu.register_s = 70;
    cpu.register_x = 130;
    cpu.register_y = 253;

    cpu.run();

    assert_eq!(cpu.register_a, 54);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 70);
    assert_eq!(cpu.register_x, 130);
    assert_eq!(cpu.register_y, 253);
}

#[test]
fn test_a9_fb_24() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfb]);
    cpu.register_a = 13;
    cpu.status = 111;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 140;
    cpu.register_y = 235;

    cpu.run();

    assert_eq!(cpu.register_a, 251);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 140);
    assert_eq!(cpu.register_y, 235);
}

#[test]
fn test_a9_ec_25() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xec]);
    cpu.register_a = 142;
    cpu.status = 173;
    cpu.pc = 0x8000;
    cpu.register_s = 15;
    cpu.register_x = 233;
    cpu.register_y = 177;

    cpu.run();

    assert_eq!(cpu.register_a, 236);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 15);
    assert_eq!(cpu.register_x, 233);
    assert_eq!(cpu.register_y, 177);
}

#[test]
fn test_a9_47_26() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x47]);
    cpu.register_a = 105;
    cpu.status = 169;
    cpu.pc = 0x8000;
    cpu.register_s = 7;
    cpu.register_x = 23;
    cpu.register_y = 155;

    cpu.run();

    assert_eq!(cpu.register_a, 71);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 7);
    assert_eq!(cpu.register_x, 23);
    assert_eq!(cpu.register_y, 155);
}

#[test]
fn test_a9_8f_27() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8f]);
    cpu.register_a = 236;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 250;
    cpu.register_x = 110;
    cpu.register_y = 121;

    cpu.run();

    assert_eq!(cpu.register_a, 143);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 250);
    assert_eq!(cpu.register_x, 110);
    assert_eq!(cpu.register_y, 121);
}

#[test]
fn test_a9_fc_28() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfc]);
    cpu.register_a = 200;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 113;
    cpu.register_y = 182;

    cpu.run();

    assert_eq!(cpu.register_a, 252);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 113);
    assert_eq!(cpu.register_y, 182);
}

#[test]
fn test_a9_77_29() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x77]);
    cpu.register_a = 227;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 174;
    cpu.register_x = 220;
    cpu.register_y = 24;

    cpu.run();

    assert_eq!(cpu.register_a, 119);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 174);
    assert_eq!(cpu.register_x, 220);
    assert_eq!(cpu.register_y, 24);
}

#[test]
fn test_a9_e1_30() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe1]);
    cpu.register_a = 6;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 55;
    cpu.register_x = 230;
    cpu.register_y = 122;

    cpu.run();

    assert_eq!(cpu.register_a, 225);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 55);
    assert_eq!(cpu.register_x, 230);
    assert_eq!(cpu.register_y, 122);
}

#[test]
fn test_a9_f3_31() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf3]);
    cpu.register_a = 58;
    cpu.status = 42;
    cpu.pc = 0x8000;
    cpu.register_s = 168;
    cpu.register_x = 30;
    cpu.register_y = 145;

    cpu.run();

    assert_eq!(cpu.register_a, 243);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 168);
    assert_eq!(cpu.register_x, 30);
    assert_eq!(cpu.register_y, 145);
}

#[test]
fn test_a9_f8_32() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf8]);
    cpu.register_a = 220;
    cpu.status = 100;
    cpu.pc = 0x8000;
    cpu.register_s = 64;
    cpu.register_x = 248;
    cpu.register_y = 72;

    cpu.run();

    assert_eq!(cpu.register_a, 248);
    assert_eq!(cpu.status, 228);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 64);
    assert_eq!(cpu.register_x, 248);
    assert_eq!(cpu.register_y, 72);
}

#[test]
fn test_a9_37_33() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x37]);
    cpu.register_a = 253;
    cpu.status = 36;
    cpu.pc = 0x8000;
    cpu.register_s = 82;
    cpu.register_x = 211;
    cpu.register_y = 243;

    cpu.run();

    assert_eq!(cpu.register_a, 55);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 82);
    assert_eq!(cpu.register_x, 211);
    assert_eq!(cpu.register_y, 243);
}

#[test]
fn test_a9_f2_34() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf2]);
    cpu.register_a = 226;
    cpu.status = 168;
    cpu.pc = 0x8000;
    cpu.register_s = 165;
    cpu.register_x = 157;
    cpu.register_y = 75;

    cpu.run();

    assert_eq!(cpu.register_a, 242);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 165);
    assert_eq!(cpu.register_x, 157);
    assert_eq!(cpu.register_y, 75);
}

#[test]
fn test_a9_41_35() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x41]);
    cpu.register_a = 6;
    cpu.status = 170;
    cpu.pc = 0x8000;
    cpu.register_s = 33;
    cpu.register_x = 211;
    cpu.register_y = 116;

    cpu.run();

    assert_eq!(cpu.register_a, 65);
    assert_eq!(cpu.status, 40);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 33);
    assert_eq!(cpu.register_x, 211);
    assert_eq!(cpu.register_y, 116);
}

#[test]
fn test_a9_25_36() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x25]);
    cpu.register_a = 1;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 13;
    cpu.register_x = 158;
    cpu.register_y = 111;

    cpu.run();

    assert_eq!(cpu.register_a, 37);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 13);
    assert_eq!(cpu.register_x, 158);
    assert_eq!(cpu.register_y, 111);
}

#[test]
fn test_a9_c2_37() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc2]);
    cpu.register_a = 137;
    cpu.status = 233;
    cpu.pc = 0x8000;
    cpu.register_s = 239;
    cpu.register_x = 35;
    cpu.register_y = 234;

    cpu.run();

    assert_eq!(cpu.register_a, 194);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 239);
    assert_eq!(cpu.register_x, 35);
    assert_eq!(cpu.register_y, 234);
}

#[test]
fn test_a9_c6_38() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc6]);
    cpu.register_a = 247;
    cpu.status = 230;
    cpu.pc = 0x8000;
    cpu.register_s = 139;
    cpu.register_x = 47;
    cpu.register_y = 169;

    cpu.run();

    assert_eq!(cpu.register_a, 198);
    assert_eq!(cpu.status, 228);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 139);
    assert_eq!(cpu.register_x, 47);
    assert_eq!(cpu.register_y, 169);
}

#[test]
fn test_a9_83_39() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x83]);
    cpu.register_a = 251;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 31;
    cpu.register_x = 124;
    cpu.register_y = 190;

    cpu.run();

    assert_eq!(cpu.register_a, 131);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 31);
    assert_eq!(cpu.register_x, 124);
    assert_eq!(cpu.register_y, 190);
}

#[test]
fn test_a9_a0_40() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xa0]);
    cpu.register_a = 228;
    cpu.status = 99;
    cpu.pc = 0x8000;
    cpu.register_s = 6;
    cpu.register_x = 169;
    cpu.register_y = 10;

    cpu.run();

    assert_eq!(cpu.register_a, 160);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 6);
    assert_eq!(cpu.register_x, 169);
    assert_eq!(cpu.register_y, 10);
}

#[test]
fn test_a9_86_41() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x86]);
    cpu.register_a = 92;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 128;
    cpu.register_x = 145;
    cpu.register_y = 55;

    cpu.run();

    assert_eq!(cpu.register_a, 134);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 128);
    assert_eq!(cpu.register_x, 145);
    assert_eq!(cpu.register_y, 55);
}

#[test]
fn test_a9_7e_42() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x7e]);
    cpu.register_a = 13;
    cpu.status = 96;
    cpu.pc = 0x8000;
    cpu.register_s = 155;
    cpu.register_x = 231;
    cpu.register_y = 17;

    cpu.run();

    assert_eq!(cpu.register_a, 126);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 155);
    assert_eq!(cpu.register_x, 231);
    assert_eq!(cpu.register_y, 17);
}

#[test]
fn test_a9_39_43() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x39]);
    cpu.register_a = 148;
    cpu.status = 102;
    cpu.pc = 0x8000;
    cpu.register_s = 83;
    cpu.register_x = 169;
    cpu.register_y = 241;

    cpu.run();

    assert_eq!(cpu.register_a, 57);
    assert_eq!(cpu.status, 100);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 83);
    assert_eq!(cpu.register_x, 169);
    assert_eq!(cpu.register_y, 241);
}

#[test]
fn test_a9_cf_44() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcf]);
    cpu.register_a = 77;
    cpu.status = 40;
    cpu.pc = 0x8000;
    cpu.register_s = 165;
    cpu.register_x = 4;
    cpu.register_y = 180;

    cpu.run();

    assert_eq!(cpu.register_a, 207);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 165);
    assert_eq!(cpu.register_x, 4);
    assert_eq!(cpu.register_y, 180);
}

#[test]
fn test_a9_25_45() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x25]);
    cpu.register_a = 42;
    cpu.status = 235;
    cpu.pc = 0x8000;
    cpu.register_s = 32;
    cpu.register_x = 68;
    cpu.register_y = 190;

    cpu.run();

    assert_eq!(cpu.register_a, 37);
    assert_eq!(cpu.status, 105);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 32);
    assert_eq!(cpu.register_x, 68);
    assert_eq!(cpu.register_y, 190);
}

#[test]
fn test_a9_40_46() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x40]);
    cpu.register_a = 37;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 67;
    cpu.register_x = 54;
    cpu.register_y = 135;

    cpu.run();

    assert_eq!(cpu.register_a, 64);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 67);
    assert_eq!(cpu.register_x, 54);
    assert_eq!(cpu.register_y, 135);
}

#[test]
fn test_a9_32_47() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x32]);
    cpu.register_a = 125;
    cpu.status = 39;
    cpu.pc = 0x8000;
    cpu.register_s = 75;
    cpu.register_x = 63;
    cpu.register_y = 46;

    cpu.run();

    assert_eq!(cpu.register_a, 50);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 75);
    assert_eq!(cpu.register_x, 63);
    assert_eq!(cpu.register_y, 46);
}

#[test]
fn test_a9_8a_48() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8a]);
    cpu.register_a = 47;
    cpu.status = 226;
    cpu.pc = 0x8000;
    cpu.register_s = 110;
    cpu.register_x = 231;
    cpu.register_y = 151;

    cpu.run();

    assert_eq!(cpu.register_a, 138);
    assert_eq!(cpu.status, 224);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 110);
    assert_eq!(cpu.register_x, 231);
    assert_eq!(cpu.register_y, 151);
}

#[test]
fn test_a9_27_49() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x27]);
    cpu.register_a = 16;
    cpu.status = 104;
    cpu.pc = 0x8000;
    cpu.register_s = 22;
    cpu.register_x = 234;
    cpu.register_y = 252;

    cpu.run();

    assert_eq!(cpu.register_a, 39);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 22);
    assert_eq!(cpu.register_x, 234);
    assert_eq!(cpu.register_y, 252);
}

#[test]
fn test_a9_cb_50() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcb]);
    cpu.register_a = 55;
    cpu.status = 38;
    cpu.pc = 0x8000;
    cpu.register_s = 216;
    cpu.register_x = 73;
    cpu.register_y = 90;

    cpu.run();

    assert_eq!(cpu.register_a, 203);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 216);
    assert_eq!(cpu.register_x, 73);
    assert_eq!(cpu.register_y, 90);
}

#[test]
fn test_a9_12_51() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x12]);
    cpu.register_a = 21;
    cpu.status = 47;
    cpu.pc = 0x8000;
    cpu.register_s = 52;
    cpu.register_x = 222;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 18);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 52);
    assert_eq!(cpu.register_x, 222);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_26_52() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x26]);
    cpu.register_a = 236;
    cpu.status = 108;
    cpu.pc = 0x8000;
    cpu.register_s = 188;
    cpu.register_x = 86;
    cpu.register_y = 117;

    cpu.run();

    assert_eq!(cpu.register_a, 38);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 188);
    assert_eq!(cpu.register_x, 86);
    assert_eq!(cpu.register_y, 117);
}

#[test]
fn test_a9_41_53() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x41]);
    cpu.register_a = 159;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 187;
    cpu.register_x = 128;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 65);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 187);
    assert_eq!(cpu.register_x, 128);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_2f_54() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2f]);
    cpu.register_a = 169;
    cpu.status = 33;
    cpu.pc = 0x8000;
    cpu.register_s = 31;
    cpu.register_x = 9;
    cpu.register_y = 138;

    cpu.run();

    assert_eq!(cpu.register_a, 47);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 31);
    assert_eq!(cpu.register_x, 9);
    assert_eq!(cpu.register_y, 138);
}

#[test]
fn test_a9_1c_55() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1c]);
    cpu.register_a = 7;
    cpu.status = 109;
    cpu.pc = 0x8000;
    cpu.register_s = 68;
    cpu.register_x = 245;
    cpu.register_y = 83;

    cpu.run();

    assert_eq!(cpu.register_a, 28);
    assert_eq!(cpu.status, 109);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 68);
    assert_eq!(cpu.register_x, 245);
    assert_eq!(cpu.register_y, 83);
}

#[test]
fn test_a9_d7_56() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd7]);
    cpu.register_a = 73;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 101;
    cpu.register_x = 92;
    cpu.register_y = 172;

    cpu.run();

    assert_eq!(cpu.register_a, 215);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 101);
    assert_eq!(cpu.register_x, 92);
    assert_eq!(cpu.register_y, 172);
}

#[test]
fn test_a9_f0_57() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf0]);
    cpu.register_a = 72;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 213;
    cpu.register_x = 170;
    cpu.register_y = 191;

    cpu.run();

    assert_eq!(cpu.register_a, 240);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 213);
    assert_eq!(cpu.register_x, 170);
    assert_eq!(cpu.register_y, 191);
}

#[test]
fn test_a9_67_58() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x67]);
    cpu.register_a = 150;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 170;
    cpu.register_x = 47;
    cpu.register_y = 23;

    cpu.run();

    assert_eq!(cpu.register_a, 103);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 170);
    assert_eq!(cpu.register_x, 47);
    assert_eq!(cpu.register_y, 23);
}

#[test]
fn test_a9_80_59() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x80]);
    cpu.register_a = 216;
    cpu.status = 225;
    cpu.pc = 0x8000;
    cpu.register_s = 194;
    cpu.register_x = 36;
    cpu.register_y = 63;

    cpu.run();

    assert_eq!(cpu.register_a, 128);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 194);
    assert_eq!(cpu.register_x, 36);
    assert_eq!(cpu.register_y, 63);
}

#[test]
fn test_a9_d8_60() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd8]);
    cpu.register_a = 206;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 138;
    cpu.register_x = 224;
    cpu.register_y = 100;

    cpu.run();

    assert_eq!(cpu.register_a, 216);
    assert_eq!(cpu.status, 165);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 138);
    assert_eq!(cpu.register_x, 224);
    assert_eq!(cpu.register_y, 100);
}

#[test]
fn test_a9_9a_61() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x9a]);
    cpu.register_a = 104;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 162;
    cpu.register_x = 198;
    cpu.register_y = 12;

    cpu.run();

    assert_eq!(cpu.register_a, 154);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 162);
    assert_eq!(cpu.register_x, 198);
    assert_eq!(cpu.register_y, 12);
}

#[test]
fn test_a9_40_62() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x40]);
    cpu.register_a = 80;
    cpu.status = 96;
    cpu.pc = 0x8000;
    cpu.register_s = 117;
    cpu.register_x = 11;
    cpu.register_y = 46;

    cpu.run();

    assert_eq!(cpu.register_a, 64);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 117);
    assert_eq!(cpu.register_x, 11);
    assert_eq!(cpu.register_y, 46);
}

#[test]
fn test_a9_37_63() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x37]);
    cpu.register_a = 228;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 244;
    cpu.register_x = 221;
    cpu.register_y = 255;

    cpu.run();

    assert_eq!(cpu.register_a, 55);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 244);
    assert_eq!(cpu.register_x, 221);
    assert_eq!(cpu.register_y, 255);
}

#[test]
fn test_a9_fa_64() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfa]);
    cpu.register_a = 196;
    cpu.status = 232;
    cpu.pc = 0x8000;
    cpu.register_s = 213;
    cpu.register_x = 74;
    cpu.register_y = 203;

    cpu.run();

    assert_eq!(cpu.register_a, 250);
    assert_eq!(cpu.status, 232);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 213);
    assert_eq!(cpu.register_x, 74);
    assert_eq!(cpu.register_y, 203);
}

#[test]
fn test_a9_e5_65() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe5]);
    cpu.register_a = 43;
    cpu.status = 233;
    cpu.pc = 0x8000;
    cpu.register_s = 16;
    cpu.register_x = 184;
    cpu.register_y = 144;

    cpu.run();

    assert_eq!(cpu.register_a, 229);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 16);
    assert_eq!(cpu.register_x, 184);
    assert_eq!(cpu.register_y, 144);
}

#[test]
fn test_a9_f5_66() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf5]);
    cpu.register_a = 155;
    cpu.status = 42;
    cpu.pc = 0x8000;
    cpu.register_s = 38;
    cpu.register_x = 49;
    cpu.register_y = 151;

    cpu.run();

    assert_eq!(cpu.register_a, 245);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 38);
    assert_eq!(cpu.register_x, 49);
    assert_eq!(cpu.register_y, 151);
}

#[test]
fn test_a9_42_67() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x42]);
    cpu.register_a = 21;
    cpu.status = 171;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 208;
    cpu.register_y = 48;

    cpu.run();

    assert_eq!(cpu.register_a, 66);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 208);
    assert_eq!(cpu.register_y, 48);
}

#[test]
fn test_a9_ca_68() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xca]);
    cpu.register_a = 28;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 106;
    cpu.register_x = 146;
    cpu.register_y = 244;

    cpu.run();

    assert_eq!(cpu.register_a, 202);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 106);
    assert_eq!(cpu.register_x, 146);
    assert_eq!(cpu.register_y, 244);
}

#[test]
fn test_a9_36_69() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x36]);
    cpu.register_a = 157;
    cpu.status = 232;
    cpu.pc = 0x8000;
    cpu.register_s = 50;
    cpu.register_x = 175;
    cpu.register_y = 153;

    cpu.run();

    assert_eq!(cpu.register_a, 54);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 50);
    assert_eq!(cpu.register_x, 175);
    assert_eq!(cpu.register_y, 153);
}

#[test]
fn test_a9_e0_70() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe0]);
    cpu.register_a = 17;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 251;
    cpu.register_x = 156;
    cpu.register_y = 15;

    cpu.run();

    assert_eq!(cpu.register_a, 224);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 251);
    assert_eq!(cpu.register_x, 156);
    assert_eq!(cpu.register_y, 15);
}

#[test]
fn test_a9_e3_71() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe3]);
    cpu.register_a = 218;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 233;
    cpu.register_x = 255;
    cpu.register_y = 160;

    cpu.run();

    assert_eq!(cpu.register_a, 227);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 233);
    assert_eq!(cpu.register_x, 255);
    assert_eq!(cpu.register_y, 160);
}

#[test]
fn test_a9_7a_72() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x7a]);
    cpu.register_a = 76;
    cpu.status = 225;
    cpu.pc = 0x8000;
    cpu.register_s = 245;
    cpu.register_x = 48;
    cpu.register_y = 144;

    cpu.run();

    assert_eq!(cpu.register_a, 122);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 245);
    assert_eq!(cpu.register_x, 48);
    assert_eq!(cpu.register_y, 144);
}

#[test]
fn test_a9_dc_73() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xdc]);
    cpu.register_a = 94;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 118;
    cpu.register_x = 83;
    cpu.register_y = 188;

    cpu.run();

    assert_eq!(cpu.register_a, 220);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 118);
    assert_eq!(cpu.register_x, 83);
    assert_eq!(cpu.register_y, 188);
}

#[test]
fn test_a9_04_74() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x04]);
    cpu.register_a = 70;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 217;
    cpu.register_x = 154;
    cpu.register_y = 186;

    cpu.run();

    assert_eq!(cpu.register_a, 4);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 217);
    assert_eq!(cpu.register_x, 154);
    assert_eq!(cpu.register_y, 186);
}

#[test]
fn test_a9_6f_75() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x6f]);
    cpu.register_a = 157;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 5;
    cpu.register_x = 80;
    cpu.register_y = 122;

    cpu.run();

    assert_eq!(cpu.register_a, 111);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 5);
    assert_eq!(cpu.register_x, 80);
    assert_eq!(cpu.register_y, 122);
}

#[test]
fn test_a9_d2_76() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd2]);
    cpu.register_a = 248;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 131;
    cpu.register_x = 180;
    cpu.register_y = 167;

    cpu.run();

    assert_eq!(cpu.register_a, 210);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 131);
    assert_eq!(cpu.register_x, 180);
    assert_eq!(cpu.register_y, 167);
}

#[test]
fn test_a9_72_77() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x72]);
    cpu.register_a = 96;
    cpu.status = 109;
    cpu.pc = 0x8000;
    cpu.register_s = 164;
    cpu.register_x = 236;
    cpu.register_y = 61;

    cpu.run();

    assert_eq!(cpu.register_a, 114);
    assert_eq!(cpu.status, 109);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 164);
    assert_eq!(cpu.register_x, 236);
    assert_eq!(cpu.register_y, 61);
}

#[test]
fn test_a9_a5_78() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xa5]);
    cpu.register_a = 103;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 189;
    cpu.register_x = 245;
    cpu.register_y = 12;

    cpu.run();

    assert_eq!(cpu.register_a, 165);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 189);
    assert_eq!(cpu.register_x, 245);
    assert_eq!(cpu.register_y, 12);
}

#[test]
fn test_a9_1d_79() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1d]);
    cpu.register_a = 48;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 136;
    cpu.register_x = 166;
    cpu.register_y = 186;

    cpu.run();

    assert_eq!(cpu.register_a, 29);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 136);
    assert_eq!(cpu.register_x, 166);
    assert_eq!(cpu.register_y, 186);
}

#[test]
fn test_a9_e6_80() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe6]);
    cpu.register_a = 182;
    cpu.status = 33;
    cpu.pc = 0x8000;
    cpu.register_s = 58;
    cpu.register_x = 242;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 230);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 58);
    assert_eq!(cpu.register_x, 242);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_2a_81() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2a]);
    cpu.register_a = 10;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 182;
    cpu.register_x = 241;
    cpu.register_y = 199;

    cpu.run();

    assert_eq!(cpu.register_a, 42);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 182);
    assert_eq!(cpu.register_x, 241);
    assert_eq!(cpu.register_y, 199);
}

#[test]
fn test_a9_1b_82() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1b]);
    cpu.register_a = 81;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 182;
    cpu.register_x = 191;
    cpu.register_y = 237;

    cpu.run();

    assert_eq!(cpu.register_a, 27);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 182);
    assert_eq!(cpu.register_x, 191);
    assert_eq!(cpu.register_y, 237);
}

#[test]
fn test_a9_8e_83() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8e]);
    cpu.register_a = 184;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 184;
    cpu.register_x = 70;
    cpu.register_y = 130;

    cpu.run();

    assert_eq!(cpu.register_a, 142);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 184);
    assert_eq!(cpu.register_x, 70);
    assert_eq!(cpu.register_y, 130);
}

#[test]
fn test_a9_ee_84() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xee]);
    cpu.register_a = 79;
    cpu.status = 161;
    cpu.pc = 0x8000;
    cpu.register_s = 199;
    cpu.register_x = 155;
    cpu.register_y = 10;

    cpu.run();

    assert_eq!(cpu.register_a, 238);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 199);
    assert_eq!(cpu.register_x, 155);
    assert_eq!(cpu.register_y, 10);
}

#[test]
fn test_a9_13_85() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x13]);
    cpu.register_a = 200;
    cpu.status = 99;
    cpu.pc = 0x8000;
    cpu.register_s = 223;
    cpu.register_x = 196;
    cpu.register_y = 97;

    cpu.run();

    assert_eq!(cpu.register_a, 19);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 223);
    assert_eq!(cpu.register_x, 196);
    assert_eq!(cpu.register_y, 97);
}

#[test]
fn test_a9_5a_86() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5a]);
    cpu.register_a = 11;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 100;
    cpu.register_x = 206;
    cpu.register_y = 126;

    cpu.run();

    assert_eq!(cpu.register_a, 90);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 100);
    assert_eq!(cpu.register_x, 206);
    assert_eq!(cpu.register_y, 126);
}

#[test]
fn test_a9_b7_87() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb7]);
    cpu.register_a = 41;
    cpu.status = 111;
    cpu.pc = 0x8000;
    cpu.register_s = 212;
    cpu.register_x = 37;
    cpu.register_y = 102;

    cpu.run();

    assert_eq!(cpu.register_a, 183);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 212);
    assert_eq!(cpu.register_x, 37);
    assert_eq!(cpu.register_y, 102);
}

#[test]
fn test_a9_2e_88() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2e]);
    cpu.register_a = 92;
    cpu.status = 230;
    cpu.pc = 0x8000;
    cpu.register_s = 230;
    cpu.register_x = 178;
    cpu.register_y = 232;

    cpu.run();

    assert_eq!(cpu.register_a, 46);
    assert_eq!(cpu.status, 100);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 230);
    assert_eq!(cpu.register_x, 178);
    assert_eq!(cpu.register_y, 232);
}

#[test]
fn test_a9_db_89() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xdb]);
    cpu.register_a = 65;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 63;
    cpu.register_x = 48;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 219);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 63);
    assert_eq!(cpu.register_x, 48);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_90_90() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x90]);
    cpu.register_a = 61;
    cpu.status = 227;
    cpu.pc = 0x8000;
    cpu.register_s = 143;
    cpu.register_x = 184;
    cpu.register_y = 154;

    cpu.run();

    assert_eq!(cpu.register_a, 144);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 143);
    assert_eq!(cpu.register_x, 184);
    assert_eq!(cpu.register_y, 154);
}

#[test]
fn test_a9_9d_91() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x9d]);
    cpu.register_a = 145;
    cpu.status = 226;
    cpu.pc = 0x8000;
    cpu.register_s = 113;
    cpu.register_x = 189;
    cpu.register_y = 234;

    cpu.run();

    assert_eq!(cpu.register_a, 157);
    assert_eq!(cpu.status, 224);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 113);
    assert_eq!(cpu.register_x, 189);
    assert_eq!(cpu.register_y, 234);
}

#[test]
fn test_a9_b4_92() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb4]);
    cpu.register_a = 84;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 244;
    cpu.register_x = 183;
    cpu.register_y = 239;

    cpu.run();

    assert_eq!(cpu.register_a, 180);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 244);
    assert_eq!(cpu.register_x, 183);
    assert_eq!(cpu.register_y, 239);
}

#[test]
fn test_a9_73_93() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x73]);
    cpu.register_a = 132;
    cpu.status = 229;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 147;
    cpu.register_y = 198;

    cpu.run();

    assert_eq!(cpu.register_a, 115);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 147);
    assert_eq!(cpu.register_y, 198);
}

#[test]
fn test_a9_27_94() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x27]);
    cpu.register_a = 56;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 17;
    cpu.register_x = 22;
    cpu.register_y = 118;

    cpu.run();

    assert_eq!(cpu.register_a, 39);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 17);
    assert_eq!(cpu.register_x, 22);
    assert_eq!(cpu.register_y, 118);
}

#[test]
fn test_a9_17_95() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x17]);
    cpu.register_a = 255;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 190;
    cpu.register_x = 152;
    cpu.register_y = 185;

    cpu.run();

    assert_eq!(cpu.register_a, 23);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 190);
    assert_eq!(cpu.register_x, 152);
    assert_eq!(cpu.register_y, 185);
}

#[test]
fn test_a9_14_96() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x14]);
    cpu.register_a = 249;
    cpu.status = 163;
    cpu.pc = 0x8000;
    cpu.register_s = 166;
    cpu.register_x = 210;
    cpu.register_y = 200;

    cpu.run();

    assert_eq!(cpu.register_a, 20);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 166);
    assert_eq!(cpu.register_x, 210);
    assert_eq!(cpu.register_y, 200);
}

#[test]
fn test_a9_fc_97() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfc]);
    cpu.register_a = 38;
    cpu.status = 105;
    cpu.pc = 0x8000;
    cpu.register_s = 237;
    cpu.register_x = 117;
    cpu.register_y = 2;

    cpu.run();

    assert_eq!(cpu.register_a, 252);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 237);
    assert_eq!(cpu.register_x, 117);
    assert_eq!(cpu.register_y, 2);
}

#[test]
fn test_a9_48_98() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x48]);
    cpu.register_a = 242;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 86;
    cpu.register_x = 98;
    cpu.register_y = 75;

    cpu.run();

    assert_eq!(cpu.register_a, 72);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 86);
    assert_eq!(cpu.register_x, 98);
    assert_eq!(cpu.register_y, 75);
}

#[test]
fn test_a9_02_99() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x02]);
    cpu.register_a = 119;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 224;
    cpu.register_x = 167;
    cpu.register_y = 2;

    cpu.run();

    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc, 0x8001);
    assert_eq!(cpu.register_s, 224);
    assert_eq!(cpu.register_x, 167);
    assert_eq!(cpu.register_y, 2);
}

// CMP

#[test]
fn test_cmp_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC9, 0x42]); // CMP immediate with value 0x42
    cpu.register_a = 0x50; // Set A register to 0x50
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC5, 0x10]); // CMP zero page with address 0x10
    cpu.mem_write(0x0010, 0x20); // Set value at address 0x10 to 0x30
    cpu.register_a = 0x20; // Set A register to 0x20
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), true);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xD5, 0x10]); // CMP zero page,X with address 0x10
    cpu.register_x = 0x03; // Set X register to 0x03
    cpu.mem_write(0x0013, 0x25); // Set value at address 0x13 (0x10 + 0x03) to 0x25
    cpu.register_a = 0x30; // Set A register to 0x30
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xCD, 0x34, 0x12]); // CMP absolute with address 0x1234
    cpu.mem_write(0x1234, 0x60); // Set value at address 0x1234 to 0x60
    cpu.register_a = 0x70; // Set A register to 0x70
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xDD, 0x34, 0x12]); // CMP absolute,X with address 0x1234
    cpu.register_x = 0x02; // Set X register to 0x02
    cpu.mem_write(0x1236, 0x60); // Set value at address 0x1236 (0x1234 + 0x02) to 0x70
    cpu.register_a = 0x60; // Set A register to 0x60
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), true);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xD9, 0x34, 0x12]); // CMP absolute,Y with address 0x1234
    cpu.register_y = 0x01; // Set Y register to 0x01
    cpu.mem_write(0x1235, 0x80); // Set value at address 0x1235 (0x1234 + 0x01) to 0x80
    cpu.register_a = 0x90; // Set A register to 0x90
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), true);
}

#[test]
fn test_cmp_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC1, 0x10]); // CMP indirect,X with address 0x10
    cpu.register_x = 0x01; // Set X register to 0x01
    cpu.mem_write(0x000F, 0x34); // Set low byte of address (0x10 - 0x01) to 0x34
    cpu.mem_write(0x0010, 0x12); // Set high byte of address (0x10 - 0x01) to 0x12
    cpu.mem_write(0x1234, 0x70); // Set value at address 0x1234 to 0x70
    cpu.register_a = 0x60; // Set A register to 0x60
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cmp_indirect_y() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xD1, 0x10]); // CMP indirect,Y with address 0x10
    cpu.register_y = 0x02; // Set Y register to 0x02
    cpu.mem_write(0x0010, 0x34); // Set low byte of address 0x10 to 0x34
    cpu.mem_write(0x0011, 0x12); // Set high byte of address 0x10 to 0x12
    cpu.mem_write(0x1236, 0x60); // Set value at address 0x1236 (0x1234 + 0x02) to 0x70
    cpu.register_a = 0x60; // Set A register to 0x60
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because A >= value
    assert_eq!(cpu.get_flag("Z"), true);
    assert_eq!(cpu.get_flag("N"), false);
}

// CPY , CPX

#[test]
fn test_cpx_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xE0, 0x42]); // CPX immediate with value 0x42
    cpu.register_x = 0x50; // Set X register to 0x50
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because X >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cpx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xE4, 0x10]); // CPX zero page with address 0x10
    cpu.mem_write(0x0010, 0x30); // Set value at address 0x10 to 0x30
    cpu.register_x = 0x30; // Set X register to 0x20
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because X >= value
    assert_eq!(cpu.get_flag("Z"), true);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cpx_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xEC, 0x34, 0x12]); // CPX absolute with address 0x1234
    cpu.mem_write(0x1234, 0x60); // Set value at address 0x1234 to 0x60
    cpu.register_x = 0x70; // Set X register to 0x70
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because X >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cpy_immediate() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC0, 0x42]); // CPY immediate with value 0x42
    cpu.register_y = 0x50; // Set Y register to 0x50
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because Y >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cpy_zero_page() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xC4, 0x10]); // CPY zero page with address 0x10
    cpu.mem_write(0x0010, 0x30); // Set value at address 0x10 to 0x30
    cpu.register_y = 0x30; // Set Y register to 0x20
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because Y >= value
    assert_eq!(cpu.get_flag("Z"), true);
    assert_eq!(cpu.get_flag("N"), false);
}

#[test]
fn test_cpy_absolute() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xCC, 0x34, 0x12]); // CPY absolute with address 0x1234
    cpu.mem_write(0x1234, 0x60); // Set value at address 0x1234 to 0x60
    cpu.register_y = 0x70; // Set Y register to 0x70
    cpu.pc = 0x8000; // Set program counter
    cpu.run();
    assert_eq!(cpu.get_flag("C"), true); // Carry set because Y >= value
    assert_eq!(cpu.get_flag("Z"), false);
    assert_eq!(cpu.get_flag("N"), false);
}

// BEQ, BNE, BCC, BCS, BMI, BPL, BVC, BVS
