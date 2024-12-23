mod common;

use crate::common::split_lines;
use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("input23.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input {
        let pairs: Vec<&str> = line.split('-').collect();
        graph
            .entry(pairs[0].to_string())
            .or_insert_with(HashSet::new)
            .insert(pairs[1].to_string());
        graph
            .entry(pairs[1].to_string())
            .or_insert_with(HashSet::new)
            .insert(pairs[0].to_string());
    }

    let mut triangles = HashSet::new();
    for n1 in graph.keys() {
        for n2 in graph.get(n1).unwrap() {
            // Skip
            if n2 == n1 {
                continue;
            }
            for n3 in graph.get(n2).unwrap() {
                // Skip
                if n3 == n2 {
                    continue;
                }
                if graph.get(n1).unwrap().contains(n3) {
                    let mut nodes = vec![n1.clone(), n2.clone(), n3.clone()];
                    nodes.sort();
                    triangles.insert(nodes);
                }
            }
        }
    }

    triangles
        .iter()
        .filter(|nodes| nodes.iter().any(|n| n.starts_with('t')))
        .count() as u64
}
