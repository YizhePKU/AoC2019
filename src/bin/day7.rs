use aoc2019::IntcodeComputer;
use itertools::Itertools;

fn amplify(phase: i64, input: i64) -> i64 {
    let mut pc = IntcodeComputer::from_file("data/day7");
    pc.input.push_back(phase);
    pc.input.push_back(input);
    pc.run();
    pc.output.pop_front().unwrap()
}

fn main() {
    let mut part1 = 0;
    for phases in (0..5).permutations(5) {
        let mut signal = 0;
        for phase in phases {
            signal = amplify(phase, signal);
        }
        part1 = part1.max(signal);
    }
    dbg!(part1);

    let mut part2 = 0;
    for phases in (5..10).permutations(5) {
        let mut pcs = phases
            .into_iter()
            .map(|phase| {
                let mut pc = IntcodeComputer::from_file("data/day7");
                pc.input.push_back(phase);
                pc
            })
            .collect_vec();

        let mut signal = 0;
        while !pcs[0].halted {
            for pc in &mut pcs {
                pc.input.push_back(signal);
                pc.run();
                signal = pc.output.pop_back().unwrap();
            }
        }
        part2 = part2.max(signal);
    }
    dbg!(part2);
}
