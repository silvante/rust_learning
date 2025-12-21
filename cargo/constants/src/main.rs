fn main() {
    println!("Hello, world!");
    // so basically consts work like JS here
    // and they can not be mutable
    // also they should be all Cappital

    const Y: i32 = 82;
    println!("Y is {}", Y);
    println!("my height is {}", HEIGHT);
    println!("there is {} seconds in {} hours", HOURS_IN_SECONDS, HOURS);
}

// and surpisingly you can use const even outside of main fn
const HEIGHT: f32 = 184.2; 

const HOURS: i32 = 4;
const HOURS_IN_SECONDS: i32 = 60 * 60 * HOURS;