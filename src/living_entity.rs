use bracket_lib::prelude::RGBA;
use bracket_lib::prelude::{BLUE, WHITE};
use crate::dailyrium::{Sprite, Action};
use crate::engine::give_id;

pub struct LivingEntity {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
    pub action: Action,
}

impl LivingEntity {
    pub fn new(x: i32, y: i32) -> LivingEntity {
        let sprite = Sprite{
            glyph: '@' as u16,
            fg_color: RGBA::named(WHITE),
            bg_color: RGBA::named(BLUE),
        
        };
        LivingEntity {
            id: give_id(),
            x,
            y,
            sprite,
            action: Action::WAITING,
        }
    }

    pub fn move_entity(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
