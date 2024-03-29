use crate::architect::building_elements::Element;
use macroquad::color::Color;

use std::f32::consts::PI;

/// Simple raycasting fov with range view
pub fn fov_raycast(
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

/// Check if a point is inside a rectangle
pub fn inside_rect(x: i32, y: i32, box_x: i32, box_y: i32, box_w: i32, box_h: i32) -> bool {
    x >= box_x
    && x <= box_x + box_w
    && y >=box_y
    && y <= box_y + box_h
}


/// Macroquad Color multiplication
pub fn mul_color(color: Color, factor: f32) -> Color {
    Color::new(factor * color.r, factor * color.g, factor * color.b, color.a)
}
