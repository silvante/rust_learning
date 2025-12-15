// compaund data types
// arrays, tuples, slices, and strings (sliced string)

fn main() {
    // let ints: [i32; 4] = [82, 37, 28, 45];
    // println!("Array of numbers: {:?}", ints);

    let strs: [&str; 3] = ["react", "vue", "svalte"];
    println!("My fav front end framework is: {}", strs[0])
}
