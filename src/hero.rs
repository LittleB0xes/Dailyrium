use macroquad::prelude::*;

use crate::architect::Stage;
use crate::dailyrium::utils::*;

pub struct Hero {
    pub x: i32,
    pub y: i32,
    dir_x: i32,
    dir_y: i32,
    pub glyph: u16,
    pub fg_color: Color,
    pub turn_finish: bool,
    path: Vec<(i32, i32)>,
    autorun: bool,
    pub climbing: Option<i32>,

}

impl Hero {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            dir_x: 0,
            dir_y: 0,
            glyph: '@' as u16,
            fg_color: WHITE,
            turn_finish: false,
            autorun: false,
            climbing: None,
            path: Vec::new(),
        }
    }

    pub fn update(&mut self, stage: &Stage) -> bool {

        let mut turn_finish = false;
        if !self.autorun {
            if is_key_pressed(KeyCode::Left) {
                self.dir_x = -1;
            } else if is_key_pressed(KeyCode::Right) {
                self.dir_x = 1;
            } else if is_key_pressed(KeyCode::Up) {
                self.dir_y = -1;
            } else if is_key_pressed(KeyCode::Down) {
                self.dir_y = 1;
            }
    
            let dest_x = self.x  + self.dir_x;
            let dest_y = self.y  + self.dir_y;
            if !(self.dir_x == 0 && self.dir_y == 0) && inside_rect(dest_x, dest_y, 0, 0, stage.width - 1, stage.height - 1)
                && stage.stage_map[(dest_x + dest_y * stage.width as i32 ) as usize].crossable
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
        }
        else {
            self.x = self.path[0].0;
            self.y = self.path[0].1;
            self.path.remove(0);
            if self.path.len() == 0 {
                self.autorun = false;
            }

            turn_finish = true;
        }
        

        turn_finish
    }

    pub fn set_autorun(&mut self, path: Vec<(i32, i32)>) {
        self.path = path;
        self.path.remove(0);
        self.autorun = true;
    }
}
