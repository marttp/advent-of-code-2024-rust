mod common;

use crate::common::{find_start_points, is_valid, split_lines, DIRECTIONS};

fn main() {
    let input = include_str!("input10.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let mut grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let trail_start_points: Vec<(i64, i64)> = find_start_points(&grid);
    let mut amount = 0;
    for points in trail_start_points {
        let value = count_reach_end(&mut grid, points, 0);
        amount += value;
    }
    amount
}

fn count_reach_end(
    grid: &mut Vec<Vec<i64>>,
    current_position: (i64, i64),
    expected: i64
) -> u64 {
    if !is_valid(grid, current_position.0, current_position.1) {
        return 0;
    }
    let current = grid[current_position.0 as usize][current_position.1 as usize];
    if current != expected {
        return 0;
    }
    if current == 9 && expected == 9 {
        return 1;
    }
    let mut count = 0;
    let tmp = current;
    grid[current_position.0 as usize][current_position.1 as usize] = 100;
    for dir in DIRECTIONS {
        let next = (current_position.0 + dir.0, current_position.1 + dir.1);
        count += count_reach_end(grid, next, expected + 1);
    }
    grid[current_position.0 as usize][current_position.1 as usize] = tmp;
    count
}
