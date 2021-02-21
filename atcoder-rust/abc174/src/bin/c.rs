use proconio::input;

fn main() {
    input!(k: isize);
    if k % 2 == 0 || k % 5 == 0 {
        print!("-1");
        return;
    }
    let mut a = 0;
    let mut m = 7 % k;
    let mut sum_m = 0;
    let am = 10 % k;
    loop {
        m *= am;
        m %= k;
        sum_m += m;
        sum_m %= k;
        if sum_m == 0 {
            print!("{}", a + 1);
            break;
        }
        a += 1;
    }
}
