use proconio::input;

fn main() {
    input!(a: usize);
    input!(b: usize, c: usize);
    input!(s: String);
    println!("{} {}", a + b + c, s);
}
