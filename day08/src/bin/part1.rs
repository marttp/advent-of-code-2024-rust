mod common;

use crate::common::split_lines;
use std::collections::HashSet;
use std::usize;

const RESONANT: char = '#';
const EMPTY: char = '.';

fn main() {
    let input = include_str!("input8.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let mut grid: Vec<Vec<char>> = input
        .clone()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    // Outer -> Search element
    let mut total = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != RESONANT && grid[r][c] != EMPTY {
                total += mark_resonant(&mut grid, &mut seen, r, c);
            }
        }
    }
    total
}

fn mark_resonant(
    grid: &mut Vec<Vec<char>>,
    seen: &mut HashSet<(usize, usize)>,
    found_row: usize,
    found_col: usize,
) -> u32 {
    let target = grid[found_row][found_col];
    let mut result = 0;
    for r in found_row + 1..grid.len() {
        for c in 0..grid[r].len() {
            if target == grid[r][c] {
                let diff_row = usize::abs_diff(found_row, r);
                let diff_col = usize::abs_diff(found_col, c);

                let nr = r as i32 + diff_row as i32;
                let nc = if c <= found_col {
                    c as i32 - diff_col as i32
                } else {
                    c as i32 + diff_col as i32
                };
                if is_inbounds(&grid, nr, nc)&& seen.insert((nr as usize, nc as usize))  {
                    result += 1
                }
                let fr = found_row as i32 - diff_row as i32;
                let fc = if c <= found_col {
                    found_col as i32 + diff_col as i32
                } else {
                    found_col as i32 - diff_col as i32
                };
                if is_inbounds(&grid, fr, fc) && seen.insert((fr as usize, fc as usize)) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn is_inbounds(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32
}

fn count_resonant(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter()
        .map(|line| line.iter().filter(|c| **c == RESONANT).count() as u32)
        .sum()
}
