mod common;

use crate::common::{parse, split_lines, VM};
use itertools::Itertools;

fn main() {
    let input = include_str!("input17.txt");
    let initial_vm = parse(split_lines(input));
    let part_two = part_two(initial_vm.clone());
    println!("Part 2: {:?}", part_two);
}

fn part_two(initial_vm: VM) -> u64 {
    find_a_matching_output(&initial_vm, &initial_vm.raw_instructions)
}

fn find_a_matching_output(initial_vm: &VM, target: &[u8]) -> u64 {
    let a_start = if target.len() == 1 {
        0
    } else {
        8 * find_a_matching_output(initial_vm, &target[1..])
    };

    let mut a = a_start;
    loop {
        let mut vm = initial_vm.clone();
        vm.a = a;
        let output = vm.run();
        if output == target {
            return a;
        }
        a += 1;
    }
}