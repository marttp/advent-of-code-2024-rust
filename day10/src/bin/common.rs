pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub const START_POSITION: usize = 0;
pub const END_POSITION: usize = 9;
pub const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn is_valid(grid: &Vec<Vec<i64>>, r: i64, c: i64) -> bool {
    r >= 0
        && r < grid.len() as i64
        && c >= 0
        && c < grid[0].len() as i64
        && grid[r as usize][c as usize] != 100
}

pub fn find_start_points(grid: &Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    let mut points: Vec<(i64, i64)> = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                points.push((r as i64, c as i64));
            }
        }
    }
    points
}
