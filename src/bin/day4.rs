fn has_strict_double(digits: &[u64]) -> bool {
    if digits[0] == digits[1] && digits[1] != digits[2] {
        return true;
    }
    if digits[1] == digits[2] && digits[0] != digits[1] && digits[2] != digits[3] {
        return true;
    }
    if digits[2] == digits[3] && digits[1] != digits[2] && digits[3] != digits[4] {
        return true;
    }
    if digits[3] == digits[4] && digits[2] != digits[3] && digits[4] != digits[5] {
        return true;
    }
    if digits[4] == digits[5] && digits[3] != digits[4] {
        return true;
    }
    return false;
}

fn main() {
    let input_low = 357253;
    let input_high = 892942;

    let mut part1 = 0;
    for d1 in 0..=9 {
        for d2 in d1..=9 {
            for d3 in d2..=9 {
                for d4 in d3..=9 {
                    for d5 in d4..=9 {
                        for d6 in d5..=9 {
                            let double = d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6;
                            let value =
                                d1 * 100000 + d2 * 10000 + d3 * 1000 + d4 * 100 + d5 * 10 + d6;
                            if double && value >= input_low && value <= input_high {
                                part1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    dbg!(part1);

    let mut part2 = 0;
    for d1 in 0..=9 {
        for d2 in d1..=9 {
            for d3 in d2..=9 {
                for d4 in d3..=9 {
                    for d5 in d4..=9 {
                        for d6 in d5..=9 {
                            let value =
                                d1 * 100000 + d2 * 10000 + d3 * 1000 + d4 * 100 + d5 * 10 + d6;
                            if has_strict_double(&[d1, d2, d3, d4, d5, d6])
                                && value >= input_low
                                && value <= input_high
                            {
                                part2 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    dbg!(part2);
}
