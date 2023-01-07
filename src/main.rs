// Importing all useful modules
mod chip8;
use chip8::cpu::Cpu;

use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use std::env;
use std::*;
use rfd;

// Pixel Size 
const SIZE_FACTOR:u32 = 4;

// Window size
const WIDTH: usize = 64 * SIZE_FACTOR as usize;
const HEIGHT: usize = 32 * SIZE_FACTOR as usize;

// Main entry point
fn main() 
{
    // The keys for input
    let mut keys:[bool;16] = [false;16]; 

    // To know whether a Rom file is loaded or not
    let mut start:bool = false;

    // Instance of the CPU
    let mut cpu:Cpu = Cpu::new(String::from(""));

    // Creating window
    let mut window = Window::new(
        "RustyChip8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    // Menu
    let mut menu = Menu::new("File").unwrap();

    menu.add_item("Open ROM File", 0)
        .shortcut(Key::O, 0)
        .build();

    // Adding the menu to window
    window.add_menu(&menu);
    if let Some(menus) = window.get_posix_menus() {
        println!("Menus {:?}", menus);
    }

    // Main Loop
    while window.is_open() && !window.is_key_down(Key::Escape) 
    {
        // Menu clicked event
        if let Some(menu_id) = window.is_menu_pressed() 
        {
            match menu_id 
            {
                // Selecting Rom File
                0 =>
                {
                    let path = env::current_dir().unwrap();
                    if let Some(res) = rfd::FileDialog::new()
                        .add_filter("Chip8 Rom File", &["ch8"])
                        .set_directory(&path)
                        .pick_file()
                    {
                        // Creating new CPU instance with the Rom file loaded in
                        cpu = Cpu::new(res.as_path().display().to_string());
                        start = true;
                    }
                    else 
                    {
                        std::process::exit(1);    
                    }
                }
                _ => (),
            }
        }

        // If a Rom file is loaded in the memory, step the CPU
        if start == true
        {
            cpu.run();
        }
        
        // Get pressed keys
        let mut wasPressed = false;
        window.get_keys().iter().for_each(|key| match key 
        {
            Key::NumPad0 => { keys[0] = true; wasPressed = true; },
            Key::NumPad1 => { keys[1] = true; wasPressed = true; },
            Key::NumPad2 => { keys[2] = true; wasPressed = true; },
            Key::NumPad3 => { keys[3] = true; wasPressed = true; },
            Key::NumPad4 => { keys[4] = true; wasPressed = true; },
            Key::NumPad5 => { keys[5] = true; wasPressed = true; },
            Key::NumPad6 => { keys[6] = true; wasPressed = true; },
            Key::NumPad7 => { keys[7] = true; wasPressed = true; },
            Key::NumPad8 => { keys[8] = true; wasPressed = true; },
            Key::NumPad9 => { keys[9] = true; wasPressed = true; },
            Key::A => { keys[10] = true ; wasPressed = true; },
            Key::B => { keys[11] = true ; wasPressed = true; },
            Key::C => { keys[12] = true ; wasPressed = true; },
            Key::D => { keys[13] = true ; wasPressed = true; },
            Key::E => { keys[14] = true ; wasPressed = true; },
            Key::F => { keys[15] = true ; wasPressed = true; },
            _ => (),
        });

        if wasPressed == true
        {
            cpu.updateKeys(keys);
            // continue;
        }


        // Get released keys
        let mut wasReleased = false;
        window.get_keys_released().iter().for_each(|key| match key 
        {
            Key::NumPad0 => { keys[0] = false; wasReleased = true; },
            Key::NumPad1 => { keys[1] = false; wasReleased = true; },
            Key::NumPad2 => { keys[2] = false; wasReleased = true; },
            Key::NumPad3 => { keys[3] = false; wasReleased = true; },
            Key::NumPad4 => { keys[4] = false; wasReleased = true; },
            Key::NumPad5 => { keys[5] = false; wasReleased = true; },
            Key::NumPad6 => { keys[6] = false; wasReleased = true; },
            Key::NumPad7 => { keys[7] = false; wasReleased = true; },
            Key::NumPad8 => { keys[8] = false; wasReleased = true; },
            Key::NumPad9 => { keys[9] = false; wasReleased = true; },
            Key::A => { keys[10] = false ; wasReleased = true; },
            Key::B => { keys[11] = false ; wasReleased = true; },
            Key::C => { keys[12] = false ; wasReleased = true; },
            Key::D => { keys[13] = false ; wasReleased = true; },
            Key::E => { keys[14] = false ; wasReleased = true; },
            Key::F => { keys[15] = false ; wasReleased = true; },
            _ => (),
        });

        if wasReleased == true
        {
            cpu.updateKeys(keys);
            // continue;
        }

         // We unwrap here as we want this code to exit if it fails
         window.update_with_buffer(&(cpu.getScreenBufferAsVec()), 64, 32).unwrap();
    }
}
