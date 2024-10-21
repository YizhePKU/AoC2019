#[derive(Debug, Clone)]
struct IntcodeComputer {
    mem: Vec<usize>,
    rip: usize,
    halted: bool,
}

impl IntcodeComputer {
    fn step(&mut self) {
        let rip = self.rip;
        let opcode = self.mem[rip];

        if opcode == 99 {
            self.halted = true;
        } else if opcode == 1 {
            let src1 = self.mem[rip + 1];
            let src2 = self.mem[rip + 2];
            let dst = self.mem[rip + 3];
            self.mem[dst] = self.mem[src1] + self.mem[src2];
            self.rip += 4;
        } else if opcode == 2 {
            let src1 = self.mem[rip + 1];
            let src2 = self.mem[rip + 2];
            let dst = self.mem[rip + 3];
            self.mem[dst] = self.mem[src1] * self.mem[src2];
            self.rip += 4;
        } else {
            panic!("Unknown opcode {opcode} at rip = {rip}");
        }
    }

    fn run(&mut self) -> usize {
        while !self.halted {
            self.step();
        }
        self.mem[0]
    }
}

fn main() {
    let input = std::fs::read("data/day2").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mem: Vec<usize> = input
        .trim()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pc1 = IntcodeComputer {
        mem: mem.clone(),
        rip: 0,
        halted: false,
    };
    pc1.mem[1] = 12;
    pc1.mem[2] = 2;
    pc1.run();
    dbg!(pc1.mem[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut pc = IntcodeComputer {
                mem: mem.clone(),
                rip: 0,
                halted: false,
            };
            pc.mem[1] = noun;
            pc.mem[2] = verb;
            if pc.run() == 19690720 {
                dbg!(noun * 100 + verb);
            }
        }
    }
}
