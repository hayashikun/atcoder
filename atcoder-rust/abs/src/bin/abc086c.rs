use proconio::input;

fn main() {
    input!(n: usize);
    let mut ts = vec![0; n + 1];
    let mut xs = vec![0; n + 1];
    let mut ys = vec![0; n + 1];
    for i in 0..n {
        input! {
            t: isize,
            x: isize,
            y: isize,
        }
        ts[i + 1] = t;
        xs[i + 1] = x;
        ys[i + 1] = y;
    }
    let dxy: Vec<isize> = (0..n)
        .map(|i| (xs[i + 1] - xs[i]).abs() + (ys[i + 1] - ys[i]).abs())
        .collect();
    let dt: Vec<isize> = (0..n).map(|i| (ts[i + 1] - ts[i])).collect();

    for i in 0..n {
        if (dxy[i] + dt[i]) % 2 != 0 || dt[i] < dxy[i] {
            print!("No");
            return;
        }
    }
    print!("Yes");
}
