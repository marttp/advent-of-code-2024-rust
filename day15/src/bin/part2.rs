use crate::common::split_lines;
use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;

mod common;

fn main() {
    let input = include_str!("input15.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<u8> = Vec::new();
    let mut is_read_direction = false;
    for line in input {
        if line.is_empty() {
            is_read_direction = true;
            continue;
        }
        if is_read_direction {
            directions.extend(line.as_bytes());
        } else {
            grid.push(line.chars().collect());
        }
    }

    // Extend grid
    let mut extended_grid = Vec::new();
    for row in grid.clone() {
        let mut new_row: Vec<u8> = Vec::new();
        for c in row {
            match c {
                '#' => new_row.extend("##".as_bytes()),
                'O' => new_row.extend("[]".as_bytes()),
                '@' => new_row.extend("@.".as_bytes()),
                _ => new_row.extend("..".as_bytes()),
            }
        }
        extended_grid.push(new_row);
    }

    let (mut r, mut c) = (0, 0);
    for rr in 0..extended_grid.len() {
        for cc in 0..extended_grid[0].len() {
            if extended_grid[rr][cc] == b'@' {
                extended_grid[rr][cc] = b'.';
                (r, c) = (rr, cc);
            }
        }
    }

    'outer: for i in directions {
        let (dr, dc): (i32, i32) = match i {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };
        let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
        let mut q = VecDeque::from([(r, c)]);
        let mut seen = HashSet::new();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            // let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            let (r2, c2) = ((rr as i32 + dr) as usize, (cc as i32 + dc) as usize);
            match extended_grid[r2][c2] {
                b'#' => continue 'outer,
                b'O' => {
                    q.push_back((r2, c2));
                }
                b'[' => {
                    q.push_back((r2, c2));
                    q.push_back((r2, c2 + 1));
                }
                b']' => {
                    q.push_back((r2, c2));
                    q.push_back((r2, c2 - 1));
                }
                _ => continue,
            }
        }
        while !seen.is_empty() {
            for (rr, cc) in seen.iter().copied().sorted() {
                let (r2, c2) = ((rr as i32 + dr) as usize, (cc as i32 + dc) as usize);
                if !seen.contains(&(r2, c2)) {
                    extended_grid[r2][c2] = extended_grid[rr][cc];
                    extended_grid[rr][cc] = b'.';
                    seen.remove(&(rr, cc));
                }
            }
        }
        (r, c) = (rr, cc);
    }

    sum_coordinate(&extended_grid)
}

fn sum_coordinate(grid: &Vec<Vec<u8>>) -> u64 {
    let mut scores = 0;
    for (row, row_data) in grid.iter().enumerate() {
        for (col, cell) in row_data.iter().enumerate() {
            if *cell == b'[' {
                let score_row = 100u64 * (row as u64);
                let score_col = col as u64;
                scores += score_row + score_col;
            }
        }
    }
    scores
}