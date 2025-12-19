// so for some reason all names of functions and vars should be in snake case (I like it)
fn main() {
    let result: i32 = JustStruct::add(48, 82);

    println!("result is: {}", result);

    println!("function add returned, {}. ", JustStruct::add(37, 82));

    let weight: f32 = 77.0;
    let height: f32 = 1.84;

    let bmi: f32 = JustStruct::col_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

struct JustStruct {}

impl JustStruct {
    fn add(a: i32, b: i32) -> i32 {
        return a + b
    }
}

impl JustStruct {
    fn col_bmi(kg: f32, hm: f32) -> f32 {
        return kg / (hm * hm)
    }
}
