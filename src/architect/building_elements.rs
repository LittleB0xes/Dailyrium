
use macroquad::prelude::*;
use crate::rand::rand;

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
            ElementType::Wall => {build_wall_block(x, y)},
            ElementType::Floor => {build_floor(x, y)},
        };
        element
    }
}
fn build_wall_block(x: u32, y: u32) -> Element {
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