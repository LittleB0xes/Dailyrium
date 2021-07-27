use tetra::graphics::Color;
use crate::dailyrium::{Sprite, Action};
use crate::engine::give_id;
use crate::elements::Element;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EntityType {
	Hero,
	Zombie,
}

#[derive(Copy, Clone)]
pub enum Behavior {
    Drunk,
    Sleep,
    PlayerControl
}

#[derive(Clone)]
pub struct LivingEntity {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
    pub nature: EntityType,
    pub inventory: Vec<Element>,
    pub behavior: Behavior,
    pub action: Action,
    pub view_range: u8,

    pub seen: bool,
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
            behavior: Behavior::Sleep,
            action: Action::Waiting,
            nature: EntityType::Hero,
            inventory: Vec::new(),
            view_range: 6,
            seen: false,
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
        self.behavior = Behavior::PlayerControl

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
