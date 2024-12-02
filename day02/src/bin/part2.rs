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
        .map(|line| line.split(' ').map(|w| w.parse::<u32>().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let safe_list = reports
        .iter()
        .map(|r| check_safe(&r))
        .collect::<Vec<bool>>();
    safe_list.iter().filter(|&&b| b).count() as u32
}

fn check_safe(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }

    if check_safe_main_logic(report) {
        return true;
    }

    /*
        More of the above example's reports are now safe:
        7 6 4 2 1: Safe without removing any level.
        1 2 7 8 9: Unsafe regardless of which level is removed.
        9 7 6 2 1: Unsafe regardless of which level is removed.
        1 3 2 4 5: Safe by removing the second level, 3.
        8 6 4 4 1: Safe by removing the third level, 4.
        1 3 6 7 9: Safe without removing any level.
        Thanks to the Problem Dampener, 4 reports are actually safe!
    */

    for i in 0..report.len() {
        let mut cloned_report = report.clone();
        cloned_report.remove(i);
        let is_safe = check_safe_main_logic(&cloned_report);
        if is_safe {
            return true;
        }
    }

    false
}

fn check_safe_main_logic(report: &Vec<u32>) -> bool {
    let mut left = 0;
    let mut right = 1;
    let is_asc = report[0] < report[1];

    while right < report.len() {
        let diff = u32::abs_diff(report[right], report[left]);
        if diff < 1 || diff > 3 {
            return false;
        }
        if (is_asc && report[right] < report[left]) || (!is_asc && report[right] > report[left]) {
            return false;
        }
        left += 1;
        right += 1;
    }

    true
}
