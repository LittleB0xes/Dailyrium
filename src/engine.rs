use std::collections::HashMap;
use std::f32::consts::PI;

use rand::prelude::*;
use crate::dailyrium::Action;
use crate::living_entities::{Behavior, EntityType};
use crate::world_factory::Level;
use crate::elements::Element;


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

fn squared_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

pub fn puppet_master(level: &mut Level, play_log: &mut Vec<String>) {

    let mut occupied_cell: HashMap<u32, (i32, i32)> = HashMap::new();
    // Clone is an expensive method... For the moment, it's not useful...
    //let player = level.entities[0].clone();        

    //... So just take the useful values
    let player_x = level.entities[0].x;
    let player_y = level.entities[0].y;


    // Behavior resolution
    for entity in level.entities.iter_mut() {
        match entity.behavior {
            Behavior::Drunk => {
                entity.action = drunk_walking(); 
            },
            Behavior::Sleep => {

                // we need to deference player data before use
                if squared_distance(entity.x, entity.y, player_x, player_y) < 9 {
                    entity.behavior = Behavior::Drunk
                }
            },
            _ =>{},

        }

        occupied_cell.insert(entity.id, (entity.x, entity.y));
    }

    // Action resolution
    for entity in level.entities.iter_mut() {
        match entity.action {
            Action::Move(dx, dy) => {
                let new_x = entity.x + dx;
                let new_y = entity.y + dy;

                if is_in_map(new_x, new_y, level.width, level.height) && level.level_map[((entity.x + dx) + (entity.y + dy) * level.width) as usize].crossable {
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
                        // Message in log
                        play_log[0] = level.items[pos].description.clone(); 
                        let pick_ok = entity.add_to_inventory(level.items[pos].clone());
                        if pick_ok {
                            level.items.remove(pos);
                        }
                    },
                    None => {
                        play_log[0] = "Nothing to pick up !".to_string(); 
                    }
                }
            }
            _ => {}
        }
        // Fov calculation only for the Hero
        if entity.nature == EntityType::Hero {
            level.in_fov = fov_raycast(entity.x, entity.y, entity.view_range as i32, &mut level.level_map, level.width, level.height);
        }

        // Check if the entity is in player (id entities[0]) FOV
        if level.in_fov.iter().any(|cell| cell.0 == entity.x && cell.1 == entity.y) {
            entity.seen = true;
        } else {
            entity.seen = false;
        }
    }

    // Check if items are in player's fov
    for item in level.items.iter_mut() {
        if level.in_fov.iter().any(|cell| cell.0 == item.x && cell.1 == item.y) {
            item.seen = true;
        } else {
            item.seen = false;
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
fn fov_raycast(
    x_entity: i32,
    y_entity: i32,
    range: i32,
    level_map: &mut Vec<Element>,
    w: i32,
    h: i32,
) -> Vec<(i32, i32)> {
    // Create vec of tiles in fov
    let mut in_fov_tile: Vec<(i32, i32)> = Vec::new();

    // Player's tile allways visible
    in_fov_tile.push((x_entity, y_entity));

    // Initialize all tile to unsee
    for i in 0..w {
        for j in 0..h {
            level_map[(i + j * w) as usize].seen = false;
        }
    }

    for a in 0..360 {
        // Set normalize direction vector
        let x = ((a as f32) * PI / 180.0).cos();
        let y = ((a as f32) * PI / 180.0).sin();

        // Player position (center)
        let mut dx = (x_entity as f32) + 0.5;
        let mut dy = (y_entity as f32) + 0.5;

        for _i in 0..range {
            // break if out of map
            if dx >= w as f32 || dx < 0.0 || dy < 0.0 || dy >= h as f32 {
                break;
            }

            // index af tile
            let z = (dx.trunc() as i32) + (dy.trunc() as i32) * w;

            level_map[z as usize].seen = true;
            level_map[z as usize].visited = true;

            // Add tile in fov
            in_fov_tile.push((z % w, z / w));

            if level_map[z as usize].see_through == false && z != x_entity + y_entity * w
            // For the Door visual effect when Player is on a Door
            {
                break;
            }
            dx += x;
            dy += y;
        }
    }

    in_fov_tile
}