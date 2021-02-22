use proconio::input;

fn main() {
    input!(s: String);
    let n = s.chars().filter(|&c| c == '1').count();
    println!("{}", n);
}
