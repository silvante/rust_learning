fn main() {
    let mut counter: i32 = 0;

    // let result: i32 = loop {
    //     counter += 1;
    //     println!("counting {counter}");

    //     if counter == 10 {
    //         println!("{counter} X 2 =>");
    //         break counter * 2;
    //     };
    // };

    // println!("result out of loop is {result}")

    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
        }
    }
}
