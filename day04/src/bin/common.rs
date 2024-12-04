pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub const DIAG_DIRECTIONS: [(i32, i32); 8] = [
    (-1,-1),(-1,0),(-1,1),
    (0,-1),(0,1),
    (1,-1),(1,0),(1,1)
];