use std::fmt;

#[allow(dead_code)]
fn size() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        size();
    }
}
