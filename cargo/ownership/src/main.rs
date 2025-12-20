// fn main() {
//    let s1 = String::from("RUST");
//    let len = calculate_length(&s1); 
//
//    println!("length of {} is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//    return s.len()
// }

fn main() {
    let owner1 = String::from("SingleValue");
    let owner2 = owner1;

    println!("value is {}", owner2); // printed
}
