use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize
    }
    print!("{}", f_abc(a, b, c, x));
}

fn f_abc(a: usize, b: usize, c: usize, x: usize) -> usize {
    (0..(min(a, x / 500) + 1))
        .map(|n| f_bc(b, c, x - n * 500))
        .sum()
}

fn f_bc(b: usize, c: usize, x: usize) -> usize {
    (0..(min(b, x / 100) + 1))
        .map(|n| f_c(c, x - n * 100))
        .sum()
}

fn f_c(c: usize, x: usize) -> usize {
    if x % 50 == 0 && c >= x / 50 {
        1
    } else {
        0
    }
}
