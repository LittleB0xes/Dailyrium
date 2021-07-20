use crate::engine::Action;

pub struct LivingEntity {
    pub x: i32,
    pub y: i32,
    pub glyph: u16,
    pub action: Action,
}

impl LivingEntity {
    pub fn new(x: i32, y: i32) -> LivingEntity {
        LivingEntity {
            x,
            y,
            glyph: '@' as u16,
            action: Action::WAITING,
        }
    }

    pub fn move_entity(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
