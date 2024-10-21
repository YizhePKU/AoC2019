use itertools::Itertools;
use ndarray::{s, Array1, Array2};

/// Returns the repeating pattern for the n-th element.
fn pattern(n: usize) -> impl Iterator<Item = i64> {
    assert!(n > 0);

    let zeros = std::iter::repeat(0).take(n);
    let ones = std::iter::repeat(1).take(n);
    let more_zeros = std::iter::repeat(0).take(n);
    let neg_ones = std::iter::repeat(-1).take(n);

    zeros
        .chain(ones)
        .chain(more_zeros)
        .chain(neg_ones)
        .cycle()
        .skip(1)
}

/// Returns the pattern as a square matrix with shape of (size, size).
fn pattern_matrix(size: usize) -> Array2<i64> {
    let mut mat = Array2::zeros((size, size));
    for i in 0..size {
        let row = pattern(i + 1).take(size).collect_vec();
        for j in 0..row.len() {
            mat[[i, j]] = row[j];
        }
    }
    mat
}

fn main() {
    let input = std::fs::read("data/day16").unwrap();
    let input = String::from_utf8(input).unwrap();
    let signal: Vec<i64> = input
        .trim()
        .chars()
        .map(|b| b.to_digit(10).unwrap() as i64)
        .collect();

    let mat = pattern_matrix(signal.len());
    let mut cur = Array1::from_vec(signal.clone());
    for _ in 0..100 {
        cur = mat.dot(&cur);
        cur = cur.mapv(|x| x.abs() % 10);
    }
    dbg!(&cur.slice(s![..8]));

    // calculate the message offset
    let mut offset = 0;
    for i in 0..7 {
        offset *= 10;
        offset += signal[i] as usize;
    }

    // check that we can "cheat"
    assert!(offset * 2 > signal.len() * 10000);

    // start cheating
    let mut arr = signal
        .iter()
        .cycle()
        .take(signal.len() * 10000)
        .skip(offset)
        .copied()
        .collect_vec();

    for _phase in 0..100 {
        for i in (0..arr.len() - 1).rev() {
            arr[i] += arr[i + 1];
            arr[i] = arr[i];
        }
        for i in 0..arr.len() {
            arr[i] = arr[i].abs() % 10;
        }
    }
    dbg!(&arr[..8]);
}
