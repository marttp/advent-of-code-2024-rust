mod common;

use crate::common::split_lines;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::usize;

const RESONANT: char = '#';
const EMPTY: char = '.';

fn main() {
    let input = include_str!("input8.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    let mut freq_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != RESONANT && ch != EMPTY {
                freq_positions.entry(ch).or_default().push((r, c));
            }
        }
    }

    // For each frequency
    for positions in freq_positions.values() {
        // For each pair of antennas of the same frequency
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                // Add the antennas themselves
                antinodes.insert((r1, c1));
                antinodes.insert((r2, c2));

                // Check all points for collinearity
                for r in 0..grid.len() {
                    for c in 0..grid[0].len() {
                        if is_collinear(r1, c1, r2, c2, r, c) {
                            antinodes.insert((r, c));
                        }
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

fn is_collinear(x1: usize, y1: usize, x2: usize, y2: usize, x3: usize, y3: usize) -> bool {
    let (x1, y1, x2, y2, x3, y3) = (
        x1 as i64, y1 as i64, x2 as i64, y2 as i64, x3 as i64, y3 as i64,
    );
    (x2 - x1) * (y3 - y1) == (y2 - y1) * (x3 - x1)
}
