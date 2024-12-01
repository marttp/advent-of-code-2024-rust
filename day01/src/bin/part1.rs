mod common;

use common::split_lines;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|pair| (pair[0], pair[1]))
        .unzip();

    let mut cloned_first = first_list.clone();
    cloned_first.sort();
    let mut cloned_second = second_list.clone();
    cloned_second.sort();

    cloned_first.iter().zip(cloned_second.iter())
        .map(|(a, b)| u32::abs_diff(*a, *b))
        .sum()
}
