mod common;

use crate::common::split_lines;

fn main() {
    let input = include_str!("input19.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> usize {
    let lines = input
        .clone()
        .first()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    input
        .clone()
        .into_iter()
        .skip(2)
        .map(|word| count_construct(word, &lines))
        .sum()
}

fn count_construct(pattern: &str, flags: &Vec<String>) -> usize {
    let length = pattern.len();
    let mut dp = vec![0; length + 1];
    dp[0] = 1;

    for i in 0..length {
        for f in flags.clone().into_iter() {
            let next_length = i + f.len();
            if next_length <= length && pattern[i..next_length] == f[..] {
                dp[next_length] += dp[i];
            }
        }
    }
    dp[length]
}
