// Primitive data types
fn main() {
    // Integers
    // ineger can be: i8, i16, i32, i64, i128: Signed integers
    // ineger can be: u8, u16, u32, u64, u128: Unsigned integers
    // also, Unsigned integers can hould a bit more then Signed ones

    let a: i32 = 20145;
    println!("a can be {} meaning it is integer", a);

    // Floats
    // f32, f64

    let pi: f64 = 44.4;
    println!("value of pi: f64 {}", pi);

    // Boolean
    // true, false

    let is_secure: bool = true;
    println!("is it secure > {}", is_secure);

    // Char
    // Can include a single letter

    let letter: char = 'x';
    println!("my fac letter is: {}", letter)
}
