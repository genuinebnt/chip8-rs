#[allow(arithmetic_overflow)]
mod chip;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::Sdl;
use std::time::Duration;

fn render(canvas: &mut WindowCanvas, texture: &mut Texture, buffer: &[u8; 4096], pitch: usize) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    texture.update(None, buffer, pitch).expect("cannot update texture");
    canvas.copy(texture, None, None).unwrap();
    canvas.present();
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let title = "Chip8 emulator";
    let window_width = 640;
    let window_height = 480;
    let texture_width = 640;
    let texture_height = 480;

    let window = video
        .window(title, window_width, window_height)
        .position_centered()
        .build()
        .expect("Cannot initialize a window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("cannot create a canvas");

    let texture_creator = canvas.texture_creator();
    let surface = Surface::new(texture_width, texture_height, PixelFormatEnum::RGB888).unwrap();

    let mut texture = Texture::from_surface(&surface, &texture_creator).unwrap();

    let mut chip = chip::Chip::new();
    chip.load_rom("./test_opcode.ch8");


    loop {
        let quit = process_input(&mut chip.keypad, &sdl_context);

        println!("Ran");

        chip.cycle();

        render(
            &mut canvas,
            &mut texture,
            &chip.memory,
            chip::VIDEO_WIDTH as usize,
        );

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));

        if quit {
            break;
        }
    }

    Ok(())
}
