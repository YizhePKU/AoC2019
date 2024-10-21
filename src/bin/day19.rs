use aoc2019::IntcodeComputer;
use itertools::Itertools;
use std::{collections::HashSet, sync::LazyLock};

static PC: LazyLock<IntcodeComputer> = LazyLock::new(|| IntcodeComputer::from_file("data/day19"));

/// Deploy a drone to target position and returns whether it's being pulled.
fn scan(position: (usize, usize)) -> bool {
    let mut pc = PC.clone();
    pc.input.push_back(position.0 as i64);
    pc.input.push_back(position.1 as i64);
    pc.run();

    let output = pc.output.pop_back().unwrap();
    output == 1
}

fn main() {
    let mut part1 = 0;
    for i in 0..50 {
        for j in 0..50 {
            if scan((i, j)) {
                part1 += 1;
            }
        }
    }
    dbg!(part1);

    // make a scan of 1000x1000
    let scan_size = 2000;
    let mut beam = HashSet::new();
    for i in 0..scan_size {
        for j in 0..scan_size {
            if scan((i, j)) {
                beam.insert((i, j));
            }
        }
    }

    // look for an 100x100 square
    let target_size = 100;
    'outer: for i in 0..scan_size {
        for j in 0..scan_size {
            if (i..i + target_size)
                .cartesian_product(j..j + target_size)
                .all(|p| beam.contains(&p))
            {
                dbg!((i, j));
                break 'outer;
            }
        }
    }
}
