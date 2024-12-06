pub const UP: (i32, i32) = (-1, 0);
pub const DOWN: (i32, i32) = (1, 0);
pub const LEFT: (i32, i32) = (0, -1);
pub const RIGHT: (i32, i32) = (0, 1);

pub const OBSTACLE: char = '#';
pub const MARKED: char = 'X';
pub const EMPTY: char = '.';

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn find_starting_points(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '^' {
                return (r, c);
            }
        }
    }
    (0, 0)
}

pub fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        _ => UP,
    }
}

pub fn is_inbounds(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32
}

pub fn count_visited(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter()
        .map(|line| line.iter().filter(|c| **c == MARKED).count() as u32)
        .sum()
}
