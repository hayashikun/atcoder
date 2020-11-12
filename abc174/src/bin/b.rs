use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        xy: [[isize; 2]; n]
    }
    let d2 = d.pow(2);
    let c = xy
        .iter()
        .filter(|p| p[0].pow(2) + p[1].pow(2) <= d2)
        .count();
    print!("{}", c)
}
