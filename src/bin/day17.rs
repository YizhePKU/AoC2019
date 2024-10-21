use aoc2019::IntcodeComputer;
use itertools::Itertools;
use std::collections::HashSet;

fn is_intersection(point: (usize, usize), scaffolds: &HashSet<(usize, usize)>) -> bool {
    let (i, j) = point;
    if !scaffolds.contains(&point) {
        return false;
    }
    if i > 0 && !scaffolds.contains(&(i - 1, j)) {
        return false;
    }
    if !scaffolds.contains(&(i + 1, j)) {
        return false;
    }
    if j > 0 && !scaffolds.contains(&(i, j - 1)) {
        return false;
    }
    if !scaffolds.contains(&(i, j + 1)) {
        return false;
    }
    return true;
}

fn main() {
    let mut computer = IntcodeComputer::from_file("data/day17");
    computer.run();

    let text = String::from_utf8(computer.output.iter().map(|&x| x as u8).collect_vec()).unwrap();
    let grid = text
        .trim()
        .split_whitespace()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut scaffolds = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                scaffolds.insert((i, j));
            }
        }
    }

    let mut alignment_parameter_sum = 0;
    for &point in &scaffolds {
        if is_intersection(point, &scaffolds) {
            alignment_parameter_sum += point.0 * point.1;
        }
    }
    dbg!(alignment_parameter_sum);

    let mut computer = IntcodeComputer::from_file("data/day17");
    computer.mem[0] = 2;

    let instruction = std::fs::read("data/day17_instruction").unwrap();
    for byte in instruction {
        computer.input.push_back(byte as i64);
    }
    
    computer.run();
    dbg!(computer.output);
}
