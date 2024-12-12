use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut grid: Vec<u8> = Vec::new();
    let mut width = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if width == 0 {
            width = line.len() + 2;
            grid.extend(iter::repeat_n(0, width));
        }
        grid.push(0);
        for c in line.into_bytes() {
            grid.push(c);
        }
        grid.push(0);
    }
    grid.extend(iter::repeat_n(0, width));
    let mut visited = vec![false; grid.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for (index, &id) in grid.iter().enumerate() {
        if id == 0 || visited[index] {
            continue;
        }
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;
        queue.push_back(index);
        visited[index] = true;
        while let Some(cur_index) = queue.pop_front() {
            area += 1;
            // The slice must be sorted CW or CCW.
            let neighbors = [
                cur_index - width,
                cur_index + 1,
                cur_index + width,
                cur_index - 1,
            ];
            for (i, neighbor) in neighbors.into_iter().enumerate() {
                if grid[neighbor] == id {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                } else {
                    perimeter += 1;
                }

                let next_neighbor = neighbors[(i + 1) % neighbors.len()];
                if grid[neighbor] != id && grid[next_neighbor] != id {
                    sides += 1; // Outer angle.
                } else if grid[neighbor] == id
                    && grid[next_neighbor] == id
                    && grid[neighbor + next_neighbor - cur_index] != id
                {
                    sides += 1; // Inner angle.
                }
            }
        }
        ans1 += area * perimeter;
        ans2 += area * sides;
    }
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}