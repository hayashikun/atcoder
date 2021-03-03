use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut nodes = vec![Vec::new(); n];

    for (f, t) in xy {
        nodes[f - 1].push(t - 1);
    }

    let mut dp = vec![-1; n];
    for i in 0..n {
        if nodes[i].len() == 0 {
            dp[i] = 0;
        }
    }

    let mut l = 0;
    for i in 0..n {
        let nl = rec(i, &mut dp, &nodes);
        if l < nl {
            l = nl;
        }
    }

    println!("{}", l);
}

fn rec(n: usize, dp: &mut Vec<i32>, nodes: &Vec<Vec<usize>>) -> i32 {
    if dp[n] != -1 {
        return dp[n];
    }

    let mut l = 0;
    for &nn in nodes[n].iter() {
        let nl = rec(nn, dp, nodes) + 1;
        if l < nl {
            l = nl;
        }
    }

    dp[n] = l;
    return l;
}
