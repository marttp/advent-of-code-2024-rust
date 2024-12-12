mod common;

use crate::common::split_lines;
use std::collections::HashSet;

const MARKED: char = '#';
const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let input = include_str!("input12.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i64 {
    let mut garden = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total_price = 0;
    for r in 0..garden.len() {
        for c in 0..garden[r].len() {
            if garden[r][c] != MARKED {
                let label = garden[r][c];
                let (area, perimeter) = explore_region(&mut garden, (r, c), label);
                total_price += area * perimeter;
            }
        }
    }
    total_price
}

fn explore_region(garden: &mut Vec<Vec<char>>, pos: (usize, usize), label: char) -> (i64, i64) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut area = 0;
    let mut perimeter = 0;
    let mut stack: Vec<(usize, usize)> = vec![pos];

    while let Some(next_pos) = stack.pop() {
        let (r, c) = next_pos;
        if visited.insert(next_pos) {
            area += 1;
            perimeter += 4;
            garden[r][c] = MARKED;
            for (dr, dc) in DIRECTIONS.iter() {
                let next_r = r as i64 + dr;
                let next_c = c as i64 + dc;
                if is_in_bound(garden, next_r, next_c)
                    && garden[next_r as usize][next_c as usize] == label
                {
                    perimeter -= 2;
                    stack.push((next_r as usize, next_c as usize));
                }
            }
        }
    }

    (area, perimeter)
}

fn is_in_bound(garden: &[Vec<char>], r: i64, c: i64) -> bool {
    r >= 0 && c >= 0 && r < garden.len() as i64 && c < garden[0].len() as i64
}
