mod common;

use common::split_lines;

fn main() {
    let input = include_str!("./input2.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let reports = input
        .into_iter()
        .map(|line| line.split_whitespace().filter_map(|w| w.parse::<u32>().ok()).collect())
        .collect::<Vec<Vec<u32>>>();
    let safe_list = reports
        .iter()
        .map(|r| check_safe(&r))
        .collect::<Vec<bool>>();
    safe_list.iter().filter(|&&b| b).count() as u32
}

fn check_safe(report: &Vec<u32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    if is_valid_sequence(report) {
        return true;
    }
    for i in 0..report.len() {
        if is_valid_sequence_with_skip(report, i) {
            return true;
        }
    }
    return false;
}

fn is_valid_sequence(report: &Vec<u32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let increasing = report[1] > report[0];
    report.windows(2).all(|pair| {
        let diff = u32::abs_diff(pair[0], pair[1]);
        diff >= 1 && diff <= 3 && (increasing == (pair[1] > pair[0]))
    })
}

fn is_valid_sequence_with_skip(report: &Vec<u32>, skip_index: usize) -> bool {
    if report.len() <= 2 {
        return true;
    }

    let mut prev = None;
    let mut first_pair = true;
    let mut increasing = false;

    for (i, &num) in report.iter().enumerate() {
        if i == skip_index {
            continue;
        }

        if let Some(previous) = prev {
            let diff = u32::abs_diff(previous, num);
            if diff < 1 || diff > 3 {
                return false;
            }

            if first_pair {
                increasing = num > previous;
                first_pair = false;
            } else if increasing != (num > previous) {
                return false;
            }
        }

        prev = Some(num);
    }

    true
}