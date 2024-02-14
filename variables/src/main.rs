use std::process::exit;

const SEVEN: i32 = 7 * 2;

fn main() {
    let x = 5;
    {
        let x = 6 * SEVEN;
        println!("The value of inner x is: {x}");
        
        let x = "hello";
        println!("The value of inner x is now: {x}");
    }
    println!("The value of x is: {x}");
    exit(0)
}
