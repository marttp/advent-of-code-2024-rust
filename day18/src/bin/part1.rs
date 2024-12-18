mod common;

use crate::common::split_lines;
use std::collections::VecDeque;

const EMPTY: char = '.';
const OBSTACLE: char = '#';

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let input = include_str!("input18.txt");
    let output = solution(split_lines(input), 70, 70, 1024);
    dbg!(output);
}

fn solution(input: Vec<&str>, m: usize, n: usize, byte_take: usize) -> u64 {
    let m = m + 1;
    let n = n + 1;
    let mut grid = vec![vec![EMPTY; n]; m];

    for line in input.clone().into_iter().take(byte_take) {
        let coord = line.split(",").collect::<Vec<&str>>();
        let c = coord[0].parse::<usize>().unwrap();
        let r = coord[1].parse::<usize>().unwrap();
        grid[r][c] = OBSTACLE;
    }

    let start = (0, 0);
    let end = (m - 1, n - 1);
    let mut visited = vec![vec![false; n]; m];
    let mut queue: VecDeque<((usize, usize), u64)> = VecDeque::new();
    queue.push_back((start, 0));

    let mut final_distance = 0;
    while let Some(((r, c), distance)) = queue.pop_front() {
        if (r == end.0) && (c == end.1) {
            final_distance = distance;
            break;
        }
        for (dr, dc) in DIRECTIONS.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 || grid[nr as usize][nc as usize] == OBSTACLE || visited[nr as usize][nc as usize] {
                continue;
            }
            visited[nr as usize][nc as usize] = true;
            queue.push_back(((nr as usize, nc as usize), distance + 1));
        }
    }
    final_distance
}

#[allow(dead_code)]
fn display(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}
