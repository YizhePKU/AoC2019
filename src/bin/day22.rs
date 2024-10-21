use std::ops::{Add, Mul, Sub};

/// Find gcd(a, b) and two integers `x` and `y` such that `ax + by = gcd(a, b)`.
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// Calculates the modular multiplicative inverse of x mod m.
fn modinverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Reverse,
    Increment(usize),
    Cut(i64),
}

fn parse_input() -> Vec<Operation> {
    let input = std::fs::read("data/day22").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut result = vec![];
    for line in input.split_terminator("\r\n") {
        if line == "deal into new stack" {
            result.push(Operation::Reverse);
        } else if line.starts_with("deal with increment") {
            let (_, value) = line.rsplit_once(' ').unwrap();
            let n = value.parse().unwrap();
            result.push(Operation::Increment(n));
        } else if line.starts_with("cut") {
            let (_, value) = line.split_once(' ').unwrap();
            let n = value.parse().unwrap();
            result.push(Operation::Cut(n));
        } else {
            panic!("Unknown instruction")
        }
    }
    result
}

/// F(a, b) represents a mathematical function f(x) = ax + b (mod m)
///
/// It also represents the transformation of index with operations. For example,
/// with a 10-card deck, `Operation::Reverse` followed by `Operation::Cut(3)` is
/// represented as f(x) = 9x + 6 (mod 10)
///
/// Invariant: both a and b are within range [0, m)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct F<const M: u64>(i64, i64);

impl<const M: u64> F<M> {
    fn unit() -> Self {
        F(1, 0)
    }

    /// Returns the position of the i-th card after the transformation.
    fn apply(self, i: usize) -> usize {
        let F(a, b) = self;
        ((a as u128) * (i as u128) + (b as u128)).rem_euclid(M as u128) as usize
    }

    /// Composes this transformation with `op`. The new transformation will
    /// apply `op` after `self`.
    fn push(self, op: Operation) -> Self {
        match op {
            Operation::Reverse => self * (-1) + (M - 1) as i64,
            Operation::Increment(n) => self * n as i64,
            Operation::Cut(n) => self - n,
        }
    }

    /// Removes `op` by composing this transformation with the inverse of `op`.
    fn pop(self, op: Operation) -> Self {
        match op {
            Operation::Reverse => self * (-1) + (M - 1) as i64,
            Operation::Increment(n) => self * modinverse(n as i64, M as i64).unwrap() as i64,
            Operation::Cut(n) => self + n,
        }
    }

    /// Composes this transformation with another one. The new transformation will
    /// apply `other` after `self`.
    fn compose(self, other: Self) -> Self {
        let F(a, b) = other;
        self * a + b
    }
}

impl<const M: u64> Add<i64> for F<M> {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        let F(a, b) = self;
        let remainder = ((b as i128) + (rhs as i128)).rem_euclid(M as i128);
        F(a, remainder as i64)
    }
}

impl<const M: u64> Sub<i64> for F<M> {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        let F(a, b) = self;
        let remainder = ((b as i128) - (rhs as i128)).rem_euclid(M as i128);
        F(a, remainder as i64)
    }
}

impl<const M: u64> Mul<i64> for F<M> {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        let F(a, b) = self;
        let factor = ((a as i128) * (rhs as i128)).rem_euclid(M as i128);
        let remainder = ((b as i128) * (rhs as i128)).rem_euclid(M as i128);
        F(factor as i64, remainder as i64)
    }
}

fn main() {
    let operations = parse_input();

    let mut forward = F::<10007>::unit();
    for op in operations.iter() {
        forward = forward.push(*op);
    }
    println!("part1 = {}", forward.apply(2019));

    let mut backward = F::<119315717514047>::unit();
    for op in operations.iter().rev() {
        backward = backward.pop(*op);
    }

    // repeat the entire backward process 101741582076661 times
    fn repeat(count: usize, tx: F<119315717514047>) -> F<119315717514047> {
        if count == 0 {
            F::<119315717514047>::unit()
        } else if count % 2 == 0 {
            let half = repeat(count / 2, tx);
            half.compose(half)
        } else {
            let dec = repeat(count - 1, tx);
            dec.compose(tx)
        }
    }
    let backward_repeat = repeat(101741582076661, backward);
    println!("part2 = {}", backward_repeat.apply(2020));
}

#[test]
fn roundtrip() {
    let operations = parse_input();

    let mut transform = F::<10007>::unit();
    for op in operations.iter() {
        transform = transform.push(*op);
    }
    for op in operations.iter().rev() {
        transform = transform.pop(*op);
    }

    assert_eq!(transform, F::<10007>::unit());
}
