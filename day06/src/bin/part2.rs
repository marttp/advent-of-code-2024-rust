mod common;

use std::collections::HashSet;
use crate::common::{find_starting_points, is_inbounds, split_lines, turn_right, EMPTY, OBSTACLE, UP};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    position: (usize, usize),
    direction: (i32, i32),
}

fn main() {
    let input = include_str!("input6.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let start = find_starting_points(&grid);
    let mut valid_positions = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == EMPTY {
                if try_path_with_obstacle(&mut grid, start, (i, j)) {
                    valid_positions += 1;
                }
            }
        }
    }

    valid_positions
}

fn try_path_with_obstacle(
    grid: &mut Vec<Vec<char>>,
    start: (usize, usize),
    obstacle_pos: (usize, usize)
) -> bool {
    if obstacle_pos == start {
        return false;
    }

    grid[obstacle_pos.0][obstacle_pos.1] = OBSTACLE;

    let mut current = State {
        position: start,
        direction: UP,
    };

    let mut visited = HashSet::new();
    visited.insert(current);

    loop {
        let next_pos = (
            (current.position.0 as i32 + current.direction.0) as usize,
            (current.position.1 as i32 + current.direction.1) as usize,
        );

        // Check if next position is out of bounds
        if !is_inbounds(grid, next_pos.0 as i32, next_pos.1 as i32) {
            grid[obstacle_pos.0][obstacle_pos.1] = EMPTY;
            return false;
        }

        // If there's an obstacle, turn right
        if grid[next_pos.0][next_pos.1] == OBSTACLE {
            current.direction = turn_right(current.direction);
        } else {
            current.position = next_pos;
        }

        if !visited.insert(current) {
            grid[obstacle_pos.0][obstacle_pos.1] = EMPTY;
            return true;
        }
    }
}
