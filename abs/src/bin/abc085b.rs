use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }
    let set: HashSet<usize> = d.into_iter().collect();
    print!("{}", set.len());
}
