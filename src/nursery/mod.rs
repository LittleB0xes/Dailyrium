use macroquad::prelude::*;
use crate::rand::gen_range;

use crate::architect::Stage;

mod behaviour;
use behaviour::*;

pub enum Behaviour {
    Waiting,
    Drunk,
}

pub enum EntityType {
    Zomby,
}

pub struct LivingEntity {
    pub x: u32,
    pub y: u32,
    pub glyph: u16,
    pub fg_color: Color,
    pub behaviour: Behaviour
}

impl LivingEntity {
    pub fn new(x: u32, y: u32, entity_type: EntityType) -> Self {
        match entity_type {
            EntityType::Zomby => create_zomby(x, y),
        }
    }

    pub fn update(&mut self, stage: &mut Stage) {
        match self.behaviour {
            Behaviour::Drunk =>{self.move_to(drunk_walk(), stage)},
            _ => {}
        }


    }

    fn move_to(&mut self, direction: (i32, i32), stage: &mut Stage) {
        let dest_x = self.x as i32 + direction.0;
        let dest_y = self.y as i32 + direction.1;
        if inside_rect(dest_x as u32, dest_y as u32, 0, 0, stage.width - 1, stage.height - 1) && stage.stage_map[(dest_x as u32 + dest_y as u32 * stage.width) as usize].crossable {
            self.x = dest_x as u32;
            self.y = dest_y as u32;
        }
    }

}

fn create_zomby(x: u32, y: u32) -> LivingEntity {
    LivingEntity {
        x,
        y,
        glyph: 'Z' as u16,
        fg_color: RED,
        behaviour: Behaviour::Drunk
    }
}

pub fn spawn_monsters(amount: u32, stage: &Stage) -> Vec<LivingEntity> {
    let mut living_entities: Vec<LivingEntity> = Vec::new();
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
        living_entities.push(LivingEntity::new(x_candidate, y_candidate , EntityType::Zomby));

        monster_index += 1;
    }
    living_entities
}

pub fn inside_rect(x: u32, y: u32, box_x: u32, box_y: u32, box_w: u32, box_h: u32) -> bool {
    x >= box_x
    && x <= box_x + box_w
    && y >=box_y
    && y <= box_y + box_h
}