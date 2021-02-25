use proconio::{fastout, input};

fn run(n: usize, xy: Vec<Vec<isize>>) -> bool {
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let a = xy[k][0] - xy[i][0];
                let b = xy[j][0] - xy[i][0];
                if a == 0 || b == 0 {
                    if a == 0 && b == 0 {
                        return true;
                    }
                    continue;
                }
                if (xy[k][1] - xy[i][1]) * b == (xy[j][1] - xy[i][1]) * a {
                    return true;
                }
            }
        }
    }
    return false;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [[isize; 2]; n]
    }

    if run(n, xy) {
        println!("Yes")
    } else {
        println!("No")
    }
}
