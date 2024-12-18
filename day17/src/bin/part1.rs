mod common;

use crate::common::{parse, split_lines, VM};
use itertools::Itertools;

fn main() {
    let input = include_str!("input17.txt");
    let initial_vm = parse(split_lines(input));
    let part_one = part_one(initial_vm.clone());
    println!("Part 1: {:?}", part_one);
}

fn part_one(mut vm: VM) -> String {
    vm.run().into_iter().join(",")
}
