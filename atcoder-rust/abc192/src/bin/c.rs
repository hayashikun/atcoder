use proconio::input;

fn f(ns: String) -> usize {
    let mut chars: Vec<_> = ns.chars().collect();
    chars.sort_unstable();
    let min_n: usize = chars
        .clone()
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap();
    let max_n: usize = chars.into_iter().rev().collect::<String>().parse().unwrap();
    return max_n - min_n;
}

fn main() {
    input! {
        mut ns: String,
        n: usize
    };

    for _ in 0..n {
        ns = f(ns).to_string();
    }
    println!("{}", ns);
}
