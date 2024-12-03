mod common;

use common::split_lines;
use regex::Regex;

fn main() {
    let input = include_str!("./input3.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let binding = input.concat();
    let concat_str = binding.as_str();
    // find 'mul' then 2 pointers find '(', ')' then cut prefix
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();

    regex.captures_iter(concat_str)
        .map(|cap| &cap[1].parse::<u32>().unwrap() * &cap[2].parse::<u32>().unwrap())
        .sum()
}
