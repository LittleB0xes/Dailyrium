use crate::rand::rand;
use macroquad::prelude::*;

use crate::nursery::LivingEntity;

pub mod building_elements;
use building_elements::*;

pub enum GenerationType {
    Random,
    Room,
    Cave,
    Test,
}


pub struct Manor {
    pub stages_number: u32,
    pub stages: Vec<Stage>,
}

impl Manor {
    pub fn new(width: i32, height: i32, amount: u32) -> Self {
        let mut stages: Vec<Stage> = Vec::new();
        for i in 0..amount {
            stages.push(Stage::new(i, width, height, GenerationType::Cave));
        }

        Self { stages_number: amount, stages }

    }
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

        let stage_map = match generation_type {
            GenerationType::Random => random_generation(width, height),
            GenerationType::Cave => cave_stage_generation(width, height),
            GenerationType::Test => test_room_generation(width, height),
            _ => random_generation(width, height),
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

/// Basic room for testing purpose
fn test_room_generation(width: i32, height: i32) -> Vec<Element> {
    let mut stage_map: Vec<Element> = Vec::new();
    for index in 0..width * height {
        let x: i32 = index % width;
        let y: i32 = index / width;
        let new_element: Element = Element::new(x, y, ElementType::Floor);

        stage_map.push(new_element);
    }

    stage_map
}

/// Random stage generation. Unuseful. Just for testing
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


/// Cave generation
fn cave_stage_generation(width: i32, height: i32) -> Vec<Element> {
    let mut stage_map = Vec::new();
    
    // First, fill all the map with wall
    for i in 0..(width * height) as usize {
        stage_map.push(Element::new(i as i32 % width, i as i32 / width, ElementType::Wall));
    }

    // Cave
    //let h_zone = 20;
    //let v_zone = 14;
    //let room_width = || {2 + rand() % 5};
    //let room_height = || {2 + rand() % 5};
    
    // Large room space
    let h_zone = 12;
    let v_zone = 8;
    let room_width = || {5 + rand() % 5};
    let room_height = || {5 + rand() % 5};

    // looks more like cave
    //let h_zone = 15;
    //let v_zone = 10;
    //let room_width = || {3 + rand() % 5};
    //let room_height = || {3 + rand() % 5};
    // create an empty place for each cell
    for i in 0..h_zone {
        for j in 0..v_zone {
            let r_width = room_width();
            let r_height = room_height();
            let xo = i * width / h_zone + (rand() % 5) as i32;
            let yo = j * height / v_zone + (rand() % 5) as i32;
            for x in xo..(xo + r_width as i32) {
                for y in yo..(yo + r_height as i32) {
                    if x < width && y < height {
                        stage_map[(x + y * width) as usize] = Element::new(x as i32, y as i32, ElementType::Floor);
                    }
                }
            }
        }
    }

    stage_map
}
