use crate::architect::building_elements::Element;
use crate::dailyrium::utils::*;


/// Pathfinding with A* algorithm
pub fn path_finder(
    x_entity: i32,
    y_entity: i32,
    x_mouse: i32,
    y_mouse: i32,
    level_map: &Vec<Element>,
    w: i32,
    h: i32,
) -> Option<Vec<(i32,i32)>> {

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Node {
        id: i32,
        x: i32,
        y: i32,
        f: i32,
        g: i32,
        h: i32,
        parent: i32,
    }

    const MAX_CYCLE: i32 = 4000;
    //let mut path = Vec::new();

    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    // Add the starting point to the open list
    open_list.push(Node {
        x: x_entity,
        y: y_entity,
        f: 0,
        g: 0,
        h: distance(x_entity, y_entity, x_mouse, y_mouse),
        id: 0,
        parent: -1,
    });

    let mut id: i32 = 0;

    // Check mouse position
    if inside_rect(x_mouse, y_mouse, 0, 0, w-1, h-1) {
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
                let g = current_node.g + 1;
                let h = distance(c.0, c.1, x_mouse, y_mouse);
                let f = g + h;
                // Zap node who are already in closed list
                if closed_list.iter().any(|&n| n.x == c.0 && n.y == c.1 && n.g < g) {
                    continue;
                }

                if open_list.iter().any(|&n| n.x == c.0 && n.y == c.1 && n.g < g ) {
                    continue;
                };
                id += 1;

                open_list.push(Node {
                    x: c.0,
                    y: c.1,
                    f,
                    g,
                    h,
                    id,//: -1,
                    parent: current_node.id,
                });
            }
        }
    }

    let mut path: Vec<(i32, i32)> = Vec::new();
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
                        h: o.h,
                        parent: o.parent,
                    };
                }
            }
        }

        path.reverse();
        Some(path)

    }
    else {
        None
    }
    //if closed_list.len() != 0 {
    //    for e in closed_list.iter() {
    //        path.push((e.x, e.y));
    //    }
    //}

}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
    //(x1 - x2).abs() + (y1 - y2).abs()
}