#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        sd: Chars,
        t: Chars
    };

    if sd.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }

    let mut ss = Vec::new();

    for i in 0..=(sd.len() - t.len()) {
        let mut ok = true;
        for j in 0..t.len() {
            if sd[i + j] != '?' && sd[i + j] != t[j] {
                ok = false;
                break;
            }
        }

        if ok {
            let mut cs = Vec::new();
            for j in 0..i {
                cs.push(if sd[j] == '?' { 'a' } else { sd[j] });
            }
            cs.extend(t.clone());
            for j in (i + t.len())..(sd.len()) {
                cs.push(if sd[j] == '?' { 'a' } else { sd[j] });
            }
            ss.push(cs.iter().join(""));
        }
    }

    if ss.is_empty() {
        println!("UNRESTORABLE");
    } else {
        ss.sort();
        println!("{}", ss.first().unwrap());
    }
}
