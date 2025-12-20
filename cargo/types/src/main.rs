// compaund data types
// arrays, tuples, slices, and strings (sliced string)

fn main() {
    // let ints: [i32; 4] = [82, 37, 28, 45];
    // println!("Array of numbers: {:?}", ints);

    // Arrays
    let strs: [&str; 3] = ["react", "vue", "svalte"];
    println!("My fav front end framework is: {}", strs[0]);

    // Tuples
    let human: (String, i32, bool, [&str; 3]) = ("Valentine".to_string(), 22, true, strs);
    println!("human tuple: {:?}", human);

    // Slices
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("slice is: {:?}", slice);

    let animal_slice: &[&str] = &["dog", "cat", "cayote"];
    println!("animals: {:?}", animal_slice);

    let book_slice: &[String] = &["REWORK".to_string(), "Mukammal Dasturlash".to_string(), "Remote".to_string()];

    println!("books: {:?}", book_slice);

    // strings
    let mut xamidov: String = String::from("What the ");
    xamidov.push_str("heck!");
    println!("xamidov says: {}", xamidov);

    // string slices
    let string: String = String::from("Hello, world!");
    let string_slice: &str = &string[0..5];
    println!("Sting slice is: {}", string_slice);
}