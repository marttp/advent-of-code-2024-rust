mod common;

use crate::common::split_lines;
use std::collections::{HashMap, VecDeque};
fn main() {
    let input = include_str!("input24.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    // Prepare initial known input
    let mut input_gates: HashMap<String, u8> = HashMap::new();
    for input_gate_line in input.clone().iter().take_while(|line| !line.is_empty()) {
        let line = input_gate_line.split(": ").collect::<Vec<&str>>();
        input_gates.insert(
            line.first().unwrap().to_string(),
            line.last().unwrap().parse::<u8>().unwrap(),
        );
    }

    // Manage gate combination
    let mut gate_combinations: VecDeque<GateCombination> = VecDeque::new();
    for gate_combination_line in input
        .clone()
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
    {
        let line = gate_combination_line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let gate_combination = GateCombination::new(
            line[0].to_string(),
            line[2].to_string(),
            line[1].to_string(),
            line.last().unwrap().to_string(),
        );
        gate_combinations.push_back(gate_combination);
    }

    while let Some(gate_combination) = gate_combinations.pop_front() {
        if !input_gates.contains_key(&gate_combination.a)
            || !input_gates.contains_key(&gate_combination.b)
        {
            gate_combinations.push_back(gate_combination);
        } else {
            let a_value = input_gates.get(&gate_combination.a).unwrap();
            let b_value = input_gates.get(&gate_combination.b).unwrap();
            let result = match gate_combination.gate.as_str() {
                "AND" => Ok(a_value & b_value),
                "OR" => Ok(a_value | b_value),
                "XOR" => Ok(a_value ^ b_value),
                _ => Err(format!("Unknown operator {}", gate_combination.gate)),
            };
            match result {
                Ok(value) => {
                    input_gates.insert(gate_combination.output.to_string(), value);
                }
                Err(error) => {
                    panic!("{:?}", error);
                }
            }
        }
    }

    // Result will be control by z bits
    let mut result = 0u64;
    let mut z_input = input_gates
        .keys()
        .filter(|i| i.starts_with('z'))
        .collect::<Vec<_>>();
    // Expect sorting -> z00, z01, z02
    z_input.sort();
    for i in z_input {
        if input_gates.contains_key(i) && input_gates[i] == 1 {
            let bit_location = i.split('z').last().unwrap().parse::<u64>().unwrap();
            result |= 1u64 << bit_location;
        }
    }
    result
}

#[derive(Debug, Clone)]
struct GateCombination {
    pub a: String,
    pub b: String,
    pub gate: String,
    pub output: String,
}

impl GateCombination {
    fn new(a: String, b: String, gate: String, output: String) -> GateCombination {
        GateCombination { a, b, gate, output }
    }
}
