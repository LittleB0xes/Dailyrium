use rand::prelude::*;
use crate::dailyrium::{Action, Property};
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


            
            if is_in_map(new_x, new_y, width, height) && level[((ent.x + dx) + (ent.y + dy) * width) as usize].have_property(Property::Crossable) {
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

// Simple raycasting fov with range view
//pub fn fov(
//    x_entity: i32,
//    y_entity: i32,
//    range: i32,
//    level_map: &mut Vec<Tile>,
//    w: i32,
//    h: i32,
//) -> Vec<[i32; 2]> {
//    // Create vec of tiles in fov
//    let mut in_fov_tile: Vec<[i32; 2]> = Vec::new();
//
//    // Player's tile allways visible
//    in_fov_tile.push([x_entity, y_entity]);
//
//    // Initialize all tile to unsee
//    for i in 0..w {
//        for j in 0..h {
//            level_map[(i + j * w) as usize].visible = false;
//        }
//    }
//
//    for a in 0..360 {
//        // Set normalize direction vector
//        let x = ((a as f32) * PI / 180.0).cos();
//        let y = ((a as f32) * PI / 180.0).sin();
//
//        // Player position (center)
//        let mut dx = (x_entity as f32) + 0.5;
//        let mut dy = (y_entity as f32) + 0.5;
//
//        for _i in 0..range {
//            // break if out of map
//            if dx >= w as f32 || dx < 0.0 || dy < 0.0 || dy >= h as f32 {
//                break;
//            }
//
//            // index af tile
//            let z = (dx.trunc() as i32) + (dy.trunc() as i32) * w;
//
//            level_map[z as usize].visible = true;
//            level_map[z as usize].visited = true;
//
//            // Add tile in fov
//            in_fov_tile.push([z % w, z / w]);
//
//            if level_map[z as usize].see_through == false && z != x_entity + y_entity * w
//            // For the Door visual effect when Player is on a Door
//            {
//                break;
//            }
//            dx += x;
//            dy += y;
//        }
//    }
//
//    in_fov_tile
//}
//