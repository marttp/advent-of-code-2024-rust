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

fn get_file_spans(disk: &[Option<usize>]) -> Vec<(usize, usize, usize)> {
    let mut spans = Vec::new();
    let mut start = 0;
    let mut current_id = None;
    let mut length = 0;

    for (i, &block) in disk.iter().enumerate() {
        match (current_id, block) {
            (None, Some(id)) => {
                current_id = Some(id);
                start = i;
                length = 1;
            }
            (Some(id), Some(curr_id)) if id == curr_id => {
                length += 1;
            }
            (Some(id), _) => {
                spans.push((id, start, length));
                current_id = block;
                start = i;
                length = if block.is_some() { 1 } else { 0 };
            }
            (None, None) => continue,
        }
    }

    if let Some(id) = current_id {
        spans.push((id, start, length));
    }

    spans
}

fn find_leftmost_space(disk: &[Option<usize>], required_len: usize) -> Option<usize> {
    let mut current_space = 0;
    let mut space_start = None;

    for (i, &block) in disk.iter().enumerate() {
        if block.is_none() {
            if space_start.is_none() {
                space_start = Some(i);
            }
            current_space += 1;
            if current_space >= required_len {
                return space_start;
            }
        } else {
            current_space = 0;
            space_start = None;
        }
    }
    None
}

fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let mut file_spans = get_file_spans(disk);
    file_spans.sort_by_key(|&(id, _, _)| std::cmp::Reverse(id));

    for (id, start, length) in file_spans {
        if let Some(target) = find_leftmost_space(disk, length) {
            if target < start {
                // Clear old position
                for i in start..start + length {
                    disk[i] = None;
                }
                // Move to new position
                for i in target..target + length {
                    disk[i] = Some(id);
                }
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