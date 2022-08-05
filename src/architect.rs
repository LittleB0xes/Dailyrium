use crate::rand::rand;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

struct Manor {
    stages_number: u32,
    stages: Stage,

}

pub enum ElementType {
    Wall,
    Floor,
}


pub struct Element {
    element_type: ElementType,
    pub glyph: u16,
    pub x: u32,
    pub y: u32,
    pub crossable: bool,
    pub fg_color: Color,
    pub bg_color: Color,
}


impl Element {
    pub fn new(x: u32, y: u32, element_type: ElementType) -> Self {
        let element: Element = match element_type {
            ElementType::Wall => {build_wall(x, y)},
            ElementType::Floor => {build_floor(x, y)},
        };
        element
    }
}

fn build_wall(x: u32, y: u32) -> Element {
    let gray_value = (50 + rand() % 50)  as f32 / 255.0;
    let bg_gray_value = (100 + rand() % 50)  as f32 / 255.0;

    Element {
        element_type: ElementType::Wall,
        glyph: '#' as u16,
        x,
        y,
        crossable: false,
        fg_color: Color::new(gray_value, gray_value, gray_value, 1.0),
        bg_color: Color::new(bg_gray_value, bg_gray_value, bg_gray_value, 1.0),

    }

}

fn build_floor(x: u32, y: u32) -> Element {
    let gray_value = (50 + rand() % 50)  as f32 / 255.0;
    let fg_color = Color::new(gray_value, gray_value, gray_value, 1.0);
    Element {
        element_type: ElementType::Floor,
        glyph: '.' as u16,
        x,
        y,
        crossable: true,
        fg_color,
        bg_color: BLACK,

    }
}
pub struct Stage {
    stage_id: u32,
    pub width: u32,
    pub height: u32,
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
            stage_map
        }


    }
}

