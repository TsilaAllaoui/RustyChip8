extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


// The graphic unit for the chip8
pub struct gpu
{
    sizeFactor:u8,
    BLACK: [f32; 4],
    WHITE: [f32; 4],
    pub window: Window,
    // use graphics::*;

    //     const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    //     const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    //     let square = rectangle::square(0.0, 0.0, 50.0);
    //     let (x, y) = (64.0 * 4.0  / 2.0, 32.0 * 4.0 / 2.0);

    //     let gl:opengl_graphics::GlGraphics;
    //     gl.draw(args.viewport(), |c, gl| {
    //         // Clear the screen.
    //         clear(GREEN, gl);

    //         let transform = c
    //             .transform
    //             .trans(x, y)
    //             .rot_rad(rotation)
    //             .trans(-25.0, -25.0);

    //         // Draw a box rotating around the middle of the screen.
    //         rectangle(RED, square, transform, gl);
        // });
}

impl gpu
{
    pub fn new(sizeFactor:u32) -> gpu
    {
        let width:u32 = 64 * sizeFactor;
        let heigth:u32 = 32 * sizeFactor;
        let mut _window:Window = WindowSettings::new("RustyChip8", [width, heigth])
                    .graphics_api(OpenGL::V3_2)
                    .exit_on_esc(true)
                    .build()
                    .unwrap();

        let _gpu = gpu
        {
            sizeFactor:4,
            BLACK: [0.0,0.0,0.0,1.0],
            WHITE: [1.0,1.0,1.0,1.0],
            window: _window
        };

        return _gpu;
    }
}