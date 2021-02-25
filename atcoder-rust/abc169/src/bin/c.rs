use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u128,
        b: String
    }

    println!("{}", run(a, b))
}

fn run(a: u128, b: String) -> u128 {
    let bs: Vec<&str> = b.split(".").collect();
    let b1: u128 = bs[0].parse().unwrap();
    let b2: u128 = bs[1].parse().unwrap();

    return a * b1 + a * b2 / 100;
}
