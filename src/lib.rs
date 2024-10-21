use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    pub mem: Vec<i64>,
    pub ip: usize, // instruction pointer
    pub rb: i64,   // relative base
    pub halted: bool,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    /// Get the parameter mode for the n-th parameter.
    fn new(mut modes: i64, n: usize) -> Self {
        for _ in 0..n - 1 {
            modes /= 10;
        }
        if modes % 10 == 0 {
            Self::Position
        } else if modes % 10 == 1 {
            Self::Immediate
        } else if modes % 10 == 2 {
            Self::Relative
        } else {
            panic!("Unknown parameter mode {modes} with n = {n}");
        }
    }
}

impl IntcodeComputer {
    /// Ensure the memory address `addr` is readable/writable, extending the
    /// memory with zeros when necessary.
    fn ensure_addr(&mut self, addr: i64) {
        if self.mem.len() <= addr as usize {
            self.mem.resize(addr as usize + 1, 0);
        }
    }

    /// Read a parameter according to its parameter mode.
    fn read(&mut self, value: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => {
                let addr = value;
                assert!(addr >= 0, "Position read address should be non-negative");
                self.ensure_addr(addr);
                self.mem[addr as usize]
            }
            ParameterMode::Immediate => value,
            ParameterMode::Relative => {
                let addr = self.rb + value;
                assert!(addr >= 0, "Relative read address should be non-negative");
                self.ensure_addr(addr);
                self.mem[addr as usize]
            }
        }
    }

    /// Write `value` to memory. The address is looked up according to the
    /// parameter mode, which cannot be immediate. `value` is always immediate.
    fn write(&mut self, addr: i64, mode: ParameterMode, value: i64) {
        match mode {
            ParameterMode::Position => {
                assert!(addr >= 0, "Position write address should be non-negative");
                self.ensure_addr(addr);
                self.mem[addr as usize] = value;
            }
            ParameterMode::Immediate => {
                panic!("Write address parameter mode cannot be immediate");
            }
            ParameterMode::Relative => {
                let addr = self.rb + addr;
                assert!(addr >= 0, "Relative write address should be non-negative");
                self.ensure_addr(addr);
                self.mem[addr as usize] = value;
            }
        }
    }

    /// Create a new IntcodeComputer intialized with `memory`.
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            mem: memory,
            ip: 0,
            rb: 0,
            halted: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    /// Create a new IntcodeComputer whose memory is intialized from the contents of `file`.
    pub fn from_file(file: &str) -> Self {
        let bytes = std::fs::read(file).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        let memory: Vec<i64> = text
            .split_terminator(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Self::new(memory)
    }

    /// Execute a single instruction. If the instruction couldn't be executed
    /// because the computer is waiting on input, returns false; otherwise
    /// returns true.
    pub fn step(&mut self) -> bool {
        let ip = self.ip;
        let opcode = self.mem[ip] % 100;
        let modes = self.mem[ip] / 100;

        if opcode == 99 {
            self.halted = true;
        } else if opcode == 1 {
            // ADD lhs,rhs,addr
            let lhs = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            let rhs = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
            self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), lhs + rhs);
            self.ip += 4;
        } else if opcode == 2 {
            // MUL lhs,rhs,addr
            let lhs = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            let rhs = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
            self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), lhs * rhs);
            self.ip += 4;
        } else if opcode == 3 {
            // INPUT addr
            if let Some(value) = self.input.pop_front() {
                self.write(self.mem[ip + 1], ParameterMode::new(modes, 1), value);
                self.ip += 2;
            } else {
                return false;
            }
        } else if opcode == 4 {
            // OUTPUT value
            let value = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            self.output.push_back(value);
            self.ip += 2;
        } else if opcode == 5 {
            // JNZ cond,addr
            let cond = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            if cond != 0 {
                let addr = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
                assert!(addr >= 0, "JNZ address should be non-negative");
                self.ip = addr as usize;
            } else {
                self.ip += 3;
            }
        } else if opcode == 6 {
            // JZ cond,addr
            let cond = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            if cond == 0 {
                let addr = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
                assert!(addr >= 0, "JZ address should be non-negative");
                self.ip = addr as usize;
            } else {
                self.ip += 3;
            }
        } else if opcode == 7 {
            // LT lhs,rhs,addr
            let lhs = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            let rhs = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
            if lhs < rhs {
                self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), 1);
            } else {
                self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), 0);
            }
            self.ip += 4;
        } else if opcode == 8 {
            // EQ lhs,rhs,addr
            let lhs = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            let rhs = self.read(self.mem[ip + 2], ParameterMode::new(modes, 2));
            if lhs == rhs {
                self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), 1);
            } else {
                self.write(self.mem[ip + 3], ParameterMode::new(modes, 3), 0);
            }
            self.ip += 4;
        } else if opcode == 9 {
            // RB delta
            let delta = self.read(self.mem[ip + 1], ParameterMode::new(modes, 1));
            self.rb += delta;
            self.ip += 2;
        } else {
            panic!("Unknown opcode {opcode} at rip = {ip}");
        }

        true
    }

    /// Execute until the computer blocks on input or halts.
    pub fn run(&mut self) {
        while !self.halted && self.step() {}
    }
}

#[cfg(test)]
mod test {
    use super::IntcodeComputer;

    #[test]
    fn instruction_halt() {
        let mut pc = IntcodeComputer::new(vec![99]);
        pc.step();
        assert!(pc.halted);
    }

    #[test]
    fn instruction_add() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![1, 4, 5, 0, 100, -200]);
        pc.step();
        assert_eq!(pc.mem[0], -100);
        assert_eq!(pc.ip, 4);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![1001, 4, -200, 0, 100]);
        pc.step();
        assert_eq!(pc.mem[0], -100);
        assert_eq!(pc.ip, 4);

        // relative mode
        let mut pc = IntcodeComputer::new(vec![109, 10, 22201, 0, 1, -5, 99, 99, 99, 99, 100, 200]);
        pc.step();
        pc.step();
        assert_eq!(pc.mem[5], 300);
        assert_eq!(pc.ip, 6);
    }

    #[test]
    fn instruction_mul() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![2, 4, 5, 0, 30, -30]);
        pc.step();
        assert_eq!(pc.mem[0], -900);
        assert_eq!(pc.ip, 4);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![102, 30, 4, 0, -30]);
        pc.step();
        assert_eq!(pc.mem[0], -900);
        assert_eq!(pc.ip, 4);
    }

    #[test]
    fn instruction_input() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![3, 0]);
        pc.input.push_back(123);
        pc.step();
        assert_eq!(pc.mem[0], 123);
        assert!(pc.input.is_empty());
    }

    #[test]
    fn block_on_input() {
        let mut pc = IntcodeComputer::new(vec![3, 0]);
        assert!(!pc.step());
        assert_eq!(pc.ip, 0);
    }

    #[test]
    fn instruction_output() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![4, 0]);
        pc.step();
        assert_eq!(pc.output, [4]);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![104, 0]);
        pc.step();
        assert_eq!(pc.output, [0]);
    }

    #[test]
    fn instruction_jnz() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![5, 2, 0]);
        pc.step();
        assert_eq!(pc.ip, 3);

        let mut pc = IntcodeComputer::new(vec![5, 2, 1]);
        pc.step();
        assert_eq!(pc.ip, 2);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![1005, 3, 200, 0]);
        pc.step();
        assert_eq!(pc.ip, 3);

        let mut pc = IntcodeComputer::new(vec![1005, 3, 200, 1]);
        pc.step();
        assert_eq!(pc.ip, 200);

        // relative mode
        let mut pc = IntcodeComputer::new(vec![109, 10, 2105, 1, -5, 99]);
        pc.step();
        pc.step();
        assert_eq!(pc.ip, 99);
    }

    #[test]
    fn instruction_jz() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![6, 2, 0]);
        pc.step();
        assert_eq!(pc.ip, 6);

        let mut pc = IntcodeComputer::new(vec![6, 2, 1]);
        pc.step();
        assert_eq!(pc.ip, 3);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![1006, 3, 200, 0]);
        pc.step();
        assert_eq!(pc.ip, 200);

        let mut pc = IntcodeComputer::new(vec![1006, 3, 200, 1]);
        pc.step();
        assert_eq!(pc.ip, 3);
    }

    #[test]
    fn instruction_le() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![7, 4, 5, 0, 100, 200]);
        pc.step();
        assert_eq!(pc.mem[0], 1);

        let mut pc = IntcodeComputer::new(vec![7, 4, 5, 0, 200, 100]);
        pc.step();
        assert_eq!(pc.mem[0], 0);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![1107, 4, 5, 0]);
        pc.step();
        assert_eq!(pc.mem[0], 1);

        let mut pc = IntcodeComputer::new(vec![1007, 4, 5, 0, 100]);
        pc.step();
        assert_eq!(pc.mem[0], 0);

        // relative mode
        let mut pc = IntcodeComputer::new(vec![109, -5, 21107, 100, 200, 5]);
        pc.step();
        pc.step();
        assert_eq!(pc.mem[0], 1);
    }

    #[test]
    fn instruction_eq() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![8, 4, 5, 0, 100, 200]);
        pc.step();
        assert_eq!(pc.mem[0], 0);

        let mut pc = IntcodeComputer::new(vec![8, 4, 5, 0, 200, 200]);
        pc.step();
        assert_eq!(pc.mem[0], 1);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![1108, 4, 4, 0]);
        pc.step();
        assert_eq!(pc.mem[0], 1);

        let mut pc = IntcodeComputer::new(vec![1008, 4, 100, 0, 100]);
        pc.step();
        assert_eq!(pc.mem[0], 1);

        let mut pc = IntcodeComputer::new(vec![1108, 4, 5, 0, 200, 200]);
        pc.step();
        assert_eq!(pc.mem[0], 0);
    }

    #[test]
    fn instruction_rb() {
        // location mode
        let mut pc = IntcodeComputer::new(vec![9, 0, 9, 0]);
        pc.step();
        assert_eq!(pc.rb, 9);
        pc.step();
        assert_eq!(pc.rb, 18);

        // immediate mode
        let mut pc = IntcodeComputer::new(vec![109, 10, 109, -20]);
        pc.step();
        assert_eq!(pc.rb, 10);
        pc.step();
        assert_eq!(pc.rb, -10);

        // relative mode
        let mut pc = IntcodeComputer::new(vec![109, 10, 209, -10]);
        pc.step();
        pc.step();
        assert_eq!(pc.rb, 119);
    }

    #[test]
    fn read_past_boundry() {
        let mut pc = IntcodeComputer::new(vec![4, 10]);
        pc.step();
        assert_eq!(pc.output, &[0]);
    }

    #[test]
    fn write_past_boundry() {
        let mut pc = IntcodeComputer::new(vec![1101, 10, 20, 10, 4, 10]);
        pc.step();
        pc.step();
        assert_eq!(pc.output, &[30]);
    }
}
