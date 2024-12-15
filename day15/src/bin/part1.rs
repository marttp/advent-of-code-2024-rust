use crate::common::split_lines;
use std::collections::HashMap;

mod common;

fn main() {
    let input = include_str!("input15.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<char> = Vec::new();
    let mut is_read_direction = false;
    for line in input {
        if line.is_empty() {
            is_read_direction = true;
            continue;
        }
        if is_read_direction {
            directions.extend(line.chars());
        } else {
            grid.push(line.chars().collect());
        }
    }

    let direction_index: HashMap<char, i32> = create_direction_index();
    let cross_direction_index: HashMap<char, i32> = create_cross_direction_index();
    let direction_list: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (row, col) = robot_position(&grid);

    let mut curr_row = row;
    let mut curr_col = col;
    for direction in directions {
        let (dr, dc) = direction_list[direction_index[&direction] as usize];
        let new_row = curr_row as i32 + dr;
        let new_col = curr_col as i32 + dc;
        if is_in_bound(&grid, new_row, new_col) {
            // Count box on that direction
            let mut br = new_row;
            let mut bc = new_col;
            let mut count = 0;
            while is_in_bound(&grid, br, bc) && grid[br as usize][bc as usize] == 'O' {
                br += dr;
                bc += dc;
                count += 1;
            }
            let is_space = grid[br as usize][bc as usize] == '.';
            if is_space && count > 0 {
                let (cr, cc) = direction_list[cross_direction_index[&direction] as usize];
                // Push box
                let mut pr = br + cr;
                let mut pc = bc + cc;
                while count > 0 {
                    println!("Push box from ({}, {}) to ({}, {})", pr, pc, br, bc);
                    grid[br as usize][bc as usize] = 'O';
                    grid[pr as usize][pc as usize] = '.';
                    br = pr;
                    bc = pc;
                    pr = pr + cr;
                    pc = pc + cc;
                    count -= 1;
                }
            }
            // Replace
            if is_space {
                grid[curr_row][curr_col] = '.';
                grid[new_row as usize][new_col as usize] = '@';
                curr_row = new_row as usize;
                curr_col = new_col as usize;
            }
        }
        // Out of bound - do nothing
    }
    sum_coordinate(&grid)
}

fn sum_coordinate(grid: &Vec<Vec<char>>) -> u64 {
    let mut scores = 0;
    for (row, row_data) in grid.iter().enumerate() {
        for (col, cell) in row_data.iter().enumerate() {
            if *cell == 'O' {
                let score_row = 100u64 * 0u64.abs_diff(row as u64);
                let score_col = 0u64.abs_diff(col as u64);
                scores += score_row + score_col;
            }
        }
    }
    scores
}

fn is_in_bound(table: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0
        && c >= 0
        && r < table.len() as i32
        && c < table[0].len() as i32
        // Border
        && table[r as usize][c as usize] != '#'
}

fn robot_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (row, row_data) in grid.iter().enumerate() {
        for (col, cell) in row_data.iter().enumerate() {
            if *cell == '@' {
                return (row, col);
            }
        }
    }
    (0, 0)
}

fn display_grid(grid: &Vec<Vec<char>>) {
    println!();
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn create_direction_index() -> HashMap<char, i32> {
    let mut directions = HashMap::new();
    directions.insert('^', 0);
    directions.insert('v', 1);
    directions.insert('<', 2);
    directions.insert('>', 3);
    directions
}

fn create_cross_direction_index() -> HashMap<char, i32> {
    let mut directions = HashMap::new();
    directions.insert('^', 1);
    directions.insert('v', 0);
    directions.insert('<', 3);
    directions.insert('>', 2);
    directions
}
