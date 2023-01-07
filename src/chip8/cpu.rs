// Importing useful modules
use std::{io::Read, ops::Add, vec};
use rand::{Rng, thread_rng};

// Colors
const BLACK:u32 = 0;
const WHITE:u32 = 0xFFFFFF;

// The CPU of the Chip8
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

    // The screen buffer
    screenBuffer:Vec<Vec<u32>>,

    // Timers
    DT:u16,
    ST:u16,

    // Keys
    keys:[bool;16]
}

// All CPU methods
impl Cpu
{
    // Constructor
    pub fn new(romFile:String) -> Cpu
    {
        let mut rom:std::fs::File;
        if romFile == "" 
        {
            // Read rom file
            rom = match std::fs::File::open("C:/Users/Allaoui/Desktop/RustyChip8/roms/maze.ch8") 
            // let mut rom = match std::fs::File::open("C:/Users/735/Desktop/RustyChip8/roms/Particle Demo [zeroZshadow, 2008].ch8") 
            {
                // If the file was successfully opened, return the file object
                Ok(rom) => rom,

                // If there was an error opening the file, handle the error
                Err(error) => 
                { 
                    eprint!("Error: {}", error);
                    std::process::exit(1);
                }
            };
        }
        else 
        {
            // Read rom file
            rom = match std::fs::File::open(romFile) 
            // let mut rom = match std::fs::File::open("C:/Users/735/Desktop/RustyChip8/roms/Particle Demo [zeroZshadow, 2008].ch8") 
            {
                // If the file was successfully opened, return the file object
                Ok(rom) => rom,

                // If there was an error opening the file, handle the error
                Err(error) => 
                { 
                    eprint!("Error: {}", error);
                    std::process::exit(1);
                }
            };
        }

        // Reading rom file byte per byte to vector
        let mut buffer:Vec<u8> = Vec::new();
        rom.read_to_end(&mut buffer);

        let mut _RAM:Vec<u8> = vec![0;4096];
        let mut i = 0x200;
        for value in buffer
        {
            _RAM[i] = value;
            i = i + 1;
        }

        // Loading fonts to memory
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

        // Creating new instance of a CPU from all these parameters
        let _cpu = Cpu 
        {
            PC:0x200,
            SP:0,
            Stack:vec![0],
            RAM:_RAM,
            Registers:vec![0;16],
            I:0,
            currOpcode:0,
            screenBuffer:vec![vec![0;32];68],
            DT:0,
            ST:0,
            keys:[false;16]
        };
        
        return _cpu;
    }

    // A CPU step
    pub fn run(&mut self)
    {
        // Fetch
        let pc = self.PC;
        self.fetch(pc);

        // Decode and Execute
        self.decodeAndExecute();
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

           // For the 0x2FFF opcode
           0x2000 => self.CALL(self.currOpcode & 0x0FFF),

           // For the 0x3FFF opcodes
           0x3000 => self.SE_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x4FFF opcode
           0x4000 => self.SNE_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x4FFF opcode
           0x5000 => self.SE_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),

           // For the 0x6FFF opcodes
           0x6000 => self.LD_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x7FFF opcodes
           0x7000 => self.ADD_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0x8FFF opcode
           0x8000 => 
           {
                match self.currOpcode & 0x000F
                {
                    0x0000 => self.LD_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0001 => self.OR_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0002 => self.AND_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0003 => self.XOR_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0004 => self.ADD_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0005 => self.SUB_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x0006 => self.SHR_Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                    0x0007 => self.SUBN_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),
                    0x000E => self.SHL_Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                    _ =>
                    {
                        println!("Not implemented opcode: {:#04x}", self.currOpcode);
                        std::process::exit(1);
                    } 
                }
           }

           // For the 0x9FFF opcode
           0x9000 => self.SNE_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8, ((self.currOpcode & 0x00F0) >> 4) as u8),

           // For the 0xAFFF opcodes
           0xA000 => self.LD_I(self.currOpcode & 0x0FFF),

           // For the 0xBFFF opcodes
           0xB000 => self.JP_V0(self.currOpcode & 0x0FFF),

           // For the 0xCFFF opcodes
           0xC000 => self.RND_Vx(((self.currOpcode & 0x0F00) >> 8) as u8, (self.currOpcode & 0x00FF) as u8),

           // For the 0xDFFF opcodes
           0xD000 => self.DRW_Vx_Vy(((self.currOpcode & 0x0F00) >> 8) as u8,
                                    ((self.currOpcode & 0x00F0) >> 4) as u8,
                                    (self.currOpcode & 0x000F) as u8),

            // For the 0xEFFF opcodes
           0xE000 => 
           {
            match self.currOpcode & 0x00F0 
            {
                0x0090 => self.SKP_vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                0x00A0 => self.SKNP_vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                _ =>
                {
                    println!("Not implemented opcode: {:#04x}", self.currOpcode);
                    std::process::exit(1);
                } 
            }
           }

            // For the 0xFFFF opcodes
            0xF000 => 
            {
                match self.currOpcode & 0x00F0 
                {
                    0x0000 =>
                    {
                        match self.currOpcode & 0x000F
                        {
                            0x0007 => self.LD_Vx_DT(((self.currOpcode & 0x0F00) >> 8) as u8), 
                            0x000A => self.LD_Vx_k(((self.currOpcode & 0x0F00) >> 8) as u8),
                            _ =>
                            {
                                println!("Not implemented opcode: {:#04x}", self.currOpcode);
                                std::process::exit(1);
                            }
                        } 
                    }
                    0x0010 =>
                    {
                        match self.currOpcode & 0x000F 
                        {
                            0x0005 => self.LD_DT_Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                            0x0008 => self.LD_ST_Vx(((self.currOpcode & 0x0F00) >> 8) as u8), 
                            0x000E => self.ADD_I_Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                            _ =>
                            {
                                println!("Not implemented opcode: {:#04x}", self.currOpcode);
                                std::process::exit(1);
                            } 
                        }
                    },

                    0x0020 => self.LD_F_Vx(),
                    0x0030 => self.LD_B_Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                    0x0050 => self.LD__I__Vx(((self.currOpcode & 0x0F00) >> 8) as u8),
                    0x0060 => self.LD__Vx__I(((self.currOpcode & 0x0F00) >> 8) as u8),

                    _ => 
                    {
                        println!("Not implemented opcode: {:#04x}", self.currOpcode);
                        std::process::exit(1);
                    } 
                }
            },

            _ => 
            {
                println!("Not implemented opcode: {:#04x}", self.currOpcode);
                std::process::exit(1);
            }
        }
    }

    // Debugger step
    pub fn debuggerStep(&self)
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

    // Fetch byte pointed by the PC
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
    fn CLS(&mut self)
    {
        self.screenBuffer = vec![vec![0;32];68]
    }

    // Return from subroutine
    fn RET(&mut self)
    {
        let currStackVal = self.Stack[(self.Stack.len() - 1) as usize];
        if currStackVal != 0
        {
            self.PC = currStackVal;
            self.Stack.pop();
        }
    }

    // Call subroutine at the given adress
    fn CALL(&mut self, adress:u16)
    {
        self.SP = self.SP + 1;
        self.Stack.push(self.PC);
        self.PC = adress;
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
        self.Registers[index as usize] = randomVal & val;
    }

    // Check if Vx is equal to val and increment PC by 2 if this is true
    fn SE_Vx(&mut self, index:u8, val:u8)
    {
        if self.Registers[index as usize] == val
        {
            self.PC = self.PC + 2;
        }
    }

    // Check if Vx is not equal to val and increment PC by 2 if this is true
    fn SNE_Vx(&mut self, index:u8, val:u8)
    {
        if self.Registers[index as usize] != val
        {
            self.PC = self.PC + 2;
        }
    }

    // Check if Vx is not equal to Vy and increment PC by 2 if this is true
    fn SE_Vx_Vy(&mut self, x:u8, y:u8)
    {
        if self.Registers[x as usize] == self.Registers[y as usize]
        {
            self.PC = self.PC + 2;
        }
    }

    // For drawing on screen
    fn DRW_Vx_Vy(&mut self,x:u8, y:u8, n:u8)
    {
        // Position where to begin rendering the curretn sprite
        let posx = self.Registers[x as usize];
        let posy = self.Registers[y as usize];

        // For checking collision (TODO: add collision check)
        self.Registers[15] = 0;

        // Looping througth hight
        for i in 0..n
        {
            // Getting the current byte pointed at I + current row
            let adress:u16 = self.I + i as u16;
            let byte = self.RAM[adress as usize];

            // Looping throught columns of 8 pixels (each bit is a pixel, 1 = black, 0 = white)
            for j in (0..8).rev()
            {
                let a = byte & (1 << j);
                if (a >> j) == 1
                {
                    self.screenBuffer[((7-j) + posx) as usize][(posy + i) as usize] = BLACK;
                }
                else 
                {
                    self.screenBuffer[((7-j) + posx) as usize][(posy + i) as usize] =  WHITE;
                }
            }
        }
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

    // Jump instruction
    fn JMP(&mut self, val:u16)
    {
        self.PC = val;
    }

    // Setting Vx to the value of DT
    fn LD_Vx_DT(&mut self, index:u8)
    {
        self.Registers[index as usize] = self.DT as u8;
    }

    // Wait for keypress ans set the value of the key to Vx
    fn LD_Vx_k(&mut self, index:u8)
    {
        let key = 0;
        self.Registers[index as usize] = key;
    }

    // Add the content of Vx to I and store it in I
    fn ADD_I_Vx(&mut self, index:u8)
    {
        self.I = self.I + self.Registers[index as usize] as u16;
    }

    // Store the content of Vx to DT
    fn LD_DT_Vx(&mut self, index:u8)
    {
        self.DT = self.Registers[index as usize] as u16;
    }

    // Store the content of Vx to DT
    fn LD_ST_Vx(&mut self, index:u8)
    {
        self.ST = self.Registers[index as usize] as u16;
    }

    // Add Vx and Vy and store it in Vx, Vf is set if overflow
    fn ADD_Vx_Vy(&mut self, x:u8, y:u8)
    {
        let val:u16 = self.Registers[x as usize] as u16 + self.Registers[y as usize] as u16;
        if val > 0xFF
        {
            self.Registers[15] = 1;
        }
        else
        {
            self.Registers[15] = 0;
        } 
        self.Registers[x as usize] = (val & 0xFF) as u8;
    } 

    // Loading value of Vy in Vx
    fn LD_Vx_Vy(&mut self, x:u8, y:u8)
    {
        self.Registers[x as usize] = self.Registers[y as usize];
    }

    // Logical OR value of Vy with Vx and store it in Vx
    fn OR_Vx_Vy(&mut self, x:u8, y:u8)
    {
        self.Registers[x as usize] = self.Registers[x as usize] | self.Registers[y as usize];
    }

    // Logical XOR value of Vy with Vx and store it in Vx
    fn XOR_Vx_Vy(&mut self, x:u8, y:u8)
    {
        self.Registers[x as usize] = self.Registers[x as usize] ^ self.Registers[y as usize];
    }

    // Logical AND value of Vy with Vx and store it in Vx
    fn AND_Vx_Vy(&mut self, x:u8, y:u8)
    {
        self.Registers[x as usize] = self.Registers[x as usize] & self.Registers[y as usize];
    }

    // Substract value of Vy to Vx and store it in Vx, set Vf accordingly
    fn SUB_Vx_Vy(&mut self, x:u8, y:u8)
    {
        if  self.Registers[x as usize] >  self.Registers[y as usize]
        {
            self.Registers[15] = 1;
        }
        else 
        {
            self.Registers[15] = 0;
        }

        self.Registers[x as usize] = self.Registers[x as usize] - self.Registers[y as usize];
    }

    // Substract value of Vx to Vy and store it in Vx, set Vf accordingly
    fn SUBN_Vx_Vy(&mut self, x:u8, y:u8)
    {
        if  self.Registers[x as usize] <  self.Registers[y as usize]
        {
            self.Registers[15] = 1;
        }
        else 
        {
            self.Registers[15] = 0;
        }
        
        self.Registers[x as usize] = self.Registers[y as usize] - self.Registers[x as usize];
    }

    // Shift right Vx
    fn SHR_Vx(&mut self, x:u8)
    {
        let val = self.Registers[x as usize] & 0b00000001;

        if val == 1
        {
            self.Registers[15] = 1;
        }

        else 
        {
            self.Registers[15] = 0;
        }
        
        self.Registers[x as usize] = self.Registers[x as usize] >> 1;
    }

    // Shift left Vx
    fn SHL_Vx(&mut self, x:u8)
    {
        let val = self.Registers[x as usize] & 0b10000000;

        if val == 1
        {
            self.Registers[15] = 1;
        }

        else 
        {
            self.Registers[15] = 0;
        }
        
        self.Registers[x as usize] = self.Registers[x as usize] << 1;
    }

    // Skip next instruction if Vx != Vy
    fn SNE_Vx_Vy(&mut self, x:u8, y:u8)
    {
        if self.Registers[x as usize] != self.Registers[y as usize]
        {
            self.PC = self.PC + 2;
        }
    }

    // Jump to location adress + V0
    fn JP_V0(&mut self, adress:u16)
    {
        self.PC = adress + self.Registers[0] as u16; 
    }

    // Skip next instruction if key with the value of Vx is pressed
    fn SKP_vx(&mut self, index:u8)
    {
        if self.keys[index as usize] == true
        {
            self.PC = self.PC + 2;
        }
        else
        {
            self.PC = self.PC - 2;
        } 
    }
    
    // Skip next instruction if key with the value of Vx is pressed
    fn SKNP_vx(&mut self, index:u8)
    {
        if self.keys[index as usize] == false
        {
            self.PC = self.PC + 2;
        }

        else
        {
            self.PC = self.PC - 2;
        }
    }

    // BDC
    fn LD_B_Vx(&mut self, index:u8)
    {
        self.RAM[self.I as usize] = (self.Registers[index as usize] / 100) as u8;
        self.RAM[(self.I + 1) as usize] = ((self.Registers[index as usize] % 100) / 10) as u8;
        self.RAM[(self.I + 2) as usize] = (self.Registers[index as usize] % 10) as u8;
    }

    // Copy regiters V0 to Vx values to memory starting at I
    fn LD__I__Vx(&mut self, index:u8)
    {
        for i in 0..(index+1)
        {
            self.RAM[(self.I + i as u16) as usize] = self.Registers[i as usize];
        }
    }

    // Load regiters V0 to Vx values from memory starting at I
    fn LD__Vx__I(&mut self, index:u8)
    {
        for i in 0..(index+1)
        {
            self.Registers[i as usize] = self.RAM[(self.I + i as u16) as usize];
        }
    }

    // Set keys
    pub fn setKeys(&mut self,i:usize, val:bool)
    {
        self.keys[i] = val;
    }

    // // Getting the current screen buffer
    // pub fn getScreenBuffer(&mut self) -> Vec<Vec<[f32;4]>>
    // {
    //     let mut tmpBuffer:Vec<Vec<[f32;4]>> = vec![vec![[0.1,0.1,0.1,1.0];32];64];
    //     for i in 0..64
    //     {
    //         for j in 0..32
    //         {
    //             tmpBuffer[i as usize][j as usize] = self.screenBuffer[i as usize][j as usize];
    //         }
    //     }
    //     return tmpBuffer;
    // }

    // Getting the current screen buffer
    pub fn getScreenBufferAsVec(&mut self) -> Vec<u32>
    {
        let mut tmpBuffer:Vec<u32> = vec![0;64 * 32];
        // let mut tmp:Vec<Vec<u32>> = self.screenBuffer.clone();
        // tmpBuffer = tmp
        //                 .into_iter()
        //                 .flatten()
        //                 .collect::<Vec<u32>>();
        for i in 0..32
        {
            for j in 0..64
            {
                tmpBuffer[i * 64 + j] = self.screenBuffer[j][i];
            }
        }
        return tmpBuffer;
    }

    // Update keys
    pub fn updateKeys(&mut self, keys:[bool;16])
    {
        self.keys = keys;
    }

    fn LD_F_Vx(&mut self)
    {
        // TODO
    }
}


