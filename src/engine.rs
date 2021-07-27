use std::collections::HashMap;

use rand::prelude::*;
use crate::dailyrium::{Action, Property, PropertyValue};
use crate::living_entities::Behavior;
use crate::world_factory::Level;


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

pub fn puppet_master(level: &mut Level, play_log: &mut Vec<String>) {

    let mut occupied_cell: HashMap<u32, (i32, i32)> = HashMap::new();

    // Behavior resolution
    for entity in level.entities.iter_mut() {
        match entity.behavior {
            Behavior::Drunk => {
                entity.action = drunk_walking(); 
            },
            Behavior::Sleep => {},

        }

        occupied_cell.insert(entity.id, (entity.x, entity.y));
    }

    // Action resolution
    for entity in level.entities.iter_mut() {
        match entity.action {
            Action::Move(dx, dy) => {
                let new_x = entity.x + dx;
                let new_y = entity.y + dy;

                if is_in_map(new_x, new_y, level.width, level.height) && level.level_map[((entity.x + dx) + (entity.y + dy) * level.width) as usize].have_property(Property::Crossable) == PropertyValue::Bool(true) {
                    // if crossable, we need to check if cell is occupied by another living entity
                    let not_allowed = occupied_cell.iter().any(|cell|  cell.1.0 == new_x && cell.1.1 == new_y);  // Erk !! It's ugly
                    if !not_allowed {
                        entity.move_entity(dx, dy);
                        
                        // Occupied cell hashmap updating with the new position
                        occupied_cell.insert(entity.id, (new_x, new_y));
                    }
                }
                
                entity.action = Action::Waiting;
            },
            Action::Pick => {
                let item_position = level.items.iter().position(|item| item.x == entity.x && item.y == entity.y);
                match item_position {
                    Some(pos) => {
                        play_log[0] = level.items[pos].description.clone(); 
                        level.items.remove(pos);
                    },
                    None => {
                        play_log[0] = "Nothing to pick up !".to_string(); 
                    }
                }
            }
            _ => {}
        }
    }
}

fn drunk_walking() -> Action {
    let mut rng = rand::thread_rng();
    let dx = rng.gen_range(-1..2);
    let dy = rng.gen_range(-1..2);
    Action::Move(dx, dy)
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