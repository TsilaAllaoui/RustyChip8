// Importing useful modules
use rand::Rng;
use std::{fs, io::Read, vec};

// Colors
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

// The CPU of the Chip8
pub struct Cpu {
    // The Program Counter 5PC)
    pc: u16,

    // The Stack Pointer (SP)
    sp: u8,

    // The stack
    stack: Vec<u16>,

    // The chip8 memory
    ram: Vec<u8>,

    // Registers (v0-VF)
    registers: Vec<u8>,

    // Special Register
    i_register: u16,

    // The current opcode where the PC is
    curr_opcode: u16,

    //     // The screen buffer
    screen_buffer: Vec<Vec<[f32; 4]>>,

    // Timers
    delta_timer: u16,
    sub_timer: u16,

    // Keys
    keys: [bool; 16],
}

// All CPU methods
impl Cpu {
    // Constructor
    pub fn new(rom_file: Option<String>) -> Cpu {
        let mut rom: std::fs::File;
        let mut buffer: Vec<u8> = Vec::new();

        // If rom file arg is not empty
        if rom_file.is_some() {
            // Read rom file
            let value = rom_file.unwrap();
            rom = fs::File::open(&value).expect(&format!("Can't open rom file {}!", value));
            rom.read_to_end(&mut buffer).expect("Can't read rom file");
        }

        // Reading rom file byte per byte to vector
        let mut _ram: Vec<u8> = vec![0; 4096];
        let mut i = 0x200;
        for value in buffer {
            _ram[i] = value;
            i = i + 1;
        }

        // Loading fonts to memory
        pub const CHIP8_FONT_SET: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        i = 0;
        for value in CHIP8_FONT_SET {
            _ram[i] = value;
            i = i + 1;
        }

        // Creating new instance of a CPU from all these parameters
        let _cpu = Cpu {
            pc: 0x200,
            sp: 0,
            stack: vec![0],
            ram: _ram,
            registers: vec![0; 16],
            i_register: 0,
            curr_opcode: 0,
            screen_buffer: vec![vec![[0.1, 0.1, 0.1, 1.0]; 32]; 68],
            delta_timer: 0,
            sub_timer: 0,
            keys: [false; 16],
        };

        return _cpu;
    }

    // A CPU step
    pub fn run(&mut self) {
        // Fetch
        let pc = self.pc;
        self.fetch(pc);

        // Decode and Execute
        self.decode_and_execute();
    }

    // Fetch current opcode pointed by the PC
    pub fn decode_and_execute(&mut self) {
        match self.curr_opcode & 0xF000 {
            // For the 0x0FFF opcodes
            0x0000 => match self.curr_opcode & 0x0FFF {
                0x00E0 => self.cls(),

                0x00EE => self.ret(),

                _ => {
                    self.pc = self.curr_opcode;
                }
            },

            // For the 0x1FFF opcode
            0x1000 => self.jmp(self.curr_opcode & 0x0FFF),

            // For the 0x2FFF opcode
            0x2000 => self.call(self.curr_opcode & 0x0FFF),

            // For the 0x3FFF opcodes
            0x3000 => self.se_vx(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                (self.curr_opcode & 0x00FF) as u8,
            ),

            // For the 0x4FFF opcode
            0x4000 => self.sne_vx(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                (self.curr_opcode & 0x00FF) as u8,
            ),

            // For the 0x4FFF opcode
            0x5000 => self.se_vx_vy(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                ((self.curr_opcode & 0x00F0) >> 4) as u8,
            ),

            // For the 0x6FFF opcodes
            0x6000 => self.ld_vx(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                (self.curr_opcode & 0x00FF) as u8,
            ),

            // For the 0x7FFF opcodes
            0x7000 => self.add_vx(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                (self.curr_opcode & 0x00FF) as u8,
            ),

            // For the 0x8FFF opcode
            0x8000 => match self.curr_opcode & 0x000F {
                0x0000 => self.ld_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0001 => self.or_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0002 => self.and_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0003 => self.xor_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0004 => self.add_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0005 => self.sub_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x0006 => self.shr_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                0x0007 => self.subn_vx_vy(
                    ((self.curr_opcode & 0x0F00) >> 8) as u8,
                    ((self.curr_opcode & 0x00F0) >> 4) as u8,
                ),
                0x000E => self.shl_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                _ => {
                    println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                    std::process::exit(1);
                }
            },

            // For the 0x9FFF opcode
            0x9000 => self.sne_vx_vy(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                ((self.curr_opcode & 0x00F0) >> 4) as u8,
            ),

            // For the 0xAFFF opcodes
            0xA000 => self.ld_i(self.curr_opcode & 0x0FFF),

            // For the 0xBFFF opcodes
            0xB000 => self.jp_v0(self.curr_opcode & 0x0FFF),

            // For the 0xCFFF opcodes
            0xC000 => self.rnd_vx(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                (self.curr_opcode & 0x00FF) as u8,
            ),

            // For the 0xDFFF opcodes
            0xD000 => self.drw_vx_vy(
                ((self.curr_opcode & 0x0F00) >> 8) as u8,
                ((self.curr_opcode & 0x00F0) >> 4) as u8,
                (self.curr_opcode & 0x000F) as u8,
            ),

            // For the 0xEFFF opcodes
            0xE000 => match self.curr_opcode & 0x00F0 {
                0x0090 => self.skp_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                0x00A0 => self.sknp_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                _ => {
                    println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                    std::process::exit(1);
                }
            },

            // For the 0xFFFF opcodes
            0xF000 => match self.curr_opcode & 0x00F0 {
                0x0000 => match self.curr_opcode & 0x000F {
                    0x0007 => self.ld_vx_dt(((self.curr_opcode & 0x0F00) >> 8) as u8),
                    0x000A => self.ld_vx_k(((self.curr_opcode & 0x0F00) >> 8) as u8),
                    _ => {
                        println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                        std::process::exit(1);
                    }
                },
                0x0010 => match self.curr_opcode & 0x000F {
                    0x0005 => self.ld_dt_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                    0x0008 => self.ld_st_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                    0x000E => self.add_i_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                    _ => {
                        println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                        std::process::exit(1);
                    }
                },

                0x0020 => self.ld_f_vx(),
                0x0030 => self.ld_b_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                0x0050 => self.ld_i_vx(((self.curr_opcode & 0x0F00) >> 8) as u8),
                0x0060 => self.ld_vx_i(((self.curr_opcode & 0x0F00) >> 8) as u8),

                _ => {
                    println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                    std::process::exit(1);
                }
            },

            _ => {
                println!("Not implemented opcode: {:#04x}", self.curr_opcode);
                std::process::exit(1);
            }
        }
    }

    // Fetch byte pointed by the PC
    pub fn fetch(&mut self, adress: u16) {
        self.curr_opcode = ((((self.ram[(adress) as usize]) as u16) << 8) as u16)
            | (self.ram[(adress + 1) as usize] as u16);
        self.pc = self.pc + 2;
    }

    // Clearing screen
    fn cls(&mut self) {
        self.screen_buffer = vec![vec![[0.1, 0.1, 0.1, 1.0]; 32]; 68];
    }

    // Return from subroutine
    fn ret(&mut self) {
        let curr_stack_val = self.stack[(self.stack.len() - 1) as usize];
        if curr_stack_val != 0 {
            self.pc = curr_stack_val;
            self.stack.pop();
        }
    }

    // Call subroutine at the given adress
    fn call(&mut self, adress: u16) {
        self.sp = self.sp + 1;
        self.stack.push(self.pc);
        self.pc = adress;
    }

    // Loading value in register
    fn ld_vx(&mut self, index: u8, val: u8) {
        self.registers[index as usize] = val;
    }

    // Loading value in special register I
    fn ld_i(&mut self, val: u16) {
        self.i_register = val;
    }

    // Storing random number anded with a value in register
    fn rnd_vx(&mut self, index: u8, val: u8) {
        let mut rng = rand::thread_rng();
        let random_val: u8 = rng.gen_range(0..255);
        self.registers[index as usize] = random_val & val;
    }

    // Check if Vx is equal to val and increment PC by 2 if this is true
    fn se_vx(&mut self, index: u8, val: u8) {
        if self.registers[index as usize] == val {
            self.pc = self.pc + 2;
        }
    }

    // Check if Vx is not equal to val and increment PC by 2 if this is true
    fn sne_vx(&mut self, index: u8, val: u8) {
        if self.registers[index as usize] != val {
            self.pc = self.pc + 2;
        }
    }

    // Check if Vx is not equal to Vy and increment PC by 2 if this is true
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc = self.pc + 2;
        }
    }

    // For drawing on screen
    fn drw_vx_vy(&mut self, x: u8, y: u8, n: u8) {
        // Position where to begin rendering the curretn sprite
        let posx = self.registers[x as usize];
        let posy = self.registers[y as usize];

        // For checking collision (TODO: add collision check)
        self.registers[15] = 0;

        // Looping througth hight
        for i in 0..n {
            // Getting the current byte pointed at I + current row
            let adress: u16 = self.i_register + i as u16;
            let byte = self.ram[adress as usize];

            // Looping throught columns of 8 pixels (each bit is a pixel, 1 = black, 0 = white)
            for j in (0..8).rev() {
                let a = byte & (1 << j);
                if (a >> j) == 1 {
                    self.screen_buffer[((7 - j) + posx) as usize][(posy + i) as usize] = BLACK;
                } else {
                    self.screen_buffer[((7 - j) + posx) as usize][(posy + i) as usize] = WHITE;
                }
            }
        }
    }

    // Add val to current Vx and store it in Vx
    fn add_vx(&mut self, index: u8, val: u8) {
        match (self.registers[index as usize]).checked_add(val) {
            Some(v) => {
                self.registers[index as usize] = v;
            }
            None => {
                println!("Overflow occured in add_vx function!");
                println!("PC={:#04x}", self.pc - 2);
                println!("Opcode={:#04x}", self.curr_opcode);
                std::process::exit(1);
            }
        };
    }

    // Jump instruction
    fn jmp(&mut self, val: u16) {
        self.pc = val;
    }

    // Setting Vx to the value of dt
    fn ld_vx_dt(&mut self, index: u8) {
        self.registers[index as usize] = self.delta_timer as u8;
    }

    // Wait for keypress ans set the value of the key to Vx
    fn ld_vx_k(&mut self, index: u8) {
        let key = 0;
        self.registers[index as usize] = key;
    }

    // Add the content of Vx to I and store it in I
    fn add_i_vx(&mut self, index: u8) {
        self.i_register = self.i_register + self.registers[index as usize] as u16;
    }

    // Store the content of Vx to dt
    fn ld_dt_vx(&mut self, index: u8) {
        self.delta_timer = self.registers[index as usize] as u16;
    }

    // Store the content of Vx to dt
    fn ld_st_vx(&mut self, index: u8) {
        self.sub_timer = self.registers[index as usize] as u16;
    }

    // Add Vx and Vy and store it in Vx, Vf is set if overflow
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let val: u16 = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
        if val > 0xFF {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }
        self.registers[x as usize] = (val & 0xFF) as u8;
    }

    // Loading value of Vy in Vx
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    // Logical or value of Vy with Vx and store it in Vx
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    // Logical xor value of Vy with Vx and store it in Vx
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }

    // Logical and value of Vy with Vx and store it in Vx
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }

    // Substract value of Vy to Vx and store it in Vx, set Vf accordingly
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] > self.registers[y as usize] {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }

        self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
    }

    // Substract value of Vx to Vy and store it in Vx, set Vf accordingly
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] < self.registers[y as usize] {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }

        self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
    }

    // Shift right Vx
    fn shr_vx(&mut self, x: u8) {
        let val = self.registers[x as usize] & 0b00000001;

        if val == 1 {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }

        self.registers[x as usize] = self.registers[x as usize] >> 1;
    }

    // Shift left Vx
    fn shl_vx(&mut self, x: u8) {
        let val = self.registers[x as usize] & 0b10000000;

        if val == 1 {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }

        self.registers[x as usize] = self.registers[x as usize] << 1;
    }

    // Skip next instruction if Vx != Vy
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.pc = self.pc + 2;
        }
    }

    // Jump to location adress + v0
    fn jp_v0(&mut self, adress: u16) {
        self.pc = adress + self.registers[0] as u16;
    }

    // Skip next instruction if key with the value of Vx is pressed
    fn skp_vx(&mut self, index: u8) {
        if self.keys[index as usize] == true {
            self.pc = self.pc + 2;
        } else {
            self.pc = self.pc - 2;
        }
    }

    // Skip next instruction if key with the value of Vx is pressed
    fn sknp_vx(&mut self, index: u8) {
        if self.keys[index as usize] == false {
            self.pc = self.pc + 2;
        } else {
            self.pc = self.pc - 2;
        }
    }

    // BDC
    fn ld_b_vx(&mut self, index: u8) {
        self.ram[self.i_register as usize] = (self.registers[index as usize] / 100) as u8;
        self.ram[(self.i_register + 1) as usize] =
            ((self.registers[index as usize] % 100) / 10) as u8;
        self.ram[(self.i_register + 2) as usize] = (self.registers[index as usize] % 10) as u8;
    }

    // Copy regiters v0 to Vx values to memory starting at I
    fn ld_i_vx(&mut self, index: u8) {
        for i in 0..(index + 1) {
            self.ram[(self.i_register + i as u16) as usize] = self.registers[i as usize];
        }
    }

    // Load regiters v0 to Vx values from memory starting at I
    fn ld_vx_i(&mut self, index: u8) {
        for i in 0..(index + 1) {
            self.registers[i as usize] = self.ram[(self.i_register + i as u16) as usize];
        }
    }

    // Getting the current screen buffer
    pub fn get_scree_buffer(&mut self) -> Vec<Vec<[f32; 4]>> {
        let mut tmp_buffer: Vec<Vec<[f32; 4]>> = vec![vec![[0.1, 0.1, 0.1, 1.0]; 32]; 64];
        for i in 0..64 {
            for j in 0..32 {
                tmp_buffer[i as usize][j as usize] = self.screen_buffer[i as usize][j as usize];
            }
        }
        return tmp_buffer;
    }

    fn ld_f_vx(&mut self) {
        // TODO
    }
}
