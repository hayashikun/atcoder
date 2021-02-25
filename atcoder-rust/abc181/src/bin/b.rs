use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n]
    }

    let mut sum = 0;
    for r in ab {
        sum += (r[1] - r[0] + 1) * (r[0] + r[1]) / 2
    }

    println!("{}", sum);
}
