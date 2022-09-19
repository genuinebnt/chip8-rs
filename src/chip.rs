use rand::prelude::random;
use std::fmt::Debug;
use std::fs;

const START_ADDRESS: u16 = 0x200;
const FONT_SET_SIZE: u32 = 80;
const FONT_SET_START_ADDRESS: u32 = 0x50;
pub const VIDEO_WIDTH: u8 = 64;
pub const VIDEO_HEIGHT: u8 = 32;

#[derive(Debug)]
pub struct Chip {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video: [u32; VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize],
    pub opcode: u16,
}

impl Chip {
    pub fn new() -> Self {
        let font_set: [u8; FONT_SET_SIZE as usize] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let mut chip = Chip {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0,
        };

        for i in 0..FONT_SET_SIZE as usize {
            chip.memory[FONT_SET_START_ADDRESS as usize + i] = font_set[i];
        }

        chip
    }

    pub fn load_rom(&mut self, filename: &str) {
        let contents = fs::read(filename).unwrap();
        for i in 0..contents.len() {
            self.memory[START_ADDRESS as usize + i] = contents[i];
        }
    }

    pub fn cycle(&mut self) {
        let hi_byte = self.memory[self.pc as usize];
        let lo_byte = self.memory[self.pc as usize + 1];

        self.opcode = (hi_byte as u16) << 8 | lo_byte as u16;

        self.pc += 2;

        match (self.opcode & 0xF000) >> 12 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => self.op_00e0(),
                0x000e => self.op_00ee(),
                _ => unreachable!(),
            },
            0x0001 => self.op_1nnn(),
            0x0002 => self.op_2nnn(),
            0x0003 => self.op_3xkk(),
            0x0005 => self.op_5xy0(),
            0x0006 => self.op_6xkk(),
            0x0007 => self.op_7xkk(),
            0x0008 => match self.opcode & 0x000F {
                0x0000 => self.op_8xy0(),
                0x0001 => self.op_8xy1(),
                0x0002 => self.op_8xy2(),
                0x0003 => self.op_8xy3(),
                0x0004 => self.op_8xy4(),
                0x0005 => self.op_8xy5(),
                0x0006 => self.op_8xy6(),
                0x0007 => self.op_8xy7(),
                0x000e => self.op_8xye(),
                _ => unreachable!(),
            },
            0x0009 => self.op_9xy0(),
            0x000a => self.op_annn(),
            0x000b => self.op_bnnn(),
            0x000c => self.op_cxnn(),
            0x000d => self.op_dxyn(),
            0x000e | 0x000f => match self.opcode & 0x00FF {
                0x00a1 => self.op_exa1(),
                0x009e => self.op_ex9e(),
                0x0007 => self.op_fx07(),
                0x000a => self.op_fx0a(),
                0x0015 => self.op_fx15(),
                0x0018 => self.op_fx18(),
                0x001e => self.op_fx1e(),
                0x0029 => self.op_fx29(),
                0x0033 => self.op_fx33(),
                0x0055 => self.op_fx55(),
                0x0065 => self.op_fx65(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn op_00e0(&mut self) {
        self.video = [0; VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize];
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn op_1nnn(&mut self) {
        let address = self.opcode & 0xFFF;
        self.pc = address;
    }

    fn op_2nnn(&mut self) {
        let address = self.opcode & 0xFFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn op_3xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = byte;
    }

    fn op_7xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] += byte;
    }

    fn op_8xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    fn op_8xy1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    fn op_8xy2(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    fn op_8xy3(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    fn op_8xy4(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        let sum: u16 = (self.registers[vx as usize] + self.registers[vy as usize]) as u16;

        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = (sum & 0xFF) as u8;
    }

    fn op_8xy5(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] > self.registers[vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] += self.registers[vy as usize];
    }

    fn op_8xy6(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF] = self.registers[vx as usize] & 0x1;

        self.registers[vx as usize] >>= 1;
    }

    fn op_8xy7(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vy as usize] > self.registers[vx as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = self.registers[vy as usize] - self.registers[vx as usize];
    }

    fn op_8xye(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;

        self.registers[vx as usize] <<= 1;
    }

    fn op_9xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        let address = self.opcode & 0x0FFF;

        self.index = address;
    }

    fn op_bnnn(&mut self) {
        let address = self.opcode & 0x0FFF;

        self.pc = self.registers[0] as u16 + address;
    }

    fn op_cxnn(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = random::<u8>() & byte;
    }

    fn op_dxyn(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        let height = self.opcode & 0x000F;

        let x_pos = self.registers[vx as usize] % VIDEO_WIDTH;
        let y_pos = self.registers[vy as usize] % VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in 0..height as usize {
            let sprite_byte = self.memory[self.index as usize + row as usize];

            for col in 0..8 as usize {
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let screen_pixel = &mut self.video
                    [(y_pos as usize + row) * VIDEO_WIDTH as usize + (x_pos as usize + col)];

                if sprite_pixel == 1 {
                    if *screen_pixel == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }

                    *screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }

    fn op_ex9e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        let key = self.registers[vx as usize];

        if self.keypad[key as usize] == 1 {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        let key = self.registers[vx as usize];

        if !self.keypad[key as usize] == 1 {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[vx as usize] = self.delay_timer;
    }

    fn op_fx0a(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for (i, key) in self.keypad.iter().enumerate() {
            if *key == 1 {
                self.registers[vx as usize] = i as u8;
                return;
            }
        }

        self.pc -= 2;
    }

    fn op_fx15(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.delay_timer = self.registers[vx as usize];
    }

    fn op_fx18(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.sound_timer = self.registers[vx as usize];
    }

    fn op_fx1e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.index = self.index + self.registers[vx as usize] as u16;
    }

    fn op_fx29(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit = self.registers[vx as usize];

        self.index = (FONT_SET_START_ADDRESS + (5 * digit as u32)) as u16;
    }

    fn op_fx33(&mut self) {
        let vx: u8 = ((self.opcode & 0xF000) >> 8) as u8;

        let mut value = self.registers[vx as usize];

        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    fn op_fx55(&mut self) {
        let vx: u8 = ((self.opcode & 0xF000) >> 8) as u8;

        for i in 0..=vx as usize {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    fn op_fx65(&mut self) {
        let vx: u8 = ((self.opcode & 0xF000) >> 8) as u8;

        for i in 0..=vx as usize {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }
}
