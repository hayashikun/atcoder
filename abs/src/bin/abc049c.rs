use proconio::input;

fn main() {
    input!(mut s: String);
    let mut s = s.as_str();
    let ps = vec!["dream", "dreamer", "erase", "eraser"];

    while s != "" {
        let mut ew = false;
        for &p in &ps {
            ew = s.ends_with(p);
            if ew {
                s = &s[0..(s.len() - p.len())];
                break;
            }
        }
        if !ew {
            print!("NO");
            return;
        }
    }
    print!("YES");
}
