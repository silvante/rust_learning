fn main() {
    let user: User = User {
        active: true,
        name: String::from("Mardonbek Khamidov"),
        username: String::from("xamidov"),
        email: String::from("khamidov.ko@gmail.com"),
        age: 17
    };

    println!("user is {}", user.name);
}

struct User {
    active: bool,
    name: String,
    username: String,
    email: String,
    age: u8,
}