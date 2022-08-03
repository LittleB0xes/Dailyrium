use macroquad::prelude::*;

pub struct Hero {
    pub x: u32,
    pub y: u32,
    pub glyph: u16,
}

impl Hero {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            glyph: '@' as u16,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.x -= 1;
        }
        else if is_key_pressed(KeyCode::Right) {
            self.x += 1;
        }
        else if is_key_pressed(KeyCode::Up) {
            self.y -= 1;
        }
        else if is_key_pressed(KeyCode::Down) {
            self.y += 1;
        }
    }
}