mod common;

use std::collections::HashMap;
use crate::common::{blink, split_lines};

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
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for i in initial_input {
        *stones.entry(i).or_default() += 1;
    }
    for _ in 0..75 {
        stones = blink(&stones);
    }
    stones.values().sum()
}

