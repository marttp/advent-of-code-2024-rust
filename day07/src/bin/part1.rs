mod common;

use crate::common::split_lines;

fn main() {
    let input = include_str!("input7.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let (eq_results, numbers): (Vec<&str>, Vec<&str>) = input
        .iter()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|pair| (pair[0], pair[1]))
        .unzip();
    let eq_results = eq_results
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let numbers = numbers
        .iter()
        .map(|list| {
            list.trim()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    eq_results
        .iter()
        .zip(numbers.iter())
        .filter(|(eq, num_list)| validate_equation(**eq, num_list, 1, num_list[0]))
        .map(|(eq, _)| eq)
        .sum()
}

fn validate_equation(equation_result: u64, num_list: &Vec<u64>, idx: usize, current: u64) -> bool {
    if num_list.len() == idx && equation_result == current {
        return true;
    } else if num_list.len() <= idx || equation_result > equation_result {
        return false
    }
    validate_equation(equation_result, num_list, idx + 1, current + num_list[idx])
        || validate_equation(equation_result, num_list, idx + 1, current * num_list[idx])
}
