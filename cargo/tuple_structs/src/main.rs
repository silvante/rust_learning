struct UnitLike; // <- this is so usefull to use as an Object in OOP of Ruby on Rails

fn main() {
    println!("Hello, world!");

    let subject = UnitLike;

    let black: Color = Color(0, 0, 0);
    let audi: Car = Car("Audi".to_string(), 2010, "A12".to_string());
}

struct Color(i32, i32, i32);
struct Car(String, i32, String);