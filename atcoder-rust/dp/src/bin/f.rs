use itertools::Itertools;
use nalgebra::max;
use proconio::{fastout, input};

fn run(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut dp = vec![vec![0; 3001]; 3001];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }

    let mut i = s.len();
    let mut j = t.len();
    let mut ss = Vec::new();
    loop {
        if s[i - 1] == t[j - 1] {
            ss.push(s[i - 1]);
            i -= 1;
            j -= 1;
        } else {
            if dp[i][j] == dp[i - 1][j] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        if i == 0 || j == 0 {
            break;
        }
    }

    return ss.iter().rev().join("");
}

#[fastout]
fn main() {
    input! {
        s: String,
        t: String
    }

    // println!("{}", run(s, t));
    print!("");
}
