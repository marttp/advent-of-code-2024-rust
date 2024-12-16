use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() {
    let input = include_str!("input16.txt");
    first_star(input).unwrap();
    second_star(input).unwrap();
}

#[derive(Debug, Clone)]
struct Reindeer {
    position: (usize, usize),
    facing: Direction,
    score: usize,
    previous: Vec<(usize, usize)>,
}

impl Reindeer {
    fn new() -> Self {
        Reindeer {
            position: (0, 0),
            facing: Direction::East,
            score: 0,
            previous: vec![],
        }
    }

    fn next_step(&self, new_pos: Option<(usize, usize)>, new_direction: Option<Direction>) -> Self {
        let mut score = self.score;
        let mut pos = self.position;
        let mut face = self.facing;
        let mut previous = self.previous.clone();

        if let Some(position) = new_pos {
            pos = position;
            previous.push(self.position);
            score += 1;
        }

        if let Some(dir) = new_direction {
            face = dir;
            score += 1_000;
        }

        Reindeer {
            position: pos,
            facing: face,
            score,
            previous,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_input(input: &str) -> (HashSet<(usize, usize)>, Reindeer, (usize, usize)) {
    let mut walls = HashSet::new();
    let mut start = Reindeer::new();
    let mut end = (0, 0);
    for (line_no, line) in input.trim().lines().enumerate() {
        for (col_no, elem) in line.char_indices() {
            match elem {
                '#' => {
                    walls.insert((line_no, col_no));
                }
                'S' => {
                    start.position = (line_no, col_no);
                }
                'E' => {
                    end = (line_no, col_no);
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }
    (walls, start, end)
}

pub fn first_star(input: &str) -> Result<(), Box<dyn Error + 'static>> {
    let (walls, reindeer, end_pos) = get_input(input);

    let mut to_do: Vec<Reindeer> = vec![];
    to_do.push(reindeer);

    let mut reached: HashMap<(usize, usize, Direction), usize> = HashMap::new();

    while let Some(current) = to_do.pop() {
        if current.position == end_pos {
            println!("The lowest score the deer can get is: {}", current.score);
            break;
        }

        let previous = reached
            .entry((current.position.0, current.position.1, current.facing))
            .or_insert(usize::MAX);
        if *previous < current.score {
            continue;
        } else {
            *previous = current.score;
        }

        let (forward_pos, left_neighbor, right_neighbor) = match current.facing {
            Direction::North => (
                (current.position.0 - 1, current.position.1),
                (current.position.0, current.position.1 - 1, Direction::West),
                (current.position.0, current.position.1 + 1, Direction::East),
            ),
            Direction::East => (
                (current.position.0, current.position.1 + 1),
                (current.position.0 - 1, current.position.1, Direction::North),
                (current.position.0 + 1, current.position.1, Direction::South),
            ),
            Direction::South => (
                (current.position.0 + 1, current.position.1),
                (current.position.0, current.position.1 + 1, Direction::East),
                (current.position.0, current.position.1 - 1, Direction::West),
            ),
            Direction::West => (
                (current.position.0, current.position.1 - 1),
                (current.position.0 + 1, current.position.1, Direction::South),
                (current.position.0 - 1, current.position.1, Direction::North),
            ),
        };

        if !walls.contains(&forward_pos) {
            to_do.push(current.next_step(Some(forward_pos), None));
        }
        if !walls.contains(&(left_neighbor.0, left_neighbor.1)) {
            to_do.push(current.next_step(None, Some(left_neighbor.2)));
        }
        if !walls.contains(&(right_neighbor.0, right_neighbor.1)) {
            to_do.push(current.next_step(None, Some(right_neighbor.2)));
        }

        to_do.sort_by(|a, b| b.score.cmp(&a.score));
    }

    Ok(())
}

pub fn second_star(input: &str) -> Result<(), Box<dyn Error + 'static>> {
    let (walls, reindeer, end_pos) = get_input(input);

    let mut to_do: Vec<Reindeer> = vec![];
    to_do.push(reindeer);

    let mut reached: HashMap<(usize, usize, Direction), usize> = HashMap::new();
    let mut lowest_score = usize::MAX;

    let mut covered_tiles: HashSet<(usize, usize)> = HashSet::new();
    covered_tiles.insert(end_pos);

    while let Some(current) = to_do.pop() {
        if current.position == end_pos && lowest_score >= current.score {
            lowest_score = current.score;
            for pos in current.previous.iter() {
                covered_tiles.insert(*pos);
            }
            continue;
        }

        let previous = reached
            .entry((current.position.0, current.position.1, current.facing))
            .or_insert(lowest_score);

        if *previous < current.score || current.score > lowest_score {
            continue;
        } else {
            *previous = current.score;
        }

        let (forward_pos, left_neighbor, right_neighbor) = match current.facing {
            Direction::North => (
                (current.position.0 - 1, current.position.1),
                (current.position.0, current.position.1 - 1, Direction::West),
                (current.position.0, current.position.1 + 1, Direction::East),
            ),
            Direction::East => (
                (current.position.0, current.position.1 + 1),
                (current.position.0 - 1, current.position.1, Direction::North),
                (current.position.0 + 1, current.position.1, Direction::South),
            ),
            Direction::South => (
                (current.position.0 + 1, current.position.1),
                (current.position.0, current.position.1 + 1, Direction::East),
                (current.position.0, current.position.1 - 1, Direction::West),
            ),
            Direction::West => (
                (current.position.0, current.position.1 - 1),
                (current.position.0 + 1, current.position.1, Direction::South),
                (current.position.0 - 1, current.position.1, Direction::North),
            ),
        };

        if !walls.contains(&forward_pos) {
            to_do.push(current.next_step(Some(forward_pos), None));
        }
        if !walls.contains(&(left_neighbor.0, left_neighbor.1)) {
            to_do.push(current.next_step(None, Some(left_neighbor.2)));
        }
        if !walls.contains(&(right_neighbor.0, right_neighbor.1)) {
            to_do.push(current.next_step(None, Some(right_neighbor.2)));
        }

        to_do.sort_by(|a, b| b.score.cmp(&a.score));
    }

    println!(
        "There are {} tiles that are part of at least one best path",
        covered_tiles.len()
    );

    Ok(())
}