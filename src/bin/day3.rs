use std::collections::{HashMap, HashSet};

/// Returns all points of a wire, excluding the starting point.
fn wire2points(wire: &[(char, usize)]) -> HashSet<(i64, i64)> {
    let mut points = HashSet::new();

    let mut pos = (0, 0);
    for &(dir, distance) in wire {
        let (dx, dy) = if dir == 'U' {
            (0, 1)
        } else if dir == 'D' {
            (0, -1)
        } else if dir == 'L' {
            (-1, 0)
        } else if dir == 'R' {
            (1, 0)
        } else {
            unreachable!()
        };

        for _ in 0..distance {
            pos.0 += dx;
            pos.1 += dy;
            points.insert(pos);
        }
    }

    points
}

/// Returns all points of a wire with timing, excluding the starting point.
fn wire2points_time(wire: &[(char, usize)]) -> HashMap<(i64, i64), usize> {
    let mut points = HashMap::new();

    let mut pos = (0, 0);
    let mut time = 0;
    for &(dir, distance) in wire {
        let (dx, dy) = if dir == 'U' {
            (0, 1)
        } else if dir == 'D' {
            (0, -1)
        } else if dir == 'L' {
            (-1, 0)
        } else if dir == 'R' {
            (1, 0)
        } else {
            unreachable!()
        };

        for _ in 0..distance {
            time += 1;
            pos.0 += dx;
            pos.1 += dy;
            points.insert(pos, time);
        }
    }

    points
}

fn manhattan_distance(point1: (i64, i64), point2: (i64, i64)) -> usize {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    (i64::abs_diff(x1, x2) as usize) + (i64::abs_diff(y1, y2) as usize)
}

fn main() {
    let input = std::fs::read("data/day3").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut wires = vec![];
    for line in input.split_terminator("\r\n") {
        let mut wire: Vec<(char, usize)> = vec![];
        for instruction in line.split_terminator(',') {
            let dir = instruction.chars().next().unwrap();
            let distance = instruction[1..].parse().unwrap();
            wire.push((dir, distance));
        }
        wires.push(wire);
    }

    let points1 = wire2points(&wires[0]);
    let points2 = wire2points(&wires[1]);
    let part1 = points1
        .intersection(&points2)
        .map(|&point| manhattan_distance(point, (0, 0)))
        .min();
    dbg!(part1);

    let points1_time = wire2points_time(&wires[0]);
    let points2_time = wire2points_time(&wires[1]);
    let part2 = points1
        .intersection(&points2)
        .map(|p| points1_time[p] + points2_time[p])
        .min();
    dbg!(part2);
}
