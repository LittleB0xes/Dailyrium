use rand::prelude::*;
use crate::dailyrium::{Action, Propertie};
use crate::elements::Element;
use crate::living_entity::LivingEntity;


pub fn give_id() -> u32 {
    static mut ID: u32 = 0;
    unsafe {
        ID += 1;
        ID
    }
}

fn is_in_map(x: i32, y: i32, w: i32, h: i32) -> bool {
    if x < 0 || y < 0 || x >= w || y >= h {
        false
    }
    else {
        true
    }
}

pub fn action_manager(ent: &mut LivingEntity, level: &mut Vec<Element>, width: i32, height: i32) -> bool{
    match ent.action {
        Action::Move(dx, dy) => {
            let new_x = ent.x + dx;
            let new_y = ent.y + dy;


            
            if is_in_map(new_x, new_y, width, height) && level[((ent.x + dx) + (ent.y + dy) * width) as usize].properties.iter().any(|&c| c == Propertie::Crossable) {
                ent.move_entity(dx, dy);
            }
            
            ent.action = Action::Waiting;
            true
        }
        _ => {false}
    }
}

pub fn brain(ent: &mut LivingEntity) {
    let mut rng = rand::thread_rng();
    let dx = rng.gen_range(-1..2);
    let dy = rng.gen_range(-1..2);
    ent.action = Action::Move(dx, dy);
}
