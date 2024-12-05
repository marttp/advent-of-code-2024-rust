use std::collections::{HashMap, HashSet};

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn extract_rules_updates(input: Vec<&str>) -> (Vec<&str>, Vec<&str>, HashMap<i32, HashSet<i32>>) {
    let mut rules: Vec<&str> = Vec::new();
    let mut updates: Vec<&str> = Vec::new();
    let mut is_received_update: bool = false;
    for line in input {
        match line.is_empty() {
            true => is_received_update = true,
            _ => {
                if is_received_update {
                    updates.push(line);
                } else {
                    rules.push(line);
                }
            }
        }
    }
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in rules.clone() {
        let order_rule: Vec<i32> = rule.split('|')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        rule_map.entry(order_rule[0])
            .or_insert(HashSet::new())
            .insert(order_rule[1]);
    }

    (rules, updates, rule_map)
}

pub fn valid_update(u: &Vec<i32>, rule_map: &HashMap<i32, HashSet<i32>>) -> bool {
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

pub fn middle_element(u: &Vec<i32>) -> i32 {
    let length = u.len();
    u[length/2]
}