use proconio::input;

fn main() {
    input!(n: usize);
    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;
    for _ in 0..n {
        input!(s: String);
        match &*s {
            "AC" => ac += 1,
            "WA" => wa += 1,
            "TLE" => tle += 1,
            "RE" => re += 1,
            _ => (),
        }
    }
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
