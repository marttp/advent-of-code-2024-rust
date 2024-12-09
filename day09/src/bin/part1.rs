mod common;

use crate::common::split_lines;

fn main() {
    let input = include_str!("input9.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u64 {
    let line = input[0];
    let (files, spaces) = parse_disk_map(line);
    let mut disk = create_disk(&files, &spaces);
    compact_disk(&mut disk);
    calculate_checksum(&disk)
}

fn parse_disk_map(line: &str) -> (Vec<usize>, Vec<usize>) {
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    for (i, c) in line.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(num);
        } else {
            spaces.push(num);
        }
    }

    (files, spaces)
}

fn create_disk(files: &[usize], spaces: &[usize]) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    let mut file_id = 0;

    for i in 0..files.len() {
        // Add file blocks
        for _ in 0..files[i] {
            disk.push(Some(file_id));
        }
        file_id += 1;

        // Add space blocks if not last element
        if i < spaces.len() {
            for _ in 0..spaces[i] {
                disk.push(None);
            }
        }
    }

    disk
}

fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let len = disk.len();

    for i in (0..len).rev() {
        if let Some(file_id) = disk[i] {
            // Find leftmost free space
            let mut target = None;
            for j in 0..i {
                if disk[j].is_none() {
                    target = Some(j);
                    break;
                }
            }

            // Move file block if free space found
            if let Some(target_idx) = target {
                disk[target_idx] = Some(file_id);
                disk[i] = None;
            }
        }
    }
}

fn calculate_checksum(disk: &[Option<usize>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|id| pos as u64 * id as u64))
        .sum()
}