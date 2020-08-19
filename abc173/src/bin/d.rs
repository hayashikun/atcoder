use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }
    aa.sort();
    aa.reverse();

    let mut total: usize = 0;
    total += aa[0];
    for i in 1..(aa.len() / 2) {
        total += aa[i] * 2
    }
    if aa.len() % 2 == 1 {
        total += aa[aa.len() / 2];
    }
    print!("{}", total);
}
