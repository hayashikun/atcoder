use proconio::input;

fn main() {
    input!(x: isize);
    if x >= 30 {
        print!("Yes");
    } else {
        print!("No");
    }
}
