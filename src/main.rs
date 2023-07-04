// Importing all useful modules
mod chip8;
use chip8::cpu::Cpu;

use minifb::{Key, Menu, Scale, Window, WindowOptions};
use rfd;
use std::env;
use std::*;

// Pixel Size
const SIZE_FACTOR: u32 = 4;

// Window size
const WIDTH: usize = 64 * SIZE_FACTOR as usize;
const HEIGHT: usize = 32 * SIZE_FACTOR as usize;

// Main entry point
fn main() {
    // Checking args
    let args: Vec<String> = env::args().collect();

    // The keys for input
    let mut keys: [bool; 16] = [false; 16];

    // To know whether a Rom file is loaded or not
    let mut start: bool = false;

    // Instance of the CPU
    let mut cpu: Cpu = Cpu::new(None);
    if args.len() == 2 {
        cpu = Cpu::new(Some(args[1].to_string()));
    }

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
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Menu clicked event
        if let Some(menu_id) = window.is_menu_pressed() {
            match menu_id {
                // Selecting Rom File
                0 => {
                    let path = env::current_dir().unwrap();
                    if let Some(res) = rfd::FileDialog::new()
                        .add_filter("Chip8 Rom File", &["ch8"])
                        .set_directory(&path)
                        .pick_file()
                    {
                        // Creating new CPU instance with the Rom file loaded in
                        cpu = Cpu::new(Some(res.as_path().display().to_string()));
                        start = true;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => (),
            }
        }

        // If a Rom file is loaded in the memory, step the CPU
        if start == true {
            cpu.run();
        }

        // Get pressed keys
        let mut was_pressed = false;
        window.get_keys().iter().for_each(|key| match key {
            Key::NumPad0 => {
                keys[0] = true;
                was_pressed = true;
            }
            Key::NumPad1 => {
                keys[1] = true;
                was_pressed = true;
            }
            Key::NumPad2 => {
                keys[2] = true;
                was_pressed = true;
            }
            Key::NumPad3 => {
                keys[3] = true;
                was_pressed = true;
            }
            Key::NumPad4 => {
                keys[4] = true;
                was_pressed = true;
            }
            Key::NumPad5 => {
                keys[5] = true;
                was_pressed = true;
            }
            Key::NumPad6 => {
                keys[6] = true;
                was_pressed = true;
            }
            Key::NumPad7 => {
                keys[7] = true;
                was_pressed = true;
            }
            Key::NumPad8 => {
                keys[8] = true;
                was_pressed = true;
            }
            Key::NumPad9 => {
                keys[9] = true;
                was_pressed = true;
            }
            Key::A => {
                keys[10] = true;
                was_pressed = true;
            }
            Key::B => {
                keys[11] = true;
                was_pressed = true;
            }
            Key::C => {
                keys[12] = true;
                was_pressed = true;
            }
            Key::D => {
                keys[13] = true;
                was_pressed = true;
            }
            Key::E => {
                keys[14] = true;
                was_pressed = true;
            }
            Key::F => {
                keys[15] = true;
                was_pressed = true;
            }
            _ => (),
        });

        if was_pressed == true {
            cpu.update_keys(keys);
            // continue;
        }

        // Get released keys
        let mut was_released = false;
        window.get_keys_released().iter().for_each(|key| match key {
            Key::NumPad0 => {
                keys[0] = false;
                was_released = true;
            }
            Key::NumPad1 => {
                keys[1] = false;
                was_released = true;
            }
            Key::NumPad2 => {
                keys[2] = false;
                was_released = true;
            }
            Key::NumPad3 => {
                keys[3] = false;
                was_released = true;
            }
            Key::NumPad4 => {
                keys[4] = false;
                was_released = true;
            }
            Key::NumPad5 => {
                keys[5] = false;
                was_released = true;
            }
            Key::NumPad6 => {
                keys[6] = false;
                was_released = true;
            }
            Key::NumPad7 => {
                keys[7] = false;
                was_released = true;
            }
            Key::NumPad8 => {
                keys[8] = false;
                was_released = true;
            }
            Key::NumPad9 => {
                keys[9] = false;
                was_released = true;
            }
            Key::A => {
                keys[10] = false;
                was_released = true;
            }
            Key::B => {
                keys[11] = false;
                was_released = true;
            }
            Key::C => {
                keys[12] = false;
                was_released = true;
            }
            Key::D => {
                keys[13] = false;
                was_released = true;
            }
            Key::E => {
                keys[14] = false;
                was_released = true;
            }
            Key::F => {
                keys[15] = false;
                was_released = true;
            }
            _ => (),
        });

        if was_released == true {
            cpu.update_keys(keys);
            // continue;
        }

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&(cpu.get_screen_buffer_as_vec()), 64, 32)
            .expect("Failed to update window with buffer!");
    }
}
