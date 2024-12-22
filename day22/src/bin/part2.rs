mod common;

use crate::common::split_lines;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input22.txt");
    let output = solution_part2(split_lines(input));
    dbg!(output);
}

fn solution_part2(input: Vec<&str>) -> u64 {
    let initial_secrets: Vec<u64> = input
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // Pre-calculate all price sequences
    let all_price_sequences: Vec<Vec<u64>> = initial_secrets
        .par_iter()
        .map(|&secret| generate_price_sequence(secret, 2000))
        .collect();

    // Pre-calculate all change sequences
    let all_changes: Vec<Vec<i32>> = all_price_sequences
        .par_iter()
        .map(|prices| {
            prices
                .windows(2)
                .map(|w| (w[1] as i32) - (w[0] as i32))
                .collect()
        })
        .collect();

    // Using parallel iterator for the main computation
    // Generate sequence
    (-9..=9).into_par_iter()
        .flat_map(|a| (-9..=9).into_par_iter().map(move |b| (a, b)))
        .flat_map(|(a, b)| (-9..=9).into_par_iter().map(move |c| (a, b, c)))
        .flat_map(|(a, b, c)| (-9..=9).into_par_iter().map(move |d| (a, b, c, d)))
        .map(|(a, b, c, d)| {
            let sequence = vec![a, b, c, d];
            let total_bananas = calculate_total_bananas(&all_changes, &all_price_sequences, &sequence);
            total_bananas
        })
        .max()
        .unwrap_or(0)
}

fn generate_price_sequence(initial_secret: u64, iteration: u64) -> Vec<u64> {
    let mut prices = vec![initial_secret % 10];
    let mut secret = initial_secret;

    for _ in 0..iteration {
        secret = produce_next_secret(secret);
        prices.push(secret % 10);
    }
    prices
}

fn calculate_total_bananas(
    all_changes: &[Vec<i32>],
    all_price_sequences: &[Vec<u64>],
    target_sequence: &[i32]
) -> u64 {
    all_changes
        .iter()
        .zip(all_price_sequences)
        .map(|(changes, prices)| {
            for i in 0..=changes.len()-target_sequence.len() {
                if changes[i..i+target_sequence.len()] == *target_sequence {
                    return prices[i + target_sequence.len()];
                }
            }
            0
        })
        .sum()
}

fn produce_next_secret(mut secret: u64) -> u64 {
    secret = prune(mix(secret * 64, secret));
    secret = prune(mix((secret as f64 / 32f64).floor() as u64, secret));
    secret = prune(mix(secret * 2048, secret));
    secret
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}