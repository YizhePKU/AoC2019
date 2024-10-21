use aoc2019::IntcodeComputer;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, io::Write};

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn reverse(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::North => f.write_str("north"),
            Dir::South => f.write_str("south"),
            Dir::West => f.write_str("west"),
            Dir::East => f.write_str("east"),
        }
    }
}

fn go(computer: &mut IntcodeComputer, path: &[Dir]) {
    for dir in path {
        let command = format!("{}\n", dir);
        for &byte in command.as_bytes() {
            computer.input.push_back(byte as i64);
        }
    }
}

fn reverse_path(path: &[Dir]) -> Vec<Dir> {
    let mut result = vec![];
    for dir in path.iter().rev() {
        result.push(dir.reverse());
    }
    result
}

/// Go where the item is, take it, then come back.
fn take_item(
    computer: &mut IntcodeComputer,
    items: &HashMap<&'static str, Vec<Dir>>,
    name: &'static str,
) {
    go(computer, &items[name]);

    let command = format!("take {}\n", name);
    for &byte in command.as_bytes() {
        computer.input.push_back(byte as i64);
    }

    go(computer, &reverse_path(&items[name]));
}

fn interact(computer: &mut IntcodeComputer) -> ! {
    let stdin = std::io::stdin();
    loop {
        computer.run();

        while let Some(byte) = computer.output.pop_front() {
            print!("{}", char::from_u32(byte as u32).unwrap());
        }
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        for &byte in buffer.as_bytes() {
            if byte != b'\r' {
                computer.input.push_back(byte as i64);
            }
        }
    }
}

fn main() {
    let items: HashMap<&'static str, Vec<Dir>> = HashMap::from_iter([
        ("tambourine", vec![Dir::North]),
        ("astrolabe", vec![Dir::North, Dir::East]),
        ("shell", vec![Dir::North, Dir::East, Dir::South]),
        (
            "klein bottle",
            vec![Dir::North, Dir::East, Dir::East, Dir::North],
        ),
        (
            "easter egg",
            vec![Dir::North, Dir::East, Dir::East, Dir::North, Dir::North],
        ),
        ("dark matter", vec![Dir::West]),
        // ("giant electromagnet", vec![Dir::West, Dir::West]),
        ("coin", vec![Dir::West, Dir::West, Dir::North, Dir::West]),
        ("hypercube", vec![Dir::South, Dir::South]),
    ]);

    let gate = vec![
        Dir::West,
        Dir::West,
        Dir::North,
        Dir::West,
        Dir::South,
        Dir::South,
    ];

    let mut computer = IntcodeComputer::from_file("data/day25");
    // for name in items.keys() {
    //     take_item(&mut computer, &items, name);
    // }
    // interact(&mut computer);

    for inventory in items.keys().powerset() {
        let mut computer = computer.clone();
        for name in &inventory {
            take_item(&mut computer, &items, name);
        }
        go(&mut computer, &gate);

        computer.run();

        let failure_message = "you are ejected back to the checkpoint";
        let output: String = computer
            .output
            .iter()
            .map(|&value| char::from_u32(value as u32).unwrap())
            .collect();
        if !output.contains(failure_message) {
            interact(&mut computer);
        } else {
            let msg = output
                .split_terminator("\n")
                .filter(|line| line.contains(failure_message))
                .next()
                .unwrap();
            println!("{}", msg);
        }
    }
}
