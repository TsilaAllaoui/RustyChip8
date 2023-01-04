mod cpu;

use cpu::Cpu;
fn main() 
{
    let mut cpu = Cpu::new();
    loop
    {
        cpu.run();
    }
}