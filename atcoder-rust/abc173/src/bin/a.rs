use proconio::input;

fn main() {
    input!(n: usize);
    if n % 1000 == 0 {
        println!("0")
    } else {
        println!("{}", ((n / 1000) + 1) * 1000 - n)
    }
}
