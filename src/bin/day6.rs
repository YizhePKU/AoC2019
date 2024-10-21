use std::collections::HashMap;

fn total_orbits(object: &str, orbits: &HashMap<String, String>) -> usize {
    if object == "COM" {
        0
    } else {
        1 + total_orbits(&orbits[object], orbits)
    }
}

fn path2com(object: &str, orbits: &HashMap<String, String>) -> Vec<String> {
    let mut result = vec![];
    let mut cur = object;
    while cur != "COM" {
        result.push(cur.to_string());
        cur = &orbits[cur];
    }
    result
}

fn main() {
    let input = std::fs::read("data/day6").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut orbits = HashMap::new();
    for line in input.split_terminator("\r\n") {
        let (star, planet) = line.split_once(')').unwrap();
        orbits.insert(planet.to_string(), star.to_string());
    }

    let mut part1 = 0;
    for object in orbits.keys() {
        part1 += total_orbits(object, &orbits);
    }
    dbg!(part1);

    let mut my_path = path2com("YOU", &orbits);
    let mut santa_path = path2com("SAN", &orbits);
    while my_path.last().unwrap() == santa_path.last().unwrap() {
        my_path.pop();
        santa_path.pop();
    }
    dbg!(my_path.len() + santa_path.len() - 2);
}
