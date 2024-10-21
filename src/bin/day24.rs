use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};

type Point = (i64, i64);
type Area = (Range<i64>, Range<i64>);

fn parse_input() -> (BTreeSet<Point>, Area) {
    let input = std::fs::read("data/day24").unwrap();
    let input = String::from_utf8(input).unwrap();

    let board = input
        .split_terminator("\r\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let n = board.len();
    let m = board[0].len();

    let mut points = BTreeSet::new();
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '#' {
                points.insert((i as i64, j as i64));
            }
        }
    }

    let area = (0..n as i64, 0..m as i64);

    (points, area)
}

fn neighbours(point: Point) -> [Point; 4] {
    let (i, j) = point;
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}

fn count_neighbours(points: &BTreeSet<Point>, point: Point) -> usize {
    neighbours(point)
        .into_iter()
        .filter(|p| points.contains(p))
        .count()
}

fn step(points: &BTreeSet<Point>, area: &Area) -> BTreeSet<Point> {
    let mut result = BTreeSet::new();
    for i in area.0.clone() {
        for j in area.1.clone() {
            let point = (i, j);
            let num_neighbours = count_neighbours(points, point);
            if points.contains(&point) && num_neighbours != 1 {
                // The bug dies.
            } else if !points.contains(&point) && (num_neighbours == 1 || num_neighbours == 2) {
                // The space becomes infected.
                result.insert(point);
            } else {
                // The space remains the same.
                if points.contains(&point) {
                    result.insert(point);
                }
            }
        }
    }
    result
}

fn print(points: &BTreeSet<Point>, area: &Area) {
    for i in area.0.clone() {
        for j in area.1.clone() {
            if points.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn biodiversity(points: &BTreeSet<Point>, area: &Area) -> u64 {
    let mut result = 0;
    let mut worth = 1;
    for i in area.0.clone() {
        for j in area.1.clone() {
            if points.contains(&(i, j)) {
                result += worth;
            }
            worth *= 2;
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RecursivePoint {
    depth: i8,
    label: i8, // 1~25, excluding 13
}

impl RecursivePoint {
    fn neighbours(self) -> Vec<Self> {
        let RecursivePoint { depth, label } = self;
        let mut result = vec![];

        // neighbour points at the same level
        if label % 5 != 0 && label != 12 {
            result.push(RecursivePoint {
                depth,
                label: label + 1,
            });
        }
        if label % 5 != 1 && label != 14 {
            result.push(RecursivePoint {
                depth,
                label: label - 1,
            });
        }
        if (label - 1) / 5 != 0 && label != 18 {
            result.push(RecursivePoint {
                depth,
                label: label - 5,
            });
        }
        if (label - 1) / 5 != 4 && label != 8 {
            result.push(RecursivePoint {
                depth,
                label: label + 5,
            });
        }

        // neighbour points at a outer level
        if [1, 2, 3, 4, 5].contains(&label) {
            result.push(RecursivePoint {
                depth: depth - 1,
                label: 8,
            });
        }
        if [21, 22, 23, 24, 25].contains(&label) {
            result.push(RecursivePoint {
                depth: depth - 1,
                label: 18,
            });
        }
        if [1, 6, 11, 16, 21].contains(&label) {
            result.push(RecursivePoint {
                depth: depth - 1,
                label: 12,
            });
        }
        if [5, 10, 15, 20, 25].contains(&label) {
            result.push(RecursivePoint {
                depth: depth - 1,
                label: 14,
            });
        }

        // neighbour points at a inner level
        if label == 8 {
            for inner in [1, 2, 3, 4, 5] {
                result.push(RecursivePoint {
                    depth: depth + 1,
                    label: inner,
                });
            }
        }
        if label == 18 {
            for inner in [21, 22, 23, 24, 25] {
                result.push(RecursivePoint {
                    depth: depth + 1,
                    label: inner,
                });
            }
        }
        if label == 12 {
            for inner in [1, 6, 11, 16, 21] {
                result.push(RecursivePoint {
                    depth: depth + 1,
                    label: inner,
                });
            }
        }
        if label == 14 {
            for inner in [5, 10, 15, 20, 25] {
                result.push(RecursivePoint {
                    depth: depth + 1,
                    label: inner,
                });
            }
        }

        result
    }
}

fn recursive_step(points: &BTreeSet<RecursivePoint>) -> BTreeSet<RecursivePoint> {
    let mut num_neighbours: BTreeMap<RecursivePoint, usize> = BTreeMap::new();
    for point in points {
        for neighbour in point.neighbours() {
            *num_neighbours.entry(neighbour).or_default() += 1;
        }
    }

    let mut result = BTreeSet::new();
    for (point, num) in num_neighbours {
        if num == 1 || (num == 2 && !points.contains(&point)) {
            result.insert(point);
        }
    }
    result
}

fn recursive_print(points: &BTreeSet<RecursivePoint>) {
    let min_depth = points.iter().map(|r| r.depth).min().unwrap();
    let max_depth = points.iter().map(|r| r.depth).max().unwrap();

    for depth in min_depth..=max_depth {
        println!("Depth {depth}:");
        for i in 0..5 {
            for j in 0..5 {
                let label = i * 5 + j + 1;
                if points.contains(&RecursivePoint { depth, label }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let (bugs, area) = parse_input();

    let mut current = bugs;
    let mut history = BTreeSet::new();
    loop {
        if history.contains(&current) {
            println!("part1 = {}", biodiversity(&current, &area));
            break;
        }
        let next = step(&current, &area);
        history.insert(current);
        current = next;
    }

    let (bugs, _area) = parse_input();
    let mut recursive_bugs = BTreeSet::new();
    for point in bugs {
        let label = (point.0 * 5 + point.1 + 1) as i8;
        recursive_bugs.insert(RecursivePoint { depth: 0, label });
    }

    for _ in 0..200 {
        recursive_bugs = recursive_step(&recursive_bugs);
    }
    recursive_print(&recursive_bugs);
    println!("part2 = {}", recursive_bugs.len());
}
