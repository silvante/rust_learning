fn main() {
    let four: IPsKind = IPsKind::V4(127, 0, 0, 1);
    let six: IPsKind = IPsKind::V6("::1".to_string());

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("Hello, World!")
}

enum IPsKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("Just How?")
    }
}