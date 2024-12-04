mod common;

use common::split_lines;
use crate::common::DIAG_DIRECTIONS;

fn main() {
    let input = include_str!("./input4.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let table = input
        .into_iter()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let xmas = "XMAS".chars().collect::<Vec<_>>();
    let m = table.len();
    let n = table[0].len();
    let mut count = 0;
    // She only has to find one word: XMAS.
    // This word search allows words to be horizontal, vertical, diagonal, written backwards,
    // or even overlapping other words.
    for r in 0..m {
        for c in 0..n {
            if table[r][c] == 'X' {
                for &(dr, dc) in DIAG_DIRECTIONS.iter() {
                    if check_direction(&table, r as i32, c as i32, dr, dc, &xmas) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn check_direction(table: &[Vec<char>], r: i32, c: i32, dr: i32, dc: i32, xmas: &[char]) -> bool {
    for i in 0..xmas.len() {
        let new_r = r + (dr * i as i32);
        let new_c = c + (dc * i as i32);

        if !is_in_bound(table, new_r, new_c) ||
            // Check if letter matches
            table[new_r as usize][new_c as usize] != xmas[i] {
            return false;
        }
    }
    true
}

fn is_in_bound(table: &[Vec<char>], r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && r < table.len() as i32 && c < table[0].len() as i32
}