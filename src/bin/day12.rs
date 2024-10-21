use itertools::Itertools;
use regex::Regex;

fn update_velocity(
    position: &Vec<(i64, i64, i64)>,
    velocity: &Vec<(i64, i64, i64)>,
) -> Vec<(i64, i64, i64)> {
    let n = position.len();

    let mut result = velocity.clone();
    for i in 0..n {
        for j in 0..n {
            if position[j].0 > position[i].0 {
                result[i].0 += 1;
            } else if position[j].0 < position[i].0 {
                result[i].0 -= 1;
            }
            if position[j].1 > position[i].1 {
                result[i].1 += 1;
            } else if position[j].1 < position[i].1 {
                result[i].1 -= 1;
            }
            if position[j].2 > position[i].2 {
                result[i].2 += 1;
            } else if position[j].2 < position[i].2 {
                result[i].2 -= 1;
            }
        }
    }
    result
}

fn update_position(
    position: &Vec<(i64, i64, i64)>,
    velocity: &Vec<(i64, i64, i64)>,
) -> Vec<(i64, i64, i64)> {
    std::iter::zip(position.into_iter(), velocity.into_iter())
        .map(|(&p, &v)| (p.0 + v.0, p.1 + v.1, p.2 + v.2))
        .collect()
}

fn total_energy(position: &Vec<(i64, i64, i64)>, velocity: &Vec<(i64, i64, i64)>) -> i64 {
    let mut result = 0;
    for i in 0..position.len() {
        let potential_energy = position[i].0.abs() + position[i].1.abs() + position[i].2.abs();
        let kinetic_energy = velocity[i].0.abs() + velocity[i].1.abs() + velocity[i].2.abs();
        result += potential_energy * kinetic_energy;
    }
    result
}

fn update_velocity_single(position: &Vec<i64>, velocity: &Vec<i64>) -> Vec<i64> {
    let n = position.len();

    let mut result = velocity.clone();
    for i in 0..n {
        for j in 0..n {
            if position[j] > position[i] {
                result[i] += 1;
            } else if position[j] < position[i] {
                result[i] -= 1;
            }
        }
    }
    result
}

fn update_position_single(position: &Vec<i64>, velocity: &Vec<i64>) -> Vec<i64> {
    std::iter::zip(position.into_iter(), velocity.into_iter())
        .map(|(&p, &v)| (p + v))
        .collect()
}

fn find_cycle_single(init_position: &Vec<i64>, init_velocity: &Vec<i64>) -> u64 {
    let mut position = init_position.clone();
    let mut velocity = init_velocity.clone();
    for cycle in 0.. {
        velocity = update_velocity_single(&position, &velocity);
        position = update_position_single(&position, &velocity);
        if position == *init_position && velocity == *init_velocity {
            return cycle + 1;
        }
    }
    unreachable!()
}

fn gcd(x: u64, y: u64) -> u64 {
    if x < y {
        gcd(y, x)
    } else if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x / gcd(x, y) * y
}

fn main() {
    let input = std::fs::read("data/day12").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut position: Vec<(i64, i64, i64)> = vec![];
    for line in input.split_terminator("\r\n") {
        let re = Regex::new(r"<x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)>").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps["x"].parse().unwrap();
        let y = caps["y"].parse().unwrap();
        let z = caps["z"].parse().unwrap();
        position.push((x, y, z));
    }

    let mut velocity = vec![(0, 0, 0); position.len()];
    // for _step in 0..1000 {
    //     velocity = update_velocity(&position, &velocity);
    //     position = update_position(&position, &velocity);
    // }
    // dbg!(total_energy(&position, &velocity));

    let position_x = position.iter().map(|p| p.0).collect_vec();
    let position_y = position.iter().map(|p| p.1).collect_vec();
    let position_z = position.iter().map(|p| p.2).collect_vec();
    let velocity_x = velocity.iter().map(|v| v.0).collect_vec();
    let velocity_y = velocity.iter().map(|v| v.1).collect_vec();
    let velocity_z = velocity.iter().map(|v| v.2).collect_vec();

    let cycle_x = find_cycle_single(&position_x, &velocity_x);
    let cycle_y = find_cycle_single(&position_y, &velocity_y);
    let cycle_z = find_cycle_single(&position_z, &velocity_z);

    dbg!(lcm(lcm(cycle_x, cycle_y), cycle_z));
}
