mod common;

use crate::common::{shortest_path, split_lines};

fn main() {
    let input = include_str!("input18.txt");
    let output = solution(split_lines(input), 70, 70);
    dbg!(output);
}

fn solution(input: Vec<&str>, m: usize, n: usize) -> u64 {
    shortest_path(input, m, n, 1024)
}

#[allow(dead_code)]
fn display(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}