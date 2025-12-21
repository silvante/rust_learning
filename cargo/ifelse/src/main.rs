fn main() {
    let age: u8 = 18;

    // if age == 18 {
    //     println!("now you should go to university");
    // } else {
    //     println!("Time has gone");
    // }

    // multiple conditions

    if age == 18 {
        println!("now you should go to university");
    } else if age > 18 {
        println!("Too late go to work");
    } else {
        println!("Too early get ready for now");
    }

    // using conditions directly in let

    let condition: bool = true;

    let number: i32 = if condition { 82 } else {37};
    println!("Number: {number}");
}
