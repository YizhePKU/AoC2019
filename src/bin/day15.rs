use aoc2019::IntcodeComputer;
use std::collections::HashSet;

/// Search the area for the oxygen system, avoiding spaces that have been
/// visited. Returns the location of the oxygen system if found.
fn search(
    pc: &mut IntcodeComputer,
    pos: (i64, i64),
    vis: &mut HashSet<(i64, i64)>,
) -> Option<(i64, i64)> {
    let mut oxygen_system = None;

    let commands = [
        (1, (0, 1), 2),
        (2, (0, -1), 1),
        (3, (-1, 0), 4),
        (4, (1, 0), 3),
    ];
    for (command, (dx, dy), reverse_command) in commands.into_iter() {
        let next_pos = (pos.0 + dx, pos.1 + dy);
        if !vis.contains(&next_pos) {
            pc.input.push_front(command);
            pc.run();
            let status = pc.output.pop_back().unwrap();

            if status != 0 {
                vis.insert(next_pos);

                if let Some(result) = search(pc, next_pos, vis) {
                    oxygen_system = Some(result);
                }

                pc.input.push_front(reverse_command);
                pc.run();
                let status = pc.output.pop_back().unwrap();
                assert_ne!(status, 0);
            }
            if status == 2 {
                oxygen_system = Some(next_pos);
            }
        }
    }

    oxygen_system
}

fn print_map(vis: &HashSet<(i64, i64)>, station: (i64, i64)) {
    let x_min = vis.iter().map(|p| p.0).min().unwrap();
    let x_max = vis.iter().map(|p| p.0).max().unwrap();
    let y_min = vis.iter().map(|p| p.1).min().unwrap();
    let y_max = vis.iter().map(|p| p.1).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if (x, y) == (0, 0) {
                print!("D");
            } else if (x, y) == station {
                print!("X");
            } else if vis.contains(&(x, y)) {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn spread(vis: &HashSet<(i64, i64)>, oxygen: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut result = oxygen.clone();
    for &(x, y) in vis {
        if oxygen.contains(&(x + 1, y))
            || oxygen.contains(&(x - 1, y))
            || oxygen.contains(&(x, y + 1))
            || oxygen.contains(&(x, y - 1))
        {
            result.insert((x, y));
        }
    }
    result
}

fn main() {
    let mut pc = IntcodeComputer::from_file("data/day15");
    let mut vis = HashSet::new();
    vis.insert((0, 0));
    let oxygen_station = search(&mut pc, (0, 0), &mut vis).unwrap();

    print_map(&vis, oxygen_station); // minimum distance is 246 by counting :)

    let mut oxygen = HashSet::new();
    oxygen.insert(oxygen_station);
    for step in 1.. {
        oxygen = spread(&vis, &oxygen);
        if oxygen.len() == vis.len() {
            dbg!(step);
            break;
        }
    }
}
