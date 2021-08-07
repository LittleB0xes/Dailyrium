use tetra::graphics::Color;
use crate::dailyrium::{Sprite, Action};
use crate::engine::give_id;
use crate::elements::items::{Item, ItemType};

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
    pub inventory: Vec<Item>,
    pub behavior: Behavior,
    pub action: Action,

    // Entity stat
    pub stealth: u8,
    pub view_range: u8,

    pub seen: bool,
}

impl LivingEntity {
    pub fn new(x: i32, y: i32, t: EntityType) -> LivingEntity {
        let sprite = Sprite{
            glyph: '@' as u16,
            fg_color: Color::rgba8(255,255,255,255),
            bg_color: Color::rgba8(0,0,0,255),
            fade: 0.0
        
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
            stealth: 6,
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
        self.behavior = Behavior::PlayerControl;
        self.view_range = 8;

    }
    fn breed_a_zombie(&mut self) {
        self.sprite.glyph = 'Z' as u16;
        self.sprite.fg_color = Color::rgba8(0,0,255, 255);
        self.sprite.bg_color = Color::rgba8(0,0,0, 255);
        self.nature = EntityType::Zombie;
    }

    pub fn move_entity(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
    
    pub fn add_to_inventory(&mut self, item: Item) -> bool {
        match item.nature {
            ItemType::Gold => {
                let gold_pos = self.inventory.iter().position(|it| it.nature == ItemType::Gold);
                match gold_pos {
                    Some(pos) => {
                        let old_gold: u64 = self.inventory[pos].data;
                        let new_gold: u64 = item.data;
                        self.inventory[pos].data =  old_gold + new_gold;
                    },
                    None => {
                        self.inventory.push(item);
                    }
                } 
            },
        }
        
        true
    }
}
