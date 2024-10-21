use aoc2019::IntcodeComputer;

fn main() {
    let mut pc1 = IntcodeComputer::from_file("data/day5");
    pc1.input.push_back(1);
    pc1.run();
    dbg!(&pc1.output);

    let mut pc2 = IntcodeComputer::from_file("data/day5");
    pc2.input.push_back(5);
    pc2.run();
    dbg!(&pc2.output);
}
