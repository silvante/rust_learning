fn main() {
    // shadowing is not that hard after all :)
    let var: i32 = 82;

    println!("var is {}", var);

    let var: i32 = var + var;

    println!("var is now shadowed and became {}", var)
}
