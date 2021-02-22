use proconio::input;

fn run(_n: usize, k: usize, aa: Vec<usize>) -> usize {
    // length
    let mut ng = 0;
    let mut ok = *aa.iter().max().unwrap();

    while ng < ok - 1 {
        let md = (ng + ok) / 2;
        if aa.iter().map(|&a| (a - 1) / md).sum::<usize>() <= k {
            ok = md;
        } else {
            ng = md;
        }
    }
    return ok;
}

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    }

    println!("{}", run(n, k, aa));
}
