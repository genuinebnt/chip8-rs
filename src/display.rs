use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

use crate::chip::{Chip, VIDEO_WIDTH};

pub struct Display {
    pub context: Sdl,
    pub canvas: WindowCanvas,
}

impl Display {
    pub fn new() -> Result<Display, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let title = "Chip8 emulator";
        let window_width = 1000;
        let window_height = 500;

        let window = video_subsystem
            .window(title, window_width, window_height)
            .opengl()
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().accelerated().present_vsync().build()?;

        Ok(Display {
            context: sdl_context,
            canvas,
        })
    }

    pub fn render(&mut self, chip: &mut Chip, scale: u32) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (i, value) in chip.video.iter().enumerate() {
            if *value != 0 {
                let x = (i % VIDEO_WIDTH as usize) as u32;
                let y = (i / VIDEO_WIDTH as usize) as u32;

                let rect = Rect::new((x * scale) as i32, (y * scale) as i32, scale, scale);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }
}
