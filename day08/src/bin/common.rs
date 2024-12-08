pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn is_inbounds(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32
}
