use crate::common::split_lines;
use std::collections::HashMap;
use image::{ImageBuffer, Rgb};

mod common;

fn main() {
    let input = include_str!("input14.txt");
    solution(split_lines(input), 103, 101);
}

fn solution(input: Vec<&str>, tall: i64, wild: i64) {
    let mut robot_position: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut robot_velocity: HashMap<usize, (i64, i64)> = HashMap::new();
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

    let mut seconds = 0;
    // I want to save 10000 frames
    while seconds < 10000 {
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

        display_grid(&robot_position, wild, tall, seconds);
        seconds += 1;
    }
    // After this step, use your file system and sort to find minimum file size
}

fn display_grid(robot_position: &HashMap<usize, (usize, usize)>, wild: i64, tall: i64, seconds: i64) {
    let mut grid = vec![vec![0; wild as usize]; tall as usize];
    for (px, py) in robot_position.clone().into_values() {
        grid[py][px] += 1;
    }

    let mut img = ImageBuffer::new(wild as u32, tall as u32);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = if cell == 0 {
                Rgb([0u8, 0u8, 0u8])
            } else {
                Rgb([255u8, 255u8, 255u8])
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    let filename = format!("frame_{:04}.png", seconds);
    img.save(&filename).expect("Failed to save image");
}
