fn fuel_required(mass: u64) -> u64 {
    (mass / 3) - 2
}

fn recursive_fuel_required(mass: u64) -> u64 {
    if mass <= 6 {
        0
    } else {
        let fuel = (mass / 3) - 2;
        fuel + recursive_fuel_required(fuel)
    }
}

fn main() {
    let input = std::fs::read("data/day1").unwrap();
    let input = String::from_utf8(input).unwrap();

    let modules: Vec<u64> = input
        .split_terminator("\r\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let part1: u64 = modules.iter().map(|m| fuel_required(*m)).sum();
    dbg!(part1);

    let part2: u64 = modules.iter().map(|m| recursive_fuel_required(*m)).sum();
    dbg!(part2);
}
