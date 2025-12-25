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

    println!("\n> User1");
    println!("user is active? => {}", user.active);
    println!("user is => {}", user.name);
    println!("his email is => {}", user.email);
    println!("his username is {}", user.username);
    println!("user is => {} years old", user.age);

    let user2 = User {
        email: "info@xamidov.uz".to_string(),
        username: "xamidov".to_string(),
        name: "Mardonbek Xamidov".to_string(),
        ..user // it is real fun here :)
    };
    // so in this example we didn't use any ::String fields of "user" (user1),
    // so name, username, and email was not moved to user2,
    // means we can use user after declining user2,
    // we also can use fiels like active and age after user2 becouse they are both not strings and they didn't move too.

    println!("\n> User2");
    println!("user is active? => {}", user2.active);
    println!("user is => {}", user2.name);
    println!("his email is => {}", user2.email);
    println!("his username is {}", user2.username);
    println!("user is => {} years old", user2.age); // not bad I can understand it

    // using user1 here
    // cuz we didn't move any String fields to user2, super
    
    println!("\nuser1 is avaible tho :) => {}, {}, {}", user.active, user.email, user.username);
}

fn build_user(name: String, email: String) -> User {
    let username = name.trim().to_lowercase().replace(" ", "");

    User {
        active: true,
        name,
        username,
        email,
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
