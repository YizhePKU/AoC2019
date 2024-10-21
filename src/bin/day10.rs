use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

/// Returns true if the vision from `src` to `tgt` is perfectly blocked by `obstacle`.
fn perfectly_blocked(src: (usize, usize), tgt: (usize, usize), obstacle: (usize, usize)) -> bool {
    let (x1, y1) = src;
    let (x2, y2) = tgt;
    let (x3, y3) = obstacle;

    let (vx1, vy1) = (x1 as i64 - x3 as i64, y1 as i64 - y3 as i64);
    let (vx2, vy2) = (x2 as i64 - x3 as i64, y2 as i64 - y3 as i64);

    let parallel = vx1 * vy2 - vx2 * vy1 == 0;
    let obstacle_inside = vx1 * vx2 + vy1 * vy2 < 0;
    parallel && obstacle_inside
}

/// Returns true if `tgt` is visible from `src`.
fn visible(src: (usize, usize), tgt: (usize, usize), asteroids: &HashSet<(usize, usize)>) -> bool {
    if src == tgt {
        return false;
    }
    for &obstacle in asteroids {
        if obstacle != src && obstacle != tgt && perfectly_blocked(src, tgt, obstacle) {
            return false;
        }
    }
    return true;
}

fn distance_squared(p1: (usize, usize), p2: (usize, usize)) -> OrderedFloat<f64> {
    let (x1, y1) = (p1.0 as f64, p1.1 as f64);
    let (x2, y2) = (p2.0 as f64, p2.1 as f64);
    OrderedFloat((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
}

fn main() {
    let input = std::fs::read("data/day10").unwrap();
    let input = String::from_utf8(input).unwrap();
    let board = input
        .split_terminator("\r\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut asteroids = HashSet::new();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '#' {
                asteroids.insert((i, j));
            }
        }
    }

    let mut station = (0, 0);
    let mut best_count = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '#' {
                // count the number of asteroids it can observe
                let src = (i, j);
                let count = asteroids
                    .iter()
                    .filter(|&&tgt| visible(src, tgt, &asteroids))
                    .count();
                if count > best_count {
                    station = src;
                    best_count = count;
                }
            }
        }
    }
    dbg!(best_count);

    // Calculate angles (in terms of atan2) relative to station for all asteroids.
    let mut angles = HashMap::new();
    for &asteroid in &asteroids {
        if asteroid != station {
            let x = asteroid.0 as f64 - station.0 as f64;
            let y = asteroid.1 as f64 - station.1 as f64;
            angles.insert(asteroid, OrderedFloat(f64::atan2(y, x)));
        }
    }

    let mut vaporized = vec![];
    // Start by vaporizing an asteroid stright up.
    for x in (0..station.0).rev() {
        if angles.contains_key(&(x, station.1)) {
            vaporized.push((x, station.1));
            break;
        }
    }
    // Keep track of the current angle (again, in terms of atan2) of the lazer.
    let mut lazer = angles.remove(&vaporized[0]).unwrap();

    // Keep looking for the next target until all asteroids are vaporized.
    while !angles.is_empty() {
        // Among asteroids that have smaller angle than the lazer, look for the
        // one with the largest angle (and the smallest distance to station).
        // If lazer is greater than all the asteroids, we pick the asteroid with
        // the smallest angle to wrap from -pi to pi.
        let (&target, &target_angle) = angles
            .iter()
            .filter(|(_, &angle)| angle < lazer)
            .min_by_key(|(&asteroid, &angle)| (-angle, distance_squared(asteroid, station)))
            .unwrap_or(
                angles
                    .iter()
                    .min_by_key(|(&asteroid, &angle)| (angle, distance_squared(asteroid, station)))
                    .unwrap(),
            );
        angles.remove(&target);
        vaporized.push(target);
        lazer = target_angle;
    }

    let (y, x) = vaporized[199];
    dbg!(x * 100 + y);
}
