mod common;

use crate::common::{count_visited, find_starting_points, is_inbounds, split_lines, turn_right, MARKED, OBSTACLE, UP};

fn main() {
    let input = include_str!("input6.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let mut grid: Vec<Vec<char>> = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let starting_point = find_starting_points(&grid);

    let mut current_point = starting_point;
    let mut direction = UP;
    grid[current_point.0][current_point.1] = MARKED;

    while is_inbounds(
        &grid,
        current_point.0 as i32 + direction.0,
        current_point.1 as i32 + direction.1,
    ) {
        let nr = current_point.0 as i32 + direction.0;
        let nc = current_point.1 as i32 + direction.1;
        if grid[nr as usize][nc as usize] == OBSTACLE {
            direction = turn_right(direction);
        } else {
            grid[nr as usize][nc as usize] = MARKED;
            current_point.0 = (current_point.0 as i32 + direction.0) as usize;
            current_point.1 = (current_point.1 as i32 + direction.1) as usize;
        }
    }

    count_visited(&grid)
}