use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, VecDeque};

type Board = Vec<Vec<char>>;
type Point = (usize, usize);

/// Returns the board, the starting point, and the location of all the keys.
fn parse_input() -> (Board, Point, HashMap<char, Point>) {
    let input = std::fs::read("data/day18").unwrap();
    let input = String::from_utf8(input).unwrap();

    let board = input
        .split_terminator("\r\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut starting_point = None;
    let mut keys = HashMap::new();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '@' {
                starting_point = Some((i, j));
            }
            if board[i][j].is_ascii_lowercase() {
                keys.insert(board[i][j], (i, j));
            }
        }
    }

    (board, starting_point.unwrap(), keys)
}

/// Returns whether the tile is accessible with the given set of keys.
fn accessible(tile: char, keys: &BTreeSet<char>) -> bool {
    // Can't go through walls.
    if tile == '#' {
        return false;
    }
    // Can't go through doors without corresponding keys.
    if tile.is_ascii_uppercase() && !keys.contains(&tile.to_ascii_lowercase()) {
        return false;
    }
    return true;
}

/// Returns the shortest distance to all accessible keys that haven't been
/// collected (without passing through another uncollected key).
fn distance_to_keys(
    board: &Board,
    position: Point,
    collected_keys: &BTreeSet<char>,
) -> HashMap<char, usize> {
    let n = board.len();
    let m = board[0].len();

    let mut result = HashMap::new();
    let mut vis = BTreeSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((position, 0)); // (point, distance)
    while let Some(((x, y), distance)) = todo.pop_front() {
        if !vis.contains(&(x, y)) && accessible(board[x][y], collected_keys) {
            vis.insert((x, y));
            if board[x][y].is_ascii_lowercase() && !collected_keys.contains(&board[x][y]) {
                result.insert(board[x][y], distance);
            } else {
                if x > 0 {
                    todo.push_back(((x - 1, y), distance + 1));
                }
                if x < n - 1 {
                    todo.push_back(((x + 1, y), distance + 1));
                }
                if y > 0 {
                    todo.push_back(((x, y - 1), distance + 1));
                }
                if y < m - 1 {
                    todo.push_back(((x, y + 1), distance + 1));
                }
            }
        }
    }
    result
}

/// Returns the minimum number of steps required to collect all accessible keys
/// not yet collected.
fn min_steps_to_collect_all_keys(
    board: &Board,
    key2point: &HashMap<char, Point>,
    position: Point,
    collected_keys: &BTreeSet<char>,
    cache: &mut HashMap<(Point, BTreeSet<char>), usize>,
) -> usize {
    let distance2keys = distance_to_keys(board, position, collected_keys);
    if distance2keys.is_empty() {
        0
    } else {
        // check cache
        if let Some(result) = cache.get(&(position, collected_keys.clone())) {
            return *result;
        }

        let mut keys = collected_keys.clone();
        let mut best = usize::MAX;
        for (key, distance) in distance2keys {
            keys.insert(key);

            let recur =
                min_steps_to_collect_all_keys(board, key2point, key2point[&key], &keys, cache);
            best = best.min(recur + distance);

            keys.remove(&key);
        }

        // update cache
        cache.insert((position, collected_keys.clone()), best);

        best
    }
}

fn min_steps_to_collect_all_keys_with_robots(
    board: &Board,
    key2point: &HashMap<char, Point>,
    robots: &[Point; 4],
    collected_keys: &BTreeSet<char>,
    cache: &mut HashMap<([Point; 4], BTreeSet<char>), usize>,
) -> usize {
    let distance2keys = robots
        .iter()
        .map(|point| distance_to_keys(board, *point, collected_keys))
        .collect_vec();

    if distance2keys.iter().all(|m| m.is_empty()) {
        0
    } else {
        // check cache
        if let Some(result) = cache.get(&(robots.clone(), collected_keys.clone())) {
            return *result;
        }

        let mut keys = collected_keys.clone();

        let mut best = usize::MAX;
        for (i, d2k) in distance2keys.into_iter().enumerate() {
            for (key, distance) in d2k {
                keys.insert(key);
                let mut robots = robots.clone();
                robots[i] = key2point[&key];

                let recur = min_steps_to_collect_all_keys_with_robots(
                    board, key2point, &robots, &keys, cache,
                );
                best = best.min(recur + distance);

                keys.remove(&key);
            }
        }

        // update cache
        cache.insert((robots.clone(), collected_keys.clone()), best);

        best
    }
}

fn main() {
    let (board, starting_point, key2point) = parse_input();

    // let part1 = min_steps_to_collect_all_keys(
    //     &board,
    //     &key2point,
    //     starting_point,
    //     &BTreeSet::new(),
    //     &mut HashMap::new(),
    // );
    // dbg!(part1);

    // update the board
    let mut board2 = board.clone();
    let (x0, y0) = starting_point;
    board2[x0][y0] = '#';
    board2[x0 - 1][y0] = '#';
    board2[x0 + 1][y0] = '#';
    board2[x0][y0 - 1] = '#';
    board2[x0][y0 + 1] = '#';

    let robots = [
        (x0 - 1, y0 - 1),
        (x0 - 1, y0 + 1),
        (x0 + 1, y0 - 1),
        (x0 + 1, y0 + 1),
    ];
    let part2 = min_steps_to_collect_all_keys_with_robots(
        &board2,
        &key2point,
        &robots,
        &BTreeSet::new(),
        &mut HashMap::new(),
    );
    dbg!(part2);
}
