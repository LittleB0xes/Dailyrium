use tetra::graphics::Color;
use crate::dailyrium::{Sprite, Action, EntityType};
use crate::engine::give_id;

pub struct LivingEntity {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
    pub action: Action,
}

impl LivingEntity {
    pub fn new(x: i32, y: i32, t: EntityType) -> LivingEntity {
        let sprite = Sprite{
            glyph: '@' as u16,
            fg_color: Color::rgba8(255,255,255,255),
            bg_color: Color::rgba8(0,0,0,255),
        
        };
        let mut entity = LivingEntity {
            id: give_id(),
            x,
            y,
            sprite,
            action: Action::Waiting,
        };

        match t {
            EntityType::Hero => entity.breed_a_hero(),
            EntityType::Zombie => entity.breed_a_zombie(),
        }



        entity
    }

    fn breed_a_hero(&mut self) {
        self.sprite.glyph = '@' as u16;

    }
    fn breed_a_zombie(&mut self) {
        self.sprite.glyph = 'Z' as u16;
        self.sprite.fg_color = Color::rgb8(0,0,255);
        self.sprite.bg_color = Color::rgb8(0,0,0);
    }

    pub fn move_entity(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
