use tetra::graphics::Color;
use crate::dailyrium::{Sprite, Action};
use crate::engine::give_id;
use crate::elements::Element;

#[derive(Eq, PartialEq)]
pub enum EntityType {
	Hero,
	Zombie,
}

pub struct LivingEntity {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
    pub action: Action,
    pub nature: EntityType,
    pub inventory: Vec<Element>,
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
            nature: EntityType::Hero,
            inventory: Vec::new(),
        };

        match t {
            EntityType::Hero => entity.breed_a_hero(),
            EntityType::Zombie => entity.breed_a_zombie(),
        }



        entity
    }

    fn breed_a_hero(&mut self) {
        self.sprite.glyph = '@' as u16;
        self.nature = EntityType::Hero;

    }
    fn breed_a_zombie(&mut self) {
        self.sprite.glyph = 'Z' as u16;
        self.sprite.fg_color = Color::rgb8(0,0,255);
        self.sprite.bg_color = Color::rgb8(0,0,0);
        self.nature = EntityType::Zombie;
    }

    pub fn move_entity(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn _take_item(&mut self, item: Element) {
        self.inventory.push(item);
    }
}
