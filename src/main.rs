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

// Key event
struct KeyCharCallback;

impl InputCallback for KeyCharCallback {
    fn add_char(&mut self, c: u32) {
        println!("add_char {}", c);
    }
}

// Main entry point
fn main() 
{
    // TO know whether a Rom file is loaded or not
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

    // Setting key callback
    window.set_input_callback(Box::new(KeyCharCallback {}));

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
        
        // Key event handler
        window.get_keys().iter().for_each(|key| match key 
        {
            Key::W => println!("holding w!"),
            Key::T => println!("holding t!"),
            _ => (),
        });

         // We unwrap here as we want this code to exit if it fails
         window.update_with_buffer(&(cpu.getScreenBufferAsVec()), 64, 32).unwrap();
    }
}
