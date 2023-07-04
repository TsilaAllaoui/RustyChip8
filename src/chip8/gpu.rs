// Importing useful modules
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use piston::window::WindowSettings;

// The GPU of the chip8
pub struct Gpu {
    size_factor: u8,
    black: [f32; 4],
    pub window: Window,
    gl: GlGraphics,
    screen: Vec<Vec<[f32; 4]>>,
}

// GPU methods
impl Gpu {
    pub fn new(size_factor: u32) -> Gpu {
        // Creating window
        let width: u32 = 64 * size_factor;
        let heigth: u32 = 32 * size_factor;
        let mut _window: Window = WindowSettings::new("RustyChip8", [width, heigth])
            .graphics_api(OpenGL::V3_2)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Creating new instance of a GPU
        let _gpu = Gpu {
            size_factor: 4,
            black: [0.0, 0.0, 0.0, 1.0],
            window: _window,
            gl: GlGraphics::new(OpenGL::V3_2),
            screen: vec![vec![[0.5, 0.1, 0.71, 1.0]; 32]; 64],
        };

        return _gpu;
    }

    // Render screen buffer to window
    pub fn render(&mut self, &args: &RenderArgs) {
        use graphics::*;

        // A square here represents a pixel
        let square = rectangle::square(0.0, 0.0, self.size_factor as f64);

        // Rendering logic
        self.gl.draw(args.viewport(), |c, gl| {
            // Clearing the screen to black
            clear(self.black, gl);

            // Looping througth all pixel and render it
            for i in 0..64 {
                for j in 0..32 {
                    let x = (i * self.size_factor) as f64;
                    let y = (j * self.size_factor) as f64;
                    let transform = c.transform.trans(x, y);
                    rectangle(self.screen[i as usize][j as usize], square, transform, gl);
                }
            }
        });
    }

    // Update the current screen buffer of the GPU
    pub fn update(&mut self, screen_buffer: Vec<Vec<[f32; 4]>>) {
        self.screen = screen_buffer;
    }

    // Update part of screen from changes data
    pub fn update_part(&mut self, data: &(u8, u8, u8, Vec<u8>), &args: &RenderArgs) {
        use graphics::*;

        // A square here represents a pixel
        let square = rectangle::square(0.0, 0.0, self.size_factor as f64);

        let mut i = 0;

        // Looping througth height
        for byte in &data.3 {
            // Looping throught columns of 8 pixels (each bit is a pixel, 1 = black, 0 = white)
            for j in (0..8).rev() {
                let a = byte & (1 << j);
                let y = ((7 - j) + data.1) as f64;
                let x = (data.0 + i) as f64;
                if (a >> j) == 1 {
                    self.gl.draw(args.viewport(), |c, gl| {
                        // Looping througth all pixel and render it
                        let transform = c
                            .transform
                            .trans(x * self.size_factor as f64, y * self.size_factor as f64);
                        rectangle([0.0, 0.0, 0.0, 0.0], square, transform, gl);
                    });
                } else {
                    self.gl.draw(args.viewport(), |c, gl| {
                        // Looping througth all pixel and render it
                        let transform = c
                            .transform
                            .trans(x * self.size_factor as f64, y * self.size_factor as f64);
                        rectangle([1.0, 1.0, 1.0, 1.0], square, transform, gl);
                    });
                }
            }
            i += 1;
        }
    }
}
