pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub const UP: (i32, i32) = (-1, 0);
pub const DOWN: (i32, i32) = (1, 0);
pub const LEFT: (i32, i32) = (0, -1);
pub const RIGHT: (i32, i32) = (0, 1);

pub fn rotate_clockwise(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        UP => RIGHT,
        DOWN => LEFT,
        LEFT => DOWN,
        RIGHT => UP,
        _ => panic!("Invalid direction"),
    }
}

pub fn rotate_counterclockwise(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        UP => LEFT,
        DOWN => RIGHT,
        LEFT => UP,
        RIGHT => DOWN,
        _ => panic!("Invalid direction"),
    }
}