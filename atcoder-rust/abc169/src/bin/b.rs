use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u128; n]
    }

    if aa.contains(&0) {
        println!("0");
        return;
    }

    let mut n = 1;
    for a in aa {
        n *= a;

        if n > 1_000_000_000_000_000_000 {
            println!("-1");
            return;
        }
    }

    println!("{}", n);
}
