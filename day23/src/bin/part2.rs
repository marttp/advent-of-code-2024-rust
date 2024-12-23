mod common;

use crate::common::split_lines;
use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("input23.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> String {
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

    find_max_clique(&graph)
}

fn find_max_clique(graph: &HashMap<String, HashSet<String>>) -> String {
    let mut max_clique = HashSet::new();

    // Try from each starting vertex
    for start in graph.keys() {
        let mut clique = HashSet::from([start.clone()]);
        let mut candidates: HashSet<String> = graph[start].clone();

        while !candidates.is_empty() {
            let mut next = None;
            // Find a candidate connected to all clique members
            'candidate: for v in &candidates {
                for member in &clique {
                    if !graph[v].contains(member) {
                        continue 'candidate;
                    }
                }
                next = Some(v.clone());
                break;
            }

            match next {
                Some(v) => {
                    clique.insert(v.clone());
                    candidates.remove(&v);
                },
                None => break
            }
        }

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }

    let mut nodes: Vec<_> = max_clique.into_iter().collect();
    nodes.sort();
    nodes.join(",")
}
