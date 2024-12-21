use std::collections::HashSet;

use pathfinding::directed::dijkstra::dijkstra;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }

    fn distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

struct Map {
    nodes: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut nodes = HashSet::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point::new(x as isize, y as isize);
                match c {
                    '.' => {
                        nodes.insert(point);
                    }
                    'S' => {
                        start = point;
                        nodes.insert(point);
                    }
                    'E' => {
                        end = point;
                        nodes.insert(point);
                    }
                    _ => {}
                }
            }
        }

        Map { nodes, start, end }
    }

    fn neighbors(&self, point: &Point) -> Vec<(Point, usize)> {
        point
            .neighbors()
            .into_iter()
            .filter(|p| self.nodes.contains(p))
            .map(|p| (p, 1))
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input20.txt");
    let map = Map::new(input);

    // Find the path.
    let now = std::time::Instant::now();
    let successors = |point: &Point| map.neighbors(point);
    let success = |point: &Point| *point == map.end;
    let (path, _) = dijkstra(&map.start, successors, success).ok_or("No path found")?;

    // create list of distances.
    let distances = path
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<Vec<_>>();

    // We can basically just iterator over the path and find and points further
    // down the path that are valid jumps and net us at least 100 picoseconds.
    let (p1, p2) = distances
        .par_iter()
        .enumerate()
        .map(|(i, (p1, d1))| {
            // Check all the subsequent points on the path.
            distances
                .iter()
                .skip(i + 1)
                .fold((0, 0), |mut acc, (p2, d2)| {
                    let distance = p1.distance(p2);
                    // For p1, we need to net 100 picoseconds and can only cheat 2 picoseconds.
                    if distance == 2 && *d2 - d1 - distance >= 100 {
                        acc.0 += 1;
                    }
                    // For p2, we need to net 100 picoseconds and can cheat up to 20 picoseconds.
                    if distance <= 20 && *d2 - d1 - distance >= 100 {
                        acc.1 += 1;
                    }
                    acc
                })
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("p1: {} ({:?})", p1, now.elapsed());
    println!("p2: {} ({:?})", p2, now.elapsed());

    Ok(())
}
