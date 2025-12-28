fn main() {
    let four: IPsKind = IPsKind::V4(127, 0, 0, 1);
    let six: IPsKind = IPsKind::V6("::1".to_string());

    println!("Hello, World!")
}

enum IPsKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
