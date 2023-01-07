// Importing all useful modules
mod chip8;
use chip8::cpu::Cpu;
use chip8::gpu::gpu;

use piston::Event;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::input::keyboard;

/*// Pixel Size 
const SIZE_FACTOR:u32 = 4;

// If outputing log to console
const DEBUG:bool = false;

// Buttons
const BUTTONS:[keyboard::Key;16] = [
                                    keyboard::Key::NumPad0,
                                    keyboard::Key::NumPad1,
                                    keyboard::Key::NumPad2,
                                    keyboard::Key::NumPad3,
                                    keyboard::Key::NumPad4,
                                    keyboard::Key::NumPad5,
                                    keyboard::Key::NumPad6,
                                    keyboard::Key::NumPad7,
                                    keyboard::Key::NumPad8,
                                    keyboard::Key::NumPad9,
                                    keyboard::Key::A,
                                    keyboard::Key::B,
                                    keyboard::Key::C,
                                    keyboard::Key::D,
                                    keyboard::Key::E,
                                    keyboard::Key::F
                                   ];

// Main entry point
fn main() 
{ 
    // The instance of the CPU
    let mut cpu = Cpu::new();

    // The instance of the GPU
    let mut gpu = gpu::new(SIZE_FACTOR);

    // Handling events
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut gpu.window)
    {
        // Render graphics
        if let Some(args) = e.render_args() 
        {
            gpu.render(&args);
        }


        // Update graphics logic 
        if let Some(args) = e.update_args() 
        {
            gpu.update(cpu.getScreenBuffer());
        }

        // Keyboard events
        if let Event::Input(input, _) = e
        {
            if let Input::Button(button_args) = input
            {
                if let Button::Keyboard(key) = button_args.button
                {
                    for i in 0..BUTTONS.len()
                    {
                        if key == BUTTONS[i]
                        {
                            cpu.setKeys(i as usize, if button_args.state == ButtonState::Press {true} else {true});
                        }
                    }
                }
            }
        }


        // CPU step
        cpu.run();

        // For debugging
        if DEBUG == true
        {
            cpu.debuggerStep();
        }
    }
}*/

use minifb::MENU_KEY_CTRL;
use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use std::env;
use std::*;
use rfd;

// Pixel Size 
const SIZE_FACTOR:u32 = 4;

const WIDTH: usize = 64 * SIZE_FACTOR as usize;
const HEIGHT: usize = 32 * SIZE_FACTOR as usize;

const OTHER_MENU_ID: usize = 2;
const COLOR_0_ID: usize = 3;
const COLOR_1_ID: usize = 4;
const COLOR_2_ID: usize = 5;
const CLOSE_MENU_ID: usize = 6;

struct KeyCharCallback;

impl InputCallback for KeyCharCallback {
    fn add_char(&mut self, c: u32) {
        println!("add_char {}", c);
    }
}

fn main() 
{

    let mut filePath:String = String::from("");
    let mut start:bool = false;

    let mut cpu:Cpu = Cpu::new(String::from(""));
    // let mut gpu:gpu;

    // The buffer to display in window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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
        // // Updating buffer values and render it
        // for y in 0..HEIGHT 
        // {
        //     for x in 0..WIDTH 
        //     {
        //         buffer[(y * WIDTH) + x] = 0x00FF00 as u32;
        //     }
        // }

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
                            filePath = res.as_path().display().to_string();
                            // The instance of the CPU
                            let s = filePath.clone();
                            cpu = Cpu::new(s);

                            // The instance of the GPU
                            // gpu = gpu::new(SIZE_FACTOR);

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

        if start == true
        {
            // CPU step
            cpu.run();
        }
        
        window.get_keys().iter().for_each(|key| match key 
        {
            Key::W => println!("holding w!"),
            Key::T => println!("holding t!"),
            _ => (),
        });

        // We unwrap here as we want this code to exit if it fails
        let buff = cpu.getScreenBufferAsVec();
        window.update_with_buffer(&buff, 64, 32).unwrap();
    }
}
