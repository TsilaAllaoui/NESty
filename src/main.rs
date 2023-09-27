use cpu::Cpu;

mod cpu;
mod cpu_implementations;
mod opcode;

fn main() {
    let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut cpu = Cpu::new();
    cpu.load_and_run(&program);
}

#[cfg(test)]
mod tests;
