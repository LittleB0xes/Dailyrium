use crate::rand::rand;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

struct Manor {
    stages_number: u32,
    stages: Stage,

}

enum ElementType {
    Wall,
    Floor,
}


pub struct Element {
    element_type: ElementType,
    pub glyph: u16,
    pub x: u32,
    pub y: u32,
}

//impl Element {
//    pub fn new(x: u32, y: u32, element_type: ElementType) -> Self {
//        Self {
//            element_type,
//            glyph,
//            x,
//            y,
//        }
//    }
//}

pub struct Stage {
    stage_id: u32,
    width: u32,
    height: u32,
    pub stage_map: Vec<Element>,
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
            let glyph: u16;
            if alea < 10 {
                glyph = '#' as u16;
            }
            else {
                glyph = '.' as u16;
            }
            let new_element = Element {
                x: index % width,
                y: index / width,
                element_type: ElementType::Floor,
                glyph
            };
            stage_map.push(new_element);

        }



        Self {
            stage_id,
            width,
            height,
            stage_map
        }


    }
}

