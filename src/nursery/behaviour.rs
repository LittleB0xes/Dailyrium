use::macroquad::rand::rand;

/// Move behavour : Totaly random walk
pub fn drunk_walk() -> (i32, i32) {
    let alea = rand()%4;
    match alea {
        0 => (-1,0),
        1 => (1,0),
        2 => (0,1),
        3 => (0,-1),
        _ => (0,0)
    }
}
