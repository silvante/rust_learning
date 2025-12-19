fn main() {
    let mut _owner1: i32 = 82;

    let _owner2: &mut i32 = &mut _owner1;

    *_owner2 += 2;

    println!("value of owner1 is {}", _owner1); // 84
}
