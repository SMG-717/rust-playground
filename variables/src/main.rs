use std::process::exit;

const SEVEN: i32 = 7 * 2;

fn main() {
    let x = 5;
    {
        let x = 6 * SEVEN;
        println!("The value of inner x is: {x}");
        
        let x = "hello";
        println!("The value of inner x is now: {x}");

        let mut nums = [1, 2, 3, 4, 5];
        nums[2] = 2;

        
        let guess: u32 = "42".parse().expect("Not a number!");

        let mut elements = ("hello", 6, false, guess);
        elements.1 = 1
    }
    println!("The value of x is: {x}");
    exit(0)
}
