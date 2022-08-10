use macroquad::math::Vec2;
use crate::architect::building_elements::Element;
use crate::dailyrium::utils::*;

#[derive(Copy, Clone)]
struct Node {
    x: i32,
    y: i32,
    f: i32,
    g: i32,

}

/// Pathfinding with A* algorithm
pub fn path_finder(
    x_entity: i32,
    y_entity: i32,
    x_mouse: i32,
    y_mouse: i32,
    level_map: &Vec<Element>,
    width: i32,
    height: i32,
) {
    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    open_list.push(Node{x: x_entity, y: y_entity, f: 0, g: 0});

    while open_list.len() != 0 {
        let mut current_node: Node;
         // Choose the node with the best score
         let mut best_index: usize = 0;
         current_node = open_list[best_index];
         for i in 0..open_list.len() {
            if open_list[i].f < current_node.f {
                current_node = open_list[i];
                best_index = i;
            }
        }
        current_node = open_list[best_index];
        closed_list.push(current_node);
        open_list.remove(best_index);

        if current_node.x == x_mouse && current_node.y == y_mouse {
            // Yes, you reach the goal ! Backtrack the closed_list
        }

        // create children list of current node
        // ad take only usable tile (crossable, visited...)
        let mut children: Vec<(i32, i32)> = Vec::new();
        for c in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ].iter() {
            let children_x = current_node.x + c.0;
            let children_y = current_node.y + c.1;
            let good_place: bool = inside_rect(children_x, children_y, 0, 0, width - 1, height -1) && level_map[(children_x + children_y * width) as usize].crossable && level_map[(children_x + children_y * width) as usize].visited;
            if good_place {
                children.push((children_x, children_y));
            }
        }

        for child in children {
            if closed_list.iter().any(|&n| n.x == child.0 && n.y == child.1) {
                continue;
            }
            let g = current_node.g + 1;
            let h = distance(child.0, child.1, x_mouse, y_mouse);
            let f = g + h;

            // check if in open list and ignore if child's g greater
            let mut out = false;
            for op in open_list.iter() {
                if op.x == child.0 && op.y == child.1 && op.g < g {
                    out = true;
                }
            }
            if out {
                continue;
            }

        }
    }

}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}