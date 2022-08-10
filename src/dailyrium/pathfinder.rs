use macroquad::math::Vec2;
use crate::architect::building_elements::Element;
use crate::dailyrium::utils::*;

//#[derive(Copy, Clone)]
//struct Node {
//    x: i32,
//    y: i32,
//    f: i32,
//    g: i32,
//    id: i32,
//    parent: i32,
//
//}
//
///// Pathfinding with A* algorithm
//pub fn path_finder(
//    x_entity: i32,
//    y_entity: i32,
//    x_mouse: i32,
//    y_mouse: i32,
//    level_map: &Vec<Element>,
//    width: i32,
//    height: i32,
//) {
//
//    let mut id: i32 = 0;
//
//    let mut open_list: Vec<Node> = Vec::new();
//    let mut closed_list: Vec<Node> = Vec::new();
//
//    open_list.push(Node{x: x_entity, y: y_entity, f: 0, g: 0, id, parent: -1});
//
//    while open_list.len() != 0 {
//        let mut current_node: Node;
//         // Choose the node with the best score
//         let mut best_index: usize = 0;
//         current_node = open_list[best_index];
//         for i in 0..open_list.len() {
//            if open_list[i].f < current_node.f {
//                current_node = open_list[i];
//                best_index = i;
//            }
//        }
//        current_node = open_list[best_index];
//        closed_list.push(current_node);
//        open_list.remove(best_index);
//
//        if current_node.x == x_mouse && current_node.y == y_mouse {
//            // Yes, you reach the goal ! Backtrack the closed_list
//        }
//
//        // create children list of current node
//        // ad take only usable tile (crossable, visited...)
//        let mut children: Vec<(i32, i32)> = Vec::new();
//        for c in [
//            (1, 0),
//            (-1, 0),
//            (0, 1),
//            (0, -1),
//            (1, 1),
//            (1, -1),
//            (-1, 1),
//            (-1, -1),
//        ].iter() {
//            let children_x = current_node.x + c.0;
//            let children_y = current_node.y + c.1;
//            let good_place: bool = inside_rect(children_x, children_y, 0, 0, width - 1, height -1) && level_map[(children_x + children_y * width) as usize].crossable && level_map[(children_x + children_y * width) as usize].visited;
//            if good_place {
//                children.push((children_x, children_y));
//            }
//        }
//
//        for child in children {
//            if closed_list.iter().any(|&n| n.x == child.0 && n.y == child.1) {
//                continue;
//            }
//            let g = current_node.g + 1;
//            let h = distance(child.0, child.1, x_mouse, y_mouse);
//            let f = g + h;
//
//            // check if in open list and ignore if child's g greater
//            let mut skip_this_child = false;
//            for op in open_list.iter() {
//                if op.x == child.0 && op.y == child.1 && op.g < g {
//                    skip_this_child = true;
//                }
//            }
//            if skip_this_child {
//                continue;
//            }
//            id += 1;
//            open_list.push(Node{
//                x: child.0,
//                y: child.1,
//                f,
//                g,
//                id,
//                parent: current_node.id,
//            });
//        }
//    }
//    // Now, take the best way
//    // Go from child to parent until reach the starting point
//    let mut path: Vec<(i32, i32)> = Vec::new();
//    if closed_list.len() != 0 {
//        let mut node = closed_list[closed_list.len() - 1];
//        path.push((node.x, node.y));
//        while node.parent != -1 {
//            for o in closed_list.iter() {
//                if node.parent == o.id {
//                    path.push((o.x, o.y));
//                    node = Node {
//                        id: o.id,
//                        x: o.x,
//                        y: o.y,
//                        f: o.f,
//                        g: o.g,
//                        parent: o.parent,
//                    };
//                }
//            }
//        }
//    }
//    for e in path {
//        print!("{}, {} -- ", e.0, e.1);
//
//    }
//    println!();
//
//}

/// Pathfinding with A* algorithm
pub fn path_finder(
    x_entity: i32,
    y_entity: i32,
    x_mouse: i32,
    y_mouse: i32,
    level_map: &Vec<Element>,
    w: i32,
    h: i32,
) -> Vec<(i32,i32)> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Node {
        id: i32,
        x: i32,
        y: i32,
        f: i32,
        g: i32,
        parent: i32,
    };

    const MAX_CYCLE: i32 = 4000;
    let mut path = Vec::new();

    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    // Add the starting point to the open list
    open_list.push(Node {
        x: x_entity,
        y: y_entity,
        f: 0,
        g: 0,
        id: 0,
        parent: -1,
    });

    let mut id: i32 = 0;

    // Check mouse position
    if inside_rect(x_mouse, y_mouse, 0, 0, w, h)
        && level_map[(x_mouse + y_mouse * w) as usize].crossable
        && level_map[(x_mouse + y_mouse * w) as usize].visited
    {
        let mut cycle = 0;
        while !open_list.is_empty() {
            // Add a depth limit for path finding (to avoid infinite loop)
            //if open_list.len() > (w * h) as usize {
            cycle += 1;
            if cycle > MAX_CYCLE {
                closed_list.clear();
                break;
            }
            let mut current_node = open_list[0];
            let mut best_index: usize = 0;

            // Choose the node with the best score
            for i in 0..open_list.len() {
                if open_list[i].f < current_node.f {
                    current_node = open_list[i];
                    best_index = i;
                }
            }
            current_node.id = id;
            closed_list.push(current_node);

            // Update the node id value
            id += 1;

            // Remove the selected node from the open list
            open_list.remove(best_index);

            // If current is the goal, the break
            if current_node.x == x_mouse && current_node.y == y_mouse {
                break;
            }

            // create children list of cuurent node
            let mut children: Vec<(i32,i32)> = Vec::new();
            for c in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
            .iter()
            {
                // keep only node in the map and crossable
                if c.0 + current_node.x >= 0
                    && c.0 + current_node.x < w
                    && c.1 + current_node.y >= 0
                    && c.1 + current_node.y < h
                    && level_map[(c.0 + current_node.x + (c.1 + current_node.y) * w) as usize]
                        .crossable
                    && level_map[(c.0 + current_node.x + (c.1 + current_node.y) * w) as usize]
                        .visited
                {
                    children.push((c.0 + current_node.x, c.1 + current_node.y));
                }
            }

            for c in children.iter() {
                // Zap node who are already in closed list
                if closed_list.iter().any(|&n| n.x == c.0 && n.y == c.1) {
                    continue;
                }
                let g = current_node.g + 1;
                let h = distance(c.0, c.1, x_mouse, y_mouse);
                let f = g + h;

                let mut out = false;

                // Just keep the best node if it's already in open list
                for op in open_list.iter() {
                    if op.x == c.0 && op.y == c.1 && op.g < g {
                        out = true;
                    }
                }
                if out {
                    continue;
                }

                open_list.push(Node {
                    x: c.0,
                    y: c.1,
                    f: f,
                    g: g,
                    id: -1,
                    parent: current_node.id,
                });
            }
        }
    }

    // Now, take the best way
    // Go from child to parent until reach the starting point
    if closed_list.len() != 0 {
        let mut node = closed_list[closed_list.len() - 1];
        path.push((node.x, node.y));
        while node.parent != -1 {
            for o in closed_list.iter() {
                if node.parent == o.id {
                    path.push((o.x, o.y));
                    node = Node {
                        id: o.id,
                        x: o.x,
                        y: o.y,
                        f: o.f,
                        g: o.g,
                        parent: o.parent,
                    };
                }
            }
        }
    }

    path
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}