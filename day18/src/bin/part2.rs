mod common;

use crate::common::{shortest_path, split_lines};

fn main() {
    let input = include_str!("input18.txt");
    let output = solution(split_lines(input), 70, 70);
    dbg!(output);
}

fn solution(input: Vec<&str>, m: usize, n: usize) -> String {
    let mut bytes: usize = 1;
    let size = input.len();

    while bytes < size {
        let result = shortest_path(input.clone(), m, n, bytes);
        if result == 0 {
            return input[bytes - 1].to_string();
        }
        bytes += 1;
    }

    "0,0".to_string()
}