use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {ss: Chars};
    let mut ans = true;

    for (i, c) in ss.iter().enumerate() {
        if !((i % 2 == 0 && c.is_lowercase()) || (i % 2 == 1 && c.is_uppercase())) {
            ans = false;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
