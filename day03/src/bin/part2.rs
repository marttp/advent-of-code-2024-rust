mod common;

use common::split_lines;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input3.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let binding = input.concat();
    let concat_str = binding.as_str();

    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let mul_regex = Regex::new(mul_pattern).unwrap();
    let mut mul_queue = mul_regex
        .find_iter(concat_str)
        .collect::<VecDeque<_>>();

    let command_pattern = r"do(?:n't)?\(\)";
    let command_regex = Regex::new(command_pattern).unwrap();
    let mut command_queue = command_regex
        .find_iter(concat_str)
        .collect::<VecDeque<_>>();

    let mut is_multiply_enable = true;
    let mut sum = 0;

    while let Some(matcher) = mul_queue.pop_front() {
        let start_idx = matcher.start();

        while !command_queue.is_empty() && command_queue.front().unwrap().start() < start_idx {
            let command = command_queue.pop_front().unwrap();
            is_multiply_enable = command.as_str() == "do()";
        }

        if is_multiply_enable {
            let capture = mul_regex.captures(matcher.as_str()).unwrap();
            let op1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let op2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            sum += op1 * op2;
        }
    }

    sum
}
