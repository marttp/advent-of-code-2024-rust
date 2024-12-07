mod common;

use crate::common::{extract_input, split_lines, validate_equation};

fn main() {
    let input = include_str!("input7.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let (eq_results, numbers) = extract_input(input);
    eq_results
        .iter()
        .zip(numbers.iter())
        .filter(|(eq, num_list)| validate_equation(**eq, num_list, 1, num_list[0], true))
        .map(|(eq, _)| eq)
        .sum()
}
