use num::abs;
use proconio::{fastout, input};

fn run(n: usize, _m: usize, mut hh: Vec<isize>, mut ww: Vec<isize>) -> isize {
    hh.sort();
    ww.sort();

    let mut hd = Vec::new();
    for i in 0..(n - 1) {
        hd.push(hh[i + 1] - hh[i]);
    }

    let mut dsl = Vec::new();
    dsl.push(0);
    for i in 0..(n / 2) {
        dsl.push(dsl.last().unwrap() + hd[i * 2]);
    }

    let mut dsr = Vec::new();
    dsr.push(0);
    for i in 0..(n / 2) {
        dsr.push(dsr.last().unwrap() + hd[n - i * 2 - 2]);
    }

    let mut min_sum = isize::max_value();
    let mut i = 0;
    for w in ww {
        while i < n && hh[i] < w {
            i += 1;
        }
        if i % 2 != 0 {
            i -= 1;
        }

        let sum = dsl[i / 2] + abs(w - hh[i]) + dsr[(n - i) / 2];
        if sum < min_sum {
            min_sum = sum;
        }
    }

    return min_sum;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        hh: [isize; n],
        ww: [isize; m],
    }

    println!("{}", run(n, m, hh, ww));
}
