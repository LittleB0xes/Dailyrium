use macroquad::prelude::*;

use crate::architect::Stage;
use crate::dailyrium::utils::*;

pub struct Hero {
    pub x: u32,
    pub y: u32,
    dir_x: i32,
    dir_y: i32,
    pub glyph: u16,
    pub fg_color: Color,
    pub turn_finish: bool,
}

impl Hero {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            dir_x: 0,
            dir_y: 0,
            glyph: '@' as u16,
            fg_color: WHITE,
            turn_finish: false,
        }
    }

    pub fn update(&mut self, stage: &Stage) -> bool {
        let mut turn_finish = false;
        if is_key_pressed(KeyCode::Left) {
            self.dir_x = -1;
        } else if is_key_pressed(KeyCode::Right) {
            self.dir_x = 1;
        } else if is_key_pressed(KeyCode::Up) {
            self.dir_y = -1;
        } else if is_key_pressed(KeyCode::Down) {
            self.dir_y = 1;
        }

        let dest_x = (self.x as i32 + self.dir_x) as u32;
        let dest_y = (self.y as i32 + self.dir_y) as u32;
        if !(self.dir_x == 0 && self.dir_y == 0) && inside_rect(dest_x, dest_y, 0, 0, stage.width - 1, stage.height - 1)
            && stage.stage_map[(dest_x + dest_y * stage.width) as usize].crossable
        {
            self.x = dest_x;
            self.y = dest_y;
            turn_finish = true;
        } else {
            self.dir_x = 0;
            self.dir_y = 0;
        }

        // Reset direction after turn
        self.dir_x = 0;
        self.dir_y = 0;

        turn_finish
    }
}
