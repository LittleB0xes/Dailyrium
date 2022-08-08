
use macroquad::prelude::*;
use crate::rand::rand;

pub enum ElementType {
    Wall,
    Floor,
}
pub struct Element {
    element_type: ElementType,
    pub glyph: u16,
    pub x: i32,
    pub y: i32,
    pub crossable: bool,
    pub seen: bool,
    pub visited: bool,
    pub see_through: bool,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl Element {
    pub fn new(x: i32, y: i32, element_type: ElementType) -> Self {
        let element: Element = match element_type {
            ElementType::Wall => {build_wall_block(x, y)},
            ElementType::Floor => {build_floor(x, y)},
        };
        element
    }
}
fn build_wall_block(x: i32, y: i32) -> Element {
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
        seen: false,
        visited: false,
        see_through: false,
    }
}

fn build_floor(x: i32, y: i32) -> Element {
    let gray_value = (50 + rand() % 50)  as f32 / 255.0;
    let fg_color = Color::new(gray_value, gray_value, gray_value, 1.0);
    let dark_gray_value =  (rand() % 25)  as f32 / 255.0;
    let bg_color = Color::new(dark_gray_value, dark_gray_value, dark_gray_value, 1.0);
    Element {
        element_type: ElementType::Floor,
        glyph: '.' as u16,
        x,
        y,
        crossable: true,
        fg_color,
        bg_color,
        seen: false,
        visited: false,
        see_through: true,
    }
}