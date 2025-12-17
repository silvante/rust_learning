// so for some reason all names of functions and vars should be in snake case (I like it)
fn main() {
    let result: i32 = add(48, 82);

    println!("result is: {}", result)
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
