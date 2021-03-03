use std::fmt;

mod dijkstra;
mod mod_ex;
mod prime;
mod topological_sort;

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
    let fs = prime::prime_factors(n);
    println!("\tfactors of {}: {:?}", n, fs);

    println!("topological_sort");
    let edges = vec![(0, 1), (0, 2), (2, 1), (1, 3), (2, 3)];
    let (_, s) = topological_sort::topological_sort(4, edges.clone());
    println!("\ttopological_sort of {:?}: {:?}", edges, s);

    println!("Size");
    fn p<T: fmt::Display>(n: &str, m: T) {
        println!("\t{} max: {} ({} digit)", n, m, m.to_string().len())
    }
    p("i8", i8::max_value());
    p("i16", i16::max_value());
    p("i32", i32::max_value());
    p("i64", i64::max_value());
    p("i128", i128::max_value());
    p("isize", isize::max_value());
    p("u8", u8::max_value());
    p("u16", u16::max_value());
    p("u32", u32::max_value());
    p("u64", u64::max_value());
    p("u128", u128::max_value());
    p("usize", usize::max_value());
}
