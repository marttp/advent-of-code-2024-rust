use crate::common::split_lines;
use std::collections::HashMap;

mod common;

fn main() {
    let input = include_str!("input14.txt");
    let output = solution(split_lines(input), 103, 101, 100);
    dbg!(output);
}

fn solution(input: Vec<&str>, tall: i64, wild: i64, seconds: u64) -> u64 {
    let mut robot_position: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut robot_velocity: HashMap<usize, (i64, i64)> = HashMap::new();
    let center_y = tall / 2;
    let center_x = wild / 2;
    for (id, line) in input.iter().enumerate() {
        let line_split = line.split_whitespace().collect::<Vec<_>>();
        let position = line_split
            .first()
            .unwrap()
            .strip_prefix("p=")
            .unwrap()
            .split(',')
            .collect::<Vec<_>>();
        if let (Ok(px), Ok(py)) = (position[0].parse::<i64>(), position[1].parse::<i64>()) {
            robot_position.insert(id, (px as usize, py as usize));
        }
        let velocity = line_split
            .last()
            .unwrap()
            .strip_prefix("v=")
            .unwrap()
            .split(',')
            .collect::<Vec<_>>();
        if let (Ok(vx), Ok(vy)) = (velocity[0].parse::<i64>(), velocity[1].parse::<i64>()) {
            robot_velocity.insert(id, (vx, vy));
        }
    }

    let mut remaining_seconds = seconds;

    while remaining_seconds > 0 {
        let mut new_robot_position: HashMap<usize, (usize, usize)> = HashMap::new();
        for (id, (px, py)) in robot_position.into_iter() {
            let (vx, vy) = robot_velocity.get(&id).unwrap();
            let mut nx = px as i64 + vx;
            if nx < 0 {
                let diff = 0_i64.abs_diff(nx);
                nx = wild - diff as i64;
            } else if nx >= wild {
                nx = nx % wild;
            }
            let mut ny = py as i64 + vy;
            if ny < 0 {
                let diff = 0_i64.abs_diff(ny);
                ny = tall - diff as i64;
            } else if ny >= tall {
                ny = ny % tall;
            }
            new_robot_position.insert(id, (nx as usize, ny as usize));
        }
        robot_position = new_robot_position;
        remaining_seconds -= 1;
    }

    // let mut grid = vec![vec![0; wild as usize]; tall as usize];
    // for (px, py) in robot_position.clone().into_values() {
    //     grid[py][px] += 1;
    // }
    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;

    for (px, py) in robot_position.into_values() {
        // Skip Center line on x, y
        if px != center_x as usize && py != center_y as usize {
            if px < center_x as usize && py < center_y as usize {
                // Q2
                q2_count += 1;
            } else if px < center_x as usize && py > center_y as usize {
                // Q3
                q3_count += 1;
            } else if px > center_x as usize && py < center_y as usize {
                // Q1
                q1_count += 1;
            } else {
                q4_count += 1;
            }
        }
    }
    println!("Q1: {}, Q2: {}, Q3: {}, Q4: {}", q1_count, q2_count, q3_count, q4_count);
    q1_count * q2_count * q3_count * q4_count
}
