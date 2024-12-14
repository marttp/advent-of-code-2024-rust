pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug, Clone, Copy)]
pub struct Machine {
    pub button_a: (i32, i32),
    pub button_b: (i32, i32),
    pub prize: (i32, i32),
}

impl Machine {
    pub fn new(button_a: (i32, i32), button_b: (i32, i32), prize: (i32, i32)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

pub fn group_machine(input: Vec<&str>) -> Vec<Machine> {
    let mut groups: Vec<Machine> = Vec::new();

    let chunk_groups: Vec<Vec<&str>> = input
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.to_vec().drain(..3).collect())
        .collect();

    for group in chunk_groups {
        let a = group[0]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();
        let ax = a[0].split("+").last().unwrap().parse::<usize>().unwrap();
        let ay = a[1].split("+").last().unwrap().parse::<usize>().unwrap();

        let b = group[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();
        let bx = b[0].split("+").last().unwrap().parse::<usize>().unwrap();
        let by = b[1].split("+").last().unwrap().parse::<usize>().unwrap();

        let p = group[2]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();
        let px = p[0].split("=").last().unwrap().parse::<usize>().unwrap();
        let py = p[1].split("=").last().unwrap().parse::<usize>().unwrap();

        let machine = Machine::new(
            (ax as i32, ay as i32),
            (bx as i32, by as i32),
            (px as i32, py as i32),
        );
        groups.push(machine);
    }

    groups
}
