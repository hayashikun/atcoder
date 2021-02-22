use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize
    }
    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
