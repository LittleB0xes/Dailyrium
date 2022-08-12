use macroquad::prelude::*;
use crate::rand::gen_range;

use crate::architect::Stage;
use crate::dailyrium::utils::inside_rect;

mod behaviour;
use behaviour::*;

mod bestiary;
use bestiary::*;




pub enum EntityType {
    ClayMold,
    Zomby,
}

pub struct LivingEntity {
    entity_type: EntityType,
    pub x: i32,
    pub y: i32,
    pub glyph: u16,
    pub fg_color: Color,
    pub behaviour: Behaviour,
    path: Vec<(i32,i32)>,
}

impl LivingEntity {
    pub fn new(x: i32, y: i32, entity_type: EntityType) -> Self {
        let mut monster_clay_mold = LivingEntity {
            entity_type: EntityType::ClayMold,
            x,
            y,
            glyph: '?' as u16,
            fg_color: WHITE,
            behaviour: Behaviour::Waiting,
            path: Vec::new(),

        };
        match entity_type {
            EntityType::Zomby => create_zomby(&mut monster_clay_mold),
            _ => {},
        }

        monster_clay_mold
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
        if inside_rect(dest_x, dest_y, 0, 0, stage.width - 1, stage.height - 1) && stage.stage_map[(dest_x + dest_y * stage.width) as usize].crossable {
            self.x = dest_x;
            self.y = dest_y;
        }
    }
}



pub fn spawn_monsters(amount: u32, stage: &Stage) -> Vec<LivingEntity> {
    let mut living_entities: Vec<LivingEntity> = Vec::new();
    let mut monster_index = 0;
    while monster_index < amount {
        let mut free_place = false;
        let mut x_candidate: i32 = 0;
        let mut y_candidate: i32= 0;
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
