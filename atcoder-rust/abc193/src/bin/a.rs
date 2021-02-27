use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize
    };

    let a = a as f64;
    let b = b as f64;
    println!("{}", (1.0 - b / a) * 100.0);
}
