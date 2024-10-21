use aoc2019::IntcodeComputer;

fn main() {
    let mut pc = IntcodeComputer::from_file("data/day9");
    pc.input.push_back(1);
    pc.run();
    dbg!(pc.output);

    let mut pc = IntcodeComputer::from_file("data/day9");
    pc.input.push_back(2);
    pc.run();
    dbg!(pc.output);
}
