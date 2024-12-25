mod common;

use crate::common::split_lines;

fn main() {
    let input = include_str!("input25.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> usize {
    let mut all_key_pairs: Vec<KeyPair> = Vec::new();
    let mut current_grid: Vec<Vec<char>> = Vec::new();

    for line in input.clone() {
        if line.is_empty() {
            let key_pair = KeyPair::new(&current_grid);
            all_key_pairs.push(key_pair);
            current_grid = Vec::new();
        } else {
            current_grid.push(line.chars().collect());
        }
    }
    let key_pair = KeyPair::new(&current_grid);
    all_key_pairs.push(key_pair);

    let (locks, keys): (Vec<_>, Vec<_>) = all_key_pairs
        .clone()
        .into_iter()
        .partition(|key_pair| key_pair.category == "Lock");

    let mut result = 0;
    for key in keys.clone() {
        for lock in locks.clone() {
            if key
                .value
                .clone()
                .iter()
                .zip(&lock.available_slots)
                .all(|(k_slot, l_slot)| k_slot <= l_slot)
            {
                result += 1;
            }
        }
    }
    result
}

#[derive(Debug, Clone)]
struct KeyPair {
    category: String,
    value: Vec<usize>,
    available_slots: Vec<usize>,
}

impl KeyPair {
    fn new(input: &Vec<Vec<char>>) -> KeyPair {
        let category = match check_key(input) {
            true => "Key".to_string(),
            _ => "Lock".to_string(),
        };
        let mut value: Vec<usize> = Vec::new();
        let mut available_slots: Vec<usize> = Vec::new();
        let max_row = input.len() - 2;
        for col in 0..input[0].len() {
            let mut curr_row = 1usize;
            let mut amount = 0;
            while curr_row <= max_row {
                if input[curr_row][col] == '#' {
                    amount += 1;
                }
                curr_row += 1;
            }
            value.push(amount);
            if category == "Lock" {
                available_slots.push(max_row - amount);
            }
        }

        KeyPair {
            category,
            value,
            available_slots,
        }
    }
}
fn check_key(value: &Vec<Vec<char>>) -> bool {
    value.first().unwrap().clone().iter().all(|c| *c == '.')
}
