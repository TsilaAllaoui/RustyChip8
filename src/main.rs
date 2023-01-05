mod chip8;
use chip8::cpu::Cpu;
use chip8::gpu::gpu;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

fn main() 
{
    let mut cpu = Cpu::new();
    let mut gpu = gpu::new(4);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut gpu.window) {
        if let Some(args) = e.render_args() {
            // app.render(&args);
        }

        if let Some(args) = e.update_args() {
            // app.update(&args);
        }

        cpu.run();
    }
}