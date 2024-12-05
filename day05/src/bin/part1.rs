mod common;

use common::{extract_rules_updates, middle_element, split_lines, valid_update};

fn main() {
    let input = include_str!("./input5.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i32 {
    let (_, updates, rule_map) = extract_rules_updates(input.clone());

    let updates: Vec<Vec<i32>> = updates.into_iter()
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    updates.iter().filter(|u| valid_update(u, &rule_map))
        .map(|u| middle_element(u))
        .sum()
}