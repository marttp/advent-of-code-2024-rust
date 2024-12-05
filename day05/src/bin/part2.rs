mod common;

use common::{extract_rules_updates, middle_element, split_lines, valid_update};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input5.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i32 {
    let (_, updates, rule_map) = extract_rules_updates(input.clone());

    let updates: Vec<Vec<i32>> = updates
        .into_iter()
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    updates
        .iter()
        .filter(|u| !valid_update(u, &rule_map))
        .map(|u| fix_positions(u, &rule_map))
        .map(|u| middle_element(&u))
        .sum()
}

fn fix_positions(u: &Vec<i32>, rule_map: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    // rule_map -> possible next element

    let mut in_degrees: HashMap<i32, i32> = HashMap::new();
    for &num in u.iter() {
        in_degrees.entry(num).or_insert(0);
    }

    let set = u.iter().collect::<HashSet<_>>();
    for i in 0..u.len() {
        if let Some(rule_set) = rule_map.get(&u[i]) {
            for &allow_target in rule_set.iter() {
                if set.contains(&allow_target) {
                    *in_degrees.entry(allow_target).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    for (&node, &degree) in in_degrees.iter() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut result = Vec::new();
    let mut remaining_degrees = in_degrees.clone();

    while let Some(current) = queue.pop_front() {
        result.push(current);

        if let Some(rule_set) = rule_map.get(&current) {
            for &next in rule_set.iter() {
                if let Some(degree) = remaining_degrees.get_mut(&next) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    result
}
