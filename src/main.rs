mod chip;
mod display;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use std::error::Error;
use std::time::Duration;

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
    let mut sdl_display_driver = display::Display::new()?;

    let mut chip = chip::Chip::new();
    chip.load_rom("roms/games/Tetris [Fran Dachille, 1991].ch8");

    loop {
        let quit = process_input(&mut chip.keypad, &sdl_display_driver.context);

        chip.cycle();

        sdl_display_driver.render(&mut chip, 15);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));

        if quit {
            break;
        }
    }

    Ok(())
}
