
use macroquad::prelude::*;
use crate::rand::rand;

//union EntityData {
    
//}

pub enum ElementType {
    Wall,
    Floor,
    Upstairs,
    Dwonstairs
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
    pub alois_factor: f32,
}

impl Element {
    pub fn new(x: i32, y: i32, element_type: ElementType) -> Self {
        let mut draft_element =  Element {
            element_type: ElementType::Floor,
            glyph: '?' as u16,
            x,
            y,
            crossable: true,
            fg_color: WHITE,
            bg_color: BLACK,
            seen: false,
            visited: false,
            see_through: true,
            alois_factor: 1.0,
        };


        match element_type {
            ElementType::Wall => {build_wall_block(&mut draft_element)},
            ElementType::Floor => {build_floor(&mut draft_element)},
            //ElementType::Upstairs => {},
            //ElementTyp::Downstairs => {}
            _ => {build_floor(&mut draft_element)}
        };
        draft_element
    }
    pub fn alzheimerize(&mut self) {
        self.alois_factor -= 0.05;
        if self.alois_factor <= 0.0 {
            self.alois_factor = 1.0;
            self.visited = false;
        }
    }
}
fn build_wall_block(element: &mut Element){
    let gray_value = (50 + rand() % 50)  as f32 / 255.0;
    let bg_gray_value = (100 + rand() % 50)  as f32 / 255.0;

        element.element_type = ElementType::Wall;
        element.glyph = '#' as u16;
        element.crossable = false;
        element.fg_color = Color::new(gray_value, gray_value, gray_value, 1.0);
        element.bg_color = Color::new(bg_gray_value, bg_gray_value, bg_gray_value, 1.0);
}

fn build_floor(element: &mut Element) {
    let gray_value = (50 + rand() % 50)  as f32 / 255.0;
    let fg_color = Color::new(gray_value, gray_value, gray_value, 1.0);
    let dark_gray_value =  (rand() % 25)  as f32 / 255.0;
    let bg_color = Color::new(dark_gray_value, dark_gray_value, dark_gray_value, 1.0);
    element.element_type = ElementType::Floor;
    element.glyph = '.' as u16;
    element.fg_color = fg_color;
    element.bg_color = bg_color;
}

fn build_downstairs(element: &mut Element) {
   
}