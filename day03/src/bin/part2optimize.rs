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

    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)|do(?:n't)?\(\)";
    let mul_regex = Regex::new(mul_pattern).unwrap();
    let match_list = mul_regex
        .find_iter(concat_str)
        .collect::<Vec<_>>();

    let mut is_multiply_enable = true;
    let mut sum = 0;

    for matcher in match_list {
        match matcher.as_str() {
            "do()" => is_multiply_enable = true,
            "don't()" => is_multiply_enable = false,
            _ => {
                if is_multiply_enable {
                    let capture = mul_regex.captures(matcher.as_str()).unwrap();
                    let op1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let op2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    sum += op1 * op2;
                }
            }
        }
    }

    sum
}
