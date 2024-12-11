mod common;

use crate::common::{first_rule_producer, split_lines};

fn main() {
    let input = include_str!("input11.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let initial_input = input
        .first()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let result = first_rule_producer(&initial_input, 25);
    result.len() as u64
}