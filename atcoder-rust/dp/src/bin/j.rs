use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    }

    let mut n1 = 0;
    let mut n2 = 0;
    let mut n3 = 0;

    for a in aa.iter() {
        match a {
            1 => n1 += 1,
            2 => n2 += 1,
            3 => n3 += 1,
            _ => (),
        }
    }

    let mut dp = vec![vec![vec![-1.0; n3 + 1]; n2 + n3 + 1]; n1 + n2 + n3 + 1];
    dp[0][0][0] = 0.0;

    let ans = rec(n1, n2, n3, &(n as f64), &mut dp);
    println!("{}", ans);
}

fn rec(n1: usize, n2: usize, n3: usize, n: &f64, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if dp[n1][n2][n3] >= 0.0 {
        return dp[n1][n2][n3];
    }
    let fn1 = n1 as f64;
    let fn2 = n2 as f64;
    let fn3 = n3 as f64;
    let p0 = (n - fn1 - fn2 - fn3) / n;
    let v = 1.0 / (1.0 - p0)
        * (1.0
            + if n1 > 0 {
                rec(n1 - 1, n2, n3, n, dp) * fn1 / n
            } else {
                0.0
            }
            + if n2 > 0 {
                rec(n1 + 1, n2 - 1, n3, n, dp) * fn2 / n
            } else {
                0.0
            }
            + if n3 > 0 {
                rec(n1, n2 + 1, n3 - 1, n, dp) * fn3 / n
            } else {
                0.0
            });
    dp[n1][n2][n3] = v;
    return v;
}
