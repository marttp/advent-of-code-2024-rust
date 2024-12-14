mod common;

use crate::common::{group_machine, split_lines};

fn main() {
    let input = include_str!("input13.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i32 {
    let machines = group_machine(input);
    let mut coins = 0;

    for machine in machines {
        let (a, b, p) = (machine.button_a, machine.button_b, machine.prize);
        let time_b = ((p.1 * a.0) - (p.0 * a.1)) as f64 / ((b.1 * a.0 - b.0 * a.1)) as f64;
        let time_a = ((p.0 as f64) - (b.0 as f64 * time_b)) / a.0 as f64;

        if (0.0..=100.0).contains(&time_a) &&
            (0.0..=100.0).contains(&time_b) &&
            time_a.fract() == 0.0 &&
            time_b.fract() == 0.0 {
            coins += (time_a * 3.0 + time_b) as i32;
        }
    }

    coins
}
