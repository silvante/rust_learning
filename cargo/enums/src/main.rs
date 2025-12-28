fn main() {
    let four: IPsKind = IPsKind::V4("127.0.0.1".to_string());
    let six: IPsKind = IPsKind::V6("::1".to_string());

    println!("Hello, World!")
}

enum IPsKind {
    V4(String),
    V6(String),
}