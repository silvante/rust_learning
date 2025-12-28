fn main() {
    let four: IPsKind = IPsKind::V4;
    let six: IPsKind = IPsKind::V6;

    route(four);
    route(six);

    let local = IpAddr {
        kind: IPsKind::V4,
        address: String::from("127.0.0.1")
    };

    println!("Hello, World!")
}

fn route (ip: IPsKind) {}

enum IPsKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IPsKind,
    address: String,
}