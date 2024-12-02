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
    let safe_list = reports.iter()
        .map(|r| check_safe(&r))
        .collect::<Vec<bool>>();

    safe_list.iter().filter(|&&b| b).count() as u32
}

fn check_safe(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }

    let mut left = 0;
    let mut right = 1;
    let is_asc = report[0] < report[1];

    while right < report.len() {
        let diff = u32::abs_diff(report[right], report[left]);
        if diff < 1 || diff > 3 {
            return false;
        }
        if (is_asc && report[right] < report[left]) || (!is_asc && report[right] > report[left]){
            return false;
        }
        left += 1;
        right += 1;
    }

    true
}
