mod cpu;

use cpu::Cpu;
fn main() 
{
    let cpu = Cpu::new();
    println!("{}",cpu.fetch(4090));
}