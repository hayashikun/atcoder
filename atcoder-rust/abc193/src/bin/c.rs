use im_rc::HashSet;
use num::integer::sqrt;
use proconio::{fastout, input};

fn run(n: usize) -> usize {
    let mut nums = HashSet::new();

    for p in 2..=sqrt(n) {
        if nums.contains(&p) {
            continue;
        }
        let mut m = p;
        loop {
            m *= p;
            if m > n {
                break;
            }
            nums.insert(m);
        }
    }

    return n - nums.len();
}

#[fastout]
fn main() {
    input! {
        n: usize
    };

    println!("{}", run(n));
}
