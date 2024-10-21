use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

type Point = (usize, usize);

fn parse_input() -> (HashSet<Point>, HashMap<String, HashSet<Point>>) {
    let input = std::fs::read("data/day20").unwrap();
    let input = String::from_utf8(input).unwrap();
    let board = input
        .split_terminator("\r\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let n = board.len();
    let m = board[0].len();

    let mut tiles = HashSet::new();
    let mut portals: HashMap<String, HashSet<Point>> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '.' {
                tiles.insert((i, j));
            }

            // horizontal label?
            if j < m - 1
                && board[i][j].is_ascii_alphabetic()
                && board[i][j + 1].is_ascii_alphabetic()
            {
                let label = String::from_iter([board[i][j], board[i][j + 1]]);
                let point = if j > 0 && board[i][j - 1] == '.' {
                    (i, j - 1)
                } else {
                    (i, j + 2)
                };
                portals.entry(label).or_default().insert(point);
            }

            // vertical label?
            if i < n - 1
                && board[i][j].is_ascii_alphabetic()
                && board[i + 1][j].is_ascii_alphabetic()
            {
                let label = String::from_iter([board[i][j], board[i + 1][j]]);
                let point = if i > 0 && board[i - 1][j] == '.' {
                    (i - 1, j)
                } else {
                    (i + 2, j)
                };
                portals.entry(label).or_default().insert(point);
            }
        }
    }

    (tiles, portals)
}

fn part1(tiles: &HashSet<Point>, portals: &HashMap<String, HashSet<Point>>) {
    // create a point->point mapping between portal endpoints
    let mut point2point = HashMap::new();
    for (label, points) in portals {
        if label != "AA" && label != "ZZ" {
            let (&p1, &p2) = points.iter().collect_tuple().unwrap();
            point2point.insert(p1, p2);
            point2point.insert(p2, p1);
        }
    }

    // find the entry and the exit
    let entry = *portals["AA"].iter().next().unwrap();
    let exit = *portals["ZZ"].iter().next().unwrap();

    // solve the maze with BFS
    let mut vis = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((entry, 0));
    while let Some((point, distance)) = todo.pop_front() {
        if point == exit {
            println!("part1 = {distance}");
            break;
        }
        if tiles.contains(&point) && !vis.contains(&point) {
            vis.insert(point);

            let (i, j) = point;
            if i > 0 {
                todo.push_back(((i - 1, j), distance + 1));
            }
            todo.push_back(((i + 1, j), distance + 1));
            if j > 0 {
                todo.push_back(((i, j - 1), distance + 1));
            }
            todo.push_back(((i, j + 1), distance + 1));

            if let Some(&target) = point2point.get(&point) {
                todo.push_back((target, distance + 1));
            }
        }
    }
}

/// Returns true if `point` is on the outer edge of the maze.
fn is_outer(point: Point, tiles: &HashSet<Point>) -> bool {
    let x_min = *tiles.iter().map(|(x, _)| x).min().unwrap();
    let x_max = *tiles.iter().map(|(x, _)| x).max().unwrap();
    let y_min = *tiles.iter().map(|(_, y)| y).min().unwrap();
    let y_max = *tiles.iter().map(|(_, y)| y).max().unwrap();

    let (x, y) = point;
    x == x_min || x == x_max || y == y_min || y == y_max
}

fn reverse_map<K, V: Eq + Hash>(m: HashMap<K, V>) -> HashMap<V, K> {
    m.into_iter().map(|(k, v)| (v, k)).collect()
}

type Label = (String, i8); // +1 means it recurses, -1 means it returns

fn bfs(
    tiles: &HashSet<Point>,
    start: Point,
    ends: &HashMap<Label, Point>,
) -> HashMap<Label, usize> {
    let ends = reverse_map(ends.clone());
    let mut result = HashMap::new();

    let mut vis = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((start, 0));
    while let Some((point, distance)) = todo.pop_front() {
        if tiles.contains(&point) && !vis.contains(&point) {
            vis.insert(point);

            if let Some(label) = ends.get(&point) {
                if distance != 0 {
                    result.insert(label.clone(), distance);
                }
            }

            let (i, j) = point;
            if i > 0 {
                todo.push_back(((i - 1, j), distance + 1));
            }
            todo.push_back(((i + 1, j), distance + 1));
            if j > 0 {
                todo.push_back(((i, j - 1), distance + 1));
            }
            todo.push_back(((i, j + 1), distance + 1));
        }
    }
    result
}

fn dijkstra(graph: &HashMap<Label, HashMap<Label, usize>>) -> usize {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct State {
        distance: Reverse<usize>,
        position: Label,
        depth: i8,
    }

    let mut todo = BinaryHeap::new();
    let mut vis = HashSet::new();
    todo.push(State {
        distance: Reverse(0),
        position: ("AA".to_string(), -1),
        depth: 0,
    });

    while let Some(state) = todo.pop() {
        let State {
            distance,
            position,
            depth,
        } = state;

        // skip previously visited state
        if vis.contains(&(position.clone(), depth)) {
            continue;
        }
        vis.insert((position.clone(), depth));

        // Check if we've reached the goal.
        // Since we started on depth 0, entering ZZ would result in depth -1.
        if position.0 == "ZZ" && depth == -1 {
            // We don't need to teleport out of ZZ, so that's one less step.
            return distance.0 - 1;
        }

        // skip illegal states (entering AA, entering ZZ while not in depth 0,
        // or being in negative depth)
        if !graph.contains_key(&position) || depth < 0 {
            continue;
        }

        for (next_hop, &cost) in &graph[&position] {
            // Calculate the new total distance.
            // Remember, teleporting also cost 1 step!
            let distance = Reverse(distance.0 + cost + 1);

            // Go to next_hop, then teleport.
            let mut position = next_hop.clone();
            position.1 = -position.1;

            // Adjust depth.
            let depth = depth + next_hop.1;

            todo.push(State {
                distance,
                position,
                depth,
            });
        }
    }

    panic!("couldn't find a valid path from AA to ZZ")
}

fn part2(tiles: &HashSet<Point>, portals: &HashMap<String, HashSet<Point>>) {
    let mut endpoints: HashMap<Label, Point> = HashMap::new();
    for (name, points) in portals {
        for &point in points {
            if is_outer(point, tiles) {
                endpoints.insert((name.to_string(), -1), point);
            } else {
                endpoints.insert((name.to_string(), 1), point);
            }
        }
    }

    // run BFS from every portal endpoint
    let mut paths = HashMap::new();
    for (label, &point) in &endpoints {
        paths.insert(label.clone(), bfs(tiles, point, &endpoints));
    }

    // Now run Dijkstra on the newly generated (infinite) graph.
    let answer = dijkstra(&paths);
    println!("part2 = {answer}");
}

fn main() {
    let (tiles, portals) = parse_input();

    part2(&tiles, &portals);
}
