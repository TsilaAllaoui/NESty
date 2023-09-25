use nesty::cpu::Cpu;
fn main() {
    let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut cpu = Cpu::new();
    cpu.interpret(&program);
}
