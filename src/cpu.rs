//The CPU of the Chip8
pub struct Cpu
{
    // The Program Counter
    PC:u16,

    // The stack
    Stack:u16,

    // The chip8 memory
    RAM:Vec<u8>,

    // Registers (V0-VF)
    Registers:Vec<u8>,

    // Special Register
    I:u16
}

// All CPU methods
impl Cpu
{
    // Constructor
    pub fn new() -> Cpu
    {
        Cpu 
        {
            PC:0,
            Stack:0,
            RAM:vec![5;4096],
            Registers:vec![0;16],
            I:0
        }
    }

    // To run the cpu
    pub fn run(&self)
    {
        println!("Cpu is running...");
    }

    // Fetch current opcode pointed by the PC
    pub fn fetch(&self, adress:u16) -> u8
    {
        return self.RAM[adress as usize];
    }
}


