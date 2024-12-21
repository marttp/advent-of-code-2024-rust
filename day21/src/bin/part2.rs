mod common;

use crate::common::{solution, split_lines};
fn main() {
    let input = include_str!("input21.txt");
    let output = solution(split_lines(input), 25);
    dbg!(output);
}
