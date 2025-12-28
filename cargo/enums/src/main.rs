fn main() {
    let four: IPsKind = IPsKind::V4;
    let six: IPsKind = IPsKind::V6;

    route(four);
    route(six);

    println!("Hello, World!")
}

fn route (ip: IPsKind) {}

enum IPsKind {
    V4,
    V6,
}