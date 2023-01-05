use std::{io::Read, ops::Add};
use rand::Rng;
use crate::gpu;
//The CPU of the Chip8
pub struct Cpu
{
    // The Program Counter 5PC)
    PC:u16,

    // The Stack Pointer (SP)
    SP:u8,

    // The stack
    Stack:Vec<u16>,

    // The chip8 memory
    RAM:Vec<u8>,

    // Registers (V0-VF)
    Registers:Vec<u8>,

    // Special Register
    I:u16,

    // The current opcode where the PC is
    currOpcode:u16,


}

// All CPU methods
impl Cpu
{
    // Constructor
    pub fn new() -> Cpu
    {
        // Read rom file
        let mut rom = match std::fs::File::open("C:/Users/735/Desktop/RustyChip8/roms/maze.ch8") 
        {
            Ok(rom) => rom, // If the file was successfully opened, return the file object
            Err(error) => 
            { // If there was an error opening the file, handle the error
                eprint!("Error: {}", error);
                std::process::exit(1);
            }
        };

        let mut buffer:Vec<u8> = Vec::new();
        rom.read_to_end(&mut buffer);

        let mut _RAM:Vec<u8> = vec![0;4096];
        let mut i = 0x200;
        for value in buffer
        {
            _RAM[i] = value;
            i = i + 1;
        }

        pub const CHIP8_FONT_SET: [u8; 80] = 
        [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        i = 0;
        for value in CHIP8_FONT_SET
        {
            _RAM[i] = value;
            i = i + 1;
        }

        let _cpu = Cpu 
        {
            PC:0x200,
            SP:0,
            Stack:vec![0;16],
            RAM:_RAM,
            Registers:vec![0;16],
            I:0,
            currOpcode:0
        };
        
        return _cpu;
    }

    // To run the cpu
    pub fn run(&mut self)
    {
        let pc = self.PC;
        self.fetch(pc);
        self.decodeAndExecute();
        self.debuggerStep();
    }

    // Fetch current opcode pointed by the PC
    pub fn decodeAndExecute(&mut self)
    {
        match self.currOpcode & 0xF000 
        {
           // For the 0x0FFF opcodes
           0x0000 =>
           {
                match self.currOpcode & 0x0FFF
                {
                    0x00E0 => self.CLS(),

                    0x00EE => self.RET(),
        
                    _ =>
                    {
                        self.PC = self.currOpcode;
                    }
                }
           }
           
           // For the 0x1FFF opcode
           0x1000 => self.JMP(self.currOpcode & 0x0FFF),

           // For the 0x3FFF opcodes
           0x3000 => self.SE_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x6FFF opcodes
           0x6000 => self.LD_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x7FFF opcodes
           0x7000 => self.ADD_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0xAFFF opcodes
           0xA000 => self.LD_I(self.currOpcode & 0x0FFF),

           // For the 0xCFFF opcodes
           0xC000 => self.RND_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0xDFFF opcodes
           0xD000 => self.DRW_Vx_Vy((self.currOpcode & 0x000F) as u8),

            _ => 
            {
                println!("Not implemented opcode: {:#04x}", self.currOpcode);
                std::process::exit(1);
            }
        }
    }

    // Debugger
    pub fn debuggerStep(&mut self)
    {
        println!("PC={:#04x}", self.PC);
        println!("SP={:#04x}", self.SP);
        println!("Opcode={:#04x}", self.currOpcode);
        println!("I={:#04x}", self.I);
        println!("Registers:");
        for i in 0..16
        {
            print!("{:#04x}  ", self.Registers[i as usize]);
        }
        println!("");
    }

    // Decode current opcode pointed by the PC
    pub fn fetch(&mut self, adress:u16)
    {
        self.currOpcode = ((((self.RAM[(adress) as usize]) as u16) << 8) as u16) | (self.RAM[(adress + 1) as usize] as u16);
        self.PC = self.PC + 2;
    }


    //Fetch a byte in memory
    pub fn fecthByte(&self, adress:u16) -> u8
    {
        return self.RAM[adress as usize];
    }

    // Clearing screen
    fn CLS(&self)
    {

    }

    // Return from subroutine
    fn RET(&mut self)
    {
        let currStackVal = self.Stack[self.SP as usize];
        if currStackVal != 0
        {
            self.PC = currStackVal;
        }
    }

    // Loading value in register
    fn LD_Vx(&mut self, index:u8, val:u8)
    {
        self.Registers[index as usize] = val;
    }

    // Loading value in special register I
    fn LD_I(&mut self, val:u16)
    {
        self.I = val;
    }

    // Storing random number ANDed with a value in register
    fn RND_Vx(&mut self, index:u8, val:u8)
    {
        let mut rng = rand::thread_rng();
        let randomVal:u8 = rng.gen_range(0, 255);
        self.Registers[index as usize] = 0x1;//randomVal & val;
    }

    // Check if Vx is equal to val and increment PC by 2
    fn SE_Vx(&mut self, index:u8, val:u8)
    {
        if self.Registers[index as usize] == val
        {
            self.PC = self.PC + 2;
        }
    }

    // For drawing on screen
    fn DRW_Vx_Vy(&mut self, n:u8)
    {

        



        // let mut bytes:Vec<u8> = vec![0, n];
        // for i in 0..(n+1)
        // {
        //     bytes.push(self.RAM[(self.I as u8 + i) as usize]);
        // }

        //TODO draw bytes
    }

    // Add val to current Vx and store it in Vx
    fn ADD_Vx(&mut self, index:u8, val:u8)
    {
        match (self.Registers[index as usize]).checked_add(val) {
            Some(v) => 
            {
                self.Registers[index as usize] = v;
            }
            None => 
            {
                println!("Overflow occured in ADD_Vx function!");
                println!("PC={:#04x}", self.PC - 2);
                println!("Opcode={:#04x}", self.currOpcode);
                std::process::exit(1);
            }
        };
    }

    fn JMP(&mut self, val:u16)
    {
        self.PC = val;
    }
}


