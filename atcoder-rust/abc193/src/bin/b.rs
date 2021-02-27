use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        apx: [[isize; 3]; n]
    };

    let mut price = isize::max_value();
    for i in 0..n {
        if apx[i][2] > apx[i][0] {
            if price > apx[i][1] {
                price = apx[i][1];
            }
        }
    }

    if price == isize::max_value() {
        println!("-1");
    } else {
        println!("{}", price);
    }
}
