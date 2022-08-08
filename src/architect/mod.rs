use crate::rand::rand;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::nursery::LivingEntity;

pub mod building_elements;
use building_elements::*;

pub enum GenerationType {
    Random,
    Room
}


struct Manor {
    stages_number: u32,
    stages: Stage,
}

pub struct Stage {
    stage_id: u32,
    pub width: i32,
    pub height: i32,
    pub stage_map: Vec<Element>,
    pub living_entities: Vec<LivingEntity>,
}

impl Stage {
    pub fn new(stage_id: u32, width: i32, height: i32, generation_type: GenerationType) -> Self{
        rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);

        let stage_map = match generation_type {
            GenerationType::Random => random_generation(width, height),
            GenerationType::Room => room_stage_generation(width, height),
        };        

        Self {
            stage_id,
            width,
            height,
            stage_map,
            living_entities: Vec::new(),
        }
    }
}

fn random_generation(width: i32, height: i32) -> Vec<Element> {
    let mut stage_map = Vec::new();
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

    stage_map
}

fn room_stage_generation(width: i32, height: i32) -> Vec<Element> {

    let h_zone = 6;
    let v_zone = 4;


    let mut stage_map = Vec::new();
    
    // First, fill all the map with wall
    for i in 0..(width * height) as usize {
        stage_map.push(Element::new(i as i32 % width, i as i32 / width, ElementType::Wall));
    }

    // create an empty place for each cell
    for i in 0..h_zone {
        for j in 0..v_zone {
            let room_width = 10 + rand() % 5;
            let room_height = 10 + rand() % 5;
            let xo = i * width / h_zone + (rand() % 5) as i32;
            let yo = j * height / v_zone + (rand() % 5) as i32;
            for x in xo..(xo + room_width as i32) {
                for y in yo..(yo + room_height as i32) {
                    if x < width && y < height {

                        stage_map[(x + y * width) as usize] = Element::new(x as i32, y as i32, ElementType::Floor);
                    }
                }
            }
        }

    }


    // Room constructioni here...

    stage_map

}
