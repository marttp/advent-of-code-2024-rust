mod common;

use std::collections::{HashMap, HashSet};
use common::{split_lines, extract_rules_updates};

fn main() {
    let input = include_str!("./input5.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i32 {
    let (rules, updates) = extract_rules_updates(input.clone());
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in rules {
        let order_rule: Vec<i32> = rule.split('|')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        rule_map.entry(order_rule[0])
            .or_insert(HashSet::new())
            .insert(order_rule[1]);
    }

    let updates: Vec<Vec<i32>> = updates.into_iter()
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    updates.iter().filter(|u| valid_update(u, &rule_map))
        .map(|u| middle_element(u))
        .sum()
}

fn valid_update(u: &Vec<i32>, rule_map: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 0..u.len() {
        if let Some(rule_set) = rule_map.get(&u[i]) {
            for j in 0..i {
                if rule_set.contains(&u[j]) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn middle_element(u: &Vec<i32>) -> i32 {
    let length = u.len();
    u[length/2]
}