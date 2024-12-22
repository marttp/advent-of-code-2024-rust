mod common;

use crate::common::split_lines;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input22.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    input
        .clone()
        .par_iter()
        .map(|&init_secret| produce_secret(init_secret.parse::<u64>().unwrap(), 2000))
        .sum()
}

/*
In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:

Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
Each step of the above process involves mixing and pruning:

To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation.
To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation.
After this process completes, the buyer is left with the next secret number in the sequence. The buyer can repeat this process as many times as necessary to produce more secret numbers.

*/
fn produce_secret(secret_input: u64, target: i32) -> u64 {
    let mut secret = secret_input;
    let mut current_iteration = 0;
    while current_iteration < target {
        let step_one_result = prune(mix(secret * 64, secret));
        secret = step_one_result;
        let step_two_result = prune(mix((secret as f64 / 32f64).floor() as u64, secret));
        secret = step_two_result;
        let step_three_result = prune(mix(secret * 2048, secret));
        secret = step_three_result;
        current_iteration += 1
    }
    secret
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
