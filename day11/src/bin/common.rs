use std::collections::HashMap;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::new();
    for (stone, amount) in stones {
        if *stone == 0 {
            *new.entry(1).or_default() += amount;
        } else {
            let digits = stone.to_string().len() as u32;
            if digits % 2 == 0 {
                let magnitude = 10u64.pow(digits / 2);
                // Right
                *new.entry(stone % magnitude)
                    .or_default() += amount;
                // Left
                *new.entry(stone / magnitude)
                    .or_default() += amount;
            } else {
                *new.entry(stone * 2024).or_default() += amount;
            }
        }
    }
    // Return new map
    new
}