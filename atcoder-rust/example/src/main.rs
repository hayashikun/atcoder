mod mod_ex;

fn main() {
    println!("mod_ex");
    let (b, e, m) = (1000000007, 1000000007, 998244353);
    let c = mod_ex::mod_ex(b, e, m);
    println!("\t{} ^ {} % {} = {}", b, e, m, c);
}
