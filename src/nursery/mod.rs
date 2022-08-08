use macroquad::prelude::*;
use crate::rand::gen_range;

use crate::architect::Stage;

pub enum EntityType {
    Zomby,
}

pub struct LivingEntity {
    pub x: u32,
    pub y: u32,
    pub glyph: u16,
    pub fg_color: Color,
}

impl LivingEntity {
    pub fn new(x: u32, y: u32, entity_type: EntityType) -> Self {
        match entity_type {
            EntityType::Zomby => create_zomby(x, y),
        }
    }
}

fn create_zomby(x: u32, y: u32) -> LivingEntity {
    LivingEntity {
        x,
        y,
        glyph: 'Z' as u16,
        fg_color: RED,
    }
}

pub fn spawn_monsters(amount: u32, stage: &mut Stage) {
    let mut monster_index = 0;
    while monster_index < amount {
        let mut free_place = false;
        let mut x_candidate: u32 = 0;
        let mut y_candidate: u32= 0;
        while !free_place {
            x_candidate = gen_range(0, stage.width);
            y_candidate = gen_range(0, stage.height);
            if stage.stage_map[(x_candidate + y_candidate * stage.width) as usize].crossable {
                free_place = true;
            }
        }
        stage.living_entities.push(LivingEntity::new(x_candidate, y_candidate , EntityType::Zomby));

        monster_index += 1;
    }

}