use proconio::input;

fn run(s: Vec<char>) -> u128 {
    let mut block: Vec<(usize, usize, char)> = Vec::new();

    let size = s.len();
    let mut bls = size;
    for i in 0..(size - 1) {
        if s[i] == s[i + 1] {
            if bls == size {
                bls = i;
            }
        } else {
            if bls != size {
                let mut add = true;
                if let Some((_, _, b)) = block.last() {
                    add = *b != s[bls]
                }
                if add {
                    block.push((bls, i, s[bls]));
                }
                bls = size;
            }
        }
    }

    // println!("{} / {:?}", size, block.clone());

    let mut count: usize = 0;
    let mut bt = size;
    for (bs, be, bc) in block.into_iter().rev() {
        count += size - be - 1;
        for i in (be + 1)..bt {
            if s[i] == bc {
                count -= 1;
            }
        }
        bt = bs;
    }

    return count as u128;
}

fn main() {
    input! {
        s: String,
    }

    println!("{}", run(s.chars().collect()));
}
