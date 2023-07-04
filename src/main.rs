// Importing all useful modules
mod chip8;
use chip8::cpu::Cpu;
use chip8::gpu::gpu;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};

// Pixel Size
const SIZE_FACTOR: u32 = 4;

// If outputing log to console
const DEBUG: bool = false;

// Main entry point
fn main() {
    // The instance of the CPU
    let mut cpu = Cpu::new(Some(String::from(
        "D:/FIT_Apprenti_Vague_006/RustyChip8/target/debug/Maze (alt) [David Winter, 199x].ch8",
    )));

    // The instance of the GPU
    let mut gpu = gpu::new(SIZE_FACTOR);

    // Handling events
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut gpu.window) {
        // Render graphics
        if let Some(args) = e.render_args() {
            gpu.render(&args);
        }

        // Update graphics logic
        if let Some(_args) = e.update_args() {
            gpu.update(cpu.get_scree_buffer());
        }

        // CPU step
        cpu.run();

        // For debugging
        if DEBUG == true {
            // cpu.debuggerStep();
        }
    }
}
