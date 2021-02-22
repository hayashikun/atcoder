use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i32; n]
    }
    let mut f = 1;
    'outer: loop {
        for &a in &aa {
            if a % 2_i32.pow(f) != 0 {
                break 'outer;
            }
        }
        f += 1;
    }
    println!("{}", f - 1);
}
