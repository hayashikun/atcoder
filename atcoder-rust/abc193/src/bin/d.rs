use proconio::{fastout, input};

fn run(k: usize, s: String, t: String) -> f64 {
    let s: Vec<usize> = s[0..4]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let t: Vec<usize> = t[0..4]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut rem = vec![k; 9];
    for n in s.clone() {
        rem[n - 1] -= 1;
    }
    for n in t.clone() {
        rem[n - 1] -= 1;
    }

    let mut st = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for n in s {
        st[n - 1] *= 10;
    }

    let mut tt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for n in t {
        tt[n - 1] *= 10;
    }

    let mut sw = 0;
    for i in 0..9 {
        let mut stc = st.clone();
        stc[i] *= 10;
        let sp: usize = stc.iter().sum();

        let sc = rem[i];
        if sc == 0 {
            continue
        }

        for j in 0..9 {
            let mut tc = rem[j];
            if i == j {
                tc -= 1;
            }
            if tc == 0 {
                continue
            }

            let mut ttc = tt.clone();
            ttc[j] *= 10;
            let tp: usize = ttc.iter().sum();

            if sp > tp {
                sw += sc * tc
            }
        }
    }

    let nc = k * 9 - 8;
    return sw as f64 / (nc * (nc - 1)) as f64;
}

#[fastout]
fn main() {
    input! {
        k: usize,
        s: String,
        t: String
    };

    println!("{}", run(k, s, t));
}
