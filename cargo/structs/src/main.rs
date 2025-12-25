fn main() {
    // let mut user: User = User {
    //     active: true,
    //     name: String::from("Mardonbek Khamidov"),
    //     username: String::from("xamidov"),
    //     email: String::from("khamidov.ko@gmail.com"),
    //     age: 17
    // };

    // println!("his email was {}", user.email);

    // user.email = String::from("info@xamidov.uz");
    let user = build_user(
        "Mardonbek Khamidov".to_string(),
        "khamidov.ko@gmail.com".to_string(),
    );

    println!("user is active? => {}", user.active);
    println!("user is => {}", user.name);
    println!("his email is => {}", user.email);
    println!("his username is {}", user.username);
    println!("user is => {} years old", user.age);
}

fn build_user(name: String, email: String) -> User {
    let username = name.trim().to_lowercase().replace(" ", "");

    User {
        active: true,
        name: name,
        username: username,
        email: email,
        age: 18,
    }
}

struct User {
    active: bool,
    name: String,
    username: String,
    email: String,
    age: u8,
}
