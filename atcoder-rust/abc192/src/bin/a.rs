use proconio::input;

fn main() {
    input! {n: usize};
    if n % 100 == 0 {
        println!("100")
    } else {
        println!("{}", (n / 100 + 1) * 100 - n)
    }
}
