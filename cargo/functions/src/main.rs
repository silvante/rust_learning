// so for some reason all names of functions and vars should be in snake case (I like it)
fn main() {
    hello();
    tell_height(184);
}

fn hello() {
    println!("Hello, rust 🦀!")
}

fn tell_height(h: u32) {
    println!("my height is {} cm.", h)
}