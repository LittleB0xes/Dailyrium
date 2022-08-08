use crate::rand::rand;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::nursery::LivingEntity;

mod building_elements;
use building_elements::*;

struct Manor {
    stages_number: u32,
    stages: Stage,
}

pub struct Stage {
    stage_id: u32,
    pub width: u32,
    pub height: u32,
    pub stage_map: Vec<Element>,
    pub living_entities: Vec<LivingEntity>,
}

impl Stage {
    pub fn new(stage_id: u32, width: u32, height: u32) -> Self{
        rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);

        let mut stage_map: Vec<Element> = Vec::new();
        for index in 0..width*height {
            let alea: u32 = rand() % 100;
            let element_type: ElementType;
            if alea < 10 {
                element_type = ElementType::Wall;
            }
            else {
                element_type = ElementType::Floor;
            }
            let x = index % width;
            let y = index / width;
            let new_element = Element::new(x, y, element_type);
            
            stage_map.push(new_element);

        }

        Self {
            stage_id,
            width,
            height,
            stage_map,
            living_entities: Vec::new(),
        }


    }
}

