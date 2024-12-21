use crate::common::{solution, split_lines};

mod common;

fn main() {
    let input = include_str!("input21.txt");
    let output = solution(split_lines(input), 2);
    dbg!(output);
}
