use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

use crate::chip::{Chip, VIDEO_WIDTH};

pub struct SdlDriver {
    pub context: Sdl,
    pub canvas: WindowCanvas,
}

impl SdlDriver {
    pub fn new() -> Result<SdlDriver, Box<dyn Error>> {
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

        Ok(SdlDriver {
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

    pub fn process_input(&self, keys: &mut [u8; 16]) -> bool {
        let mut quit: bool = false;

        let mut event_pump = self.context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => quit = true,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => keys[0] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Kp1),
                    ..
                } => keys[1] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Kp2),
                    ..
                } => keys[2] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Kp3),
                    ..
                } => keys[3] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => keys[4] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keys[5] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => keys[6] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keys[7] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keys[8] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keys[9] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => keys[10] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => keys[11] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Kp4),
                    ..
                } => keys[12] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => keys[13] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => keys[14] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => keys[15] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => quit = true,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => keys[0] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Kp1),
                    ..
                } => keys[1] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Kp2),
                    ..
                } => keys[2] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Kp3),
                    ..
                } => keys[3] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => keys[4] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => keys[5] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => keys[6] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => keys[7] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => keys[8] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => keys[9] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => keys[10] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => keys[11] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Kp4),
                    ..
                } => keys[12] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => keys[13] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => keys[14] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => keys[15] = 1,
                _ => {}
            }
        }
        quit
    }
}
