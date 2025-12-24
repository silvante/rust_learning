fn main() {
    // let mut counter: i32 = 0;
    // let result: i32 = loop {
    //     counter += 1;
    //     println!("counting {counter}");

    //     if counter == 10 {
    //         println!("{counter} X 2 =>");
    //         break counter * 2;
    //     };
    // };
    
    // println!("result out of loop is {result}")
    
    let mut counter: i32 = 0;

    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    let arr = [1, 2, 3, 4, 5, 6, 7];
    for element in arr {
        println!("in arr: {}", element)
    }
}
