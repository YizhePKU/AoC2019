use aoc2019::IntcodeComputer;

fn main() {
    let mut computer = IntcodeComputer::from_file("data/day21");

    let instruction = std::fs::read("data/day21_instruction").unwrap();
    for byte in instruction {
        computer.input.push_back(byte as i64);
    }

    computer.run();

    // for byte in computer.output {
    //     print!("{}", char::from_u32(byte as u32).unwrap())
    // }
    dbg!(computer.output);
}
