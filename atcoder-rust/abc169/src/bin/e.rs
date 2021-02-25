use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n]
    }

    let mut a: Vec<usize> = ab.iter().map(|v| v[0]).collect();
    let mut b: Vec<usize> = ab.iter().map(|v| v[1]).collect();
    a.sort();
    b.sort();

    if n % 2 == 0 {
        let m1x2 = a[n / 2 - 1] + a[n / 2];
        let m2x2 = b[n / 2 - 1] + b[n / 2];
        println!("{}", m2x2 - m1x2 + 1);
    } else {
        let m1 = a[n / 2];
        let m2 = b[n / 2];
        println!("{}", m2 - m1 + 1);
    }
}
