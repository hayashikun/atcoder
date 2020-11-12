use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        _n: usize,
        cs: String
    }

    let mut cs: Vec<usize> = cs.chars().map(|c| if c == 'R' { 1 } else { 0 }).collect();

    if cs.iter().all(|&c| c == cs[0]) {
        print!("0");
        return;
    }

    println!("{}", total);
}
