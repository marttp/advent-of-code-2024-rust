mod common;

use common::split_lines;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|pair| (pair[0], pair[1]))
        .unzip();

    let mut counter_map: HashMap<u32, u32> = HashMap::new();
    for v1 in first_list.iter() {
        counter_map.insert(*v1, 0);
    }
    for v2 in second_list.iter() {
        counter_map.entry(*v2).and_modify(|e| *e += 1);
    }

    counter_map.iter().map(|(k, v)| k * v).sum()
}
