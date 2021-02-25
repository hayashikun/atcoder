mod dijkstra;
mod mod_ex;
mod prime;

fn main() {
    println!("mod_ex");
    let (b, e, m) = (1000000007, 1000000007, 998244353);
    let c = mod_ex::mod_ex(b, e, m);
    println!("\t{} ^ {} % {} = {}", b, e, m, c);

    println!("dijkstra");
    let (cost, routes) = dijkstra::dijkstra(
        8,
        0,
        7,
        vec![
            (0, 1, 2),
            (0, 2, 7),
            (0, 3, 2),
            (1, 4, 2),
            (1, 5, 4),
            (2, 5, 2),
            (2, 6, 3),
            (3, 6, 5),
            (4, 5, 1),
            (5, 7, 6),
            (6, 7, 2),
        ],
    );
    println!(
        "\tCost: {}, Routes: {}",
        cost,
        routes
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("->")
    );

    println!("prime");
    let n = 884;
    let fs = prime::prime_factors(884);
    println!("\tfactors of {}: {:?}", n, fs);
}
