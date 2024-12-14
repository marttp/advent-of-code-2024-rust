mod common;

use crate::common::{group_machine, split_lines};

fn main() {
    let input = include_str!("input13.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> i64 {
    let machines = group_machine(input);
    let mut coins: i64 = 0;

    for machine in machines {
        let (a, b, tmp_p) = (machine.button_a, machine.button_b, machine.prize);
        let p: (i64, i64) = (tmp_p.0 as i64 + 10000000000000, tmp_p.1 as i64 + 10000000000000);
        let time_b = ((p.1 * a.0 as i64) - (p.0 * a.1 as i64)) as f64 / (b.1 * a.0 - b.0 * a.1) as f64;
        let time_a = ((p.0 as f64) - (b.0 as f64 * time_b)) / a.0 as f64;

        if time_a.fract() == 0.0 && time_b.fract() == 0.0 {
            coins += (time_a * 3.0 + time_b) as i64;
        }
    }

    coins
}
