// Importing all useful modules
mod chip8;
use chip8::cpu::Cpu;
use chip8::gpu::Gpu;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};

use std::env;

// Pixel Size
const SIZE_FACTOR: u32 = 4;

// If outputing log to console
const DEBUG: bool = false;

// Main entry point
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No rom file specified as argument!");
    }

    // The instance of the CPU
    let mut cpu = Cpu::new(Some(args[1].clone()));

    // The instance of the GPU
    let mut gpu = Gpu::new(SIZE_FACTOR);

    // Handling events
    let mut events = Events::new(EventSettings::new());

    let mut start = true;

    while let Some(e) = events.next(&mut gpu.window) {
        // if start {
        // Render graphics
        if let Some(args) = e.render_args() {
            gpu.render(&args);
            start = false;
        }
        // }

        // if start {
        // Update graphics logic
        if let Some(_args) = e.update_args() {
            gpu.update(cpu.get_screen_buffer());
        }
        // start = false;
        // }

        // CPU step
        cpu.run();

        // Checking if there is change to the screen buffer
        let data_changes = cpu.get_screen_changes_data();
        if data_changes != (0, 0, 0, vec![]) {
            if let Some(args) = e.render_args() {
                gpu.update_part(&data_changes, &args);
            }
            println!("{:?}", data_changes);
            cpu.set_screen_changes_data((0, 0, 0), vec![]);
        }

        // For debugging
        if DEBUG == true {
            // cpu.debuggerStep();
        }
    }
}
