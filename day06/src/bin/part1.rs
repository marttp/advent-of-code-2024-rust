mod common;

use common::split_lines;

fn main() {
    let input = include_str!("input6.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}
const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

const OBSTACLE: char = '#';
const MARKED: char = 'X';

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

fn find_starting_points(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '^' {
                return (r, c);
            }
        }
    }
    (0, 0)
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        _ => UP,
    }
}

fn is_inbounds(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32
}

fn count_visited(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter()
        .map(|line| line.iter().filter(|c| **c == MARKED).count() as u32)
        .sum()
}
