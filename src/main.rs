mod chip;

use chip::{VIDEO_HEIGHT, VIDEO_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;
use std::time::Duration;

const SCALE: u32 = 15;

impl chip::Chip {
    fn display_render(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (i, value) in self.video.iter().enumerate() {
            if *value != 0 {
                let x = (i % VIDEO_WIDTH as usize) as u32;
                let y = (i / VIDEO_WIDTH as usize) as u32;

                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }
        }
        canvas.present();
    }
}

fn process_input(keys: &mut [u8; 16], context: &Sdl) -> bool {
    let mut quit: bool = false;

    let mut event_pump = context.event_pump().unwrap();
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

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let title = "Chip8 emulator";
    let window_width = 1000;
    let window_height = 500;

    let window = video
        .window(title, window_width, window_height)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.clear();
    canvas.present();

    let mut chip = chip::Chip::new();
    chip.load_rom("roms/games/Tetris [Fran Dachille, 1991].ch8");

    let sleep_duration = Duration::from_millis(2);

    loop {
        let quit = process_input(&mut chip.keypad, &sdl_context);

        chip.cycle();
        println!("{:?}", chip.keypad);
        chip.display_render(&mut canvas);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        //std::thread::sleep(sleep_duration);

        if quit {
            break;
        }
    }

    Ok(())
}
