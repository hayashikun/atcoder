use proconio::input;

fn main() {
    input! {
        n: usize,
        mut uv: [[usize; 2]; n-1]
    }
    uv.iter_mut().for_each(|e| e.sort());

    let mut total: usize = (1..(n + 1)).map(|r| r * (r + 1) / 2).sum();
    for v in uv.iter() {
        total -= v[0] * (n - v[1] + 1)
    }
    println!("{}", total);
}
