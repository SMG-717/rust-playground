use std::process::exit;

const SEVEN: u32 = 7 * 2;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6 * SEVEN;
    println!("The value of x is: {x}");
    x = 6 * number();
    println!("The value of x is: {x}");
    exit(0)
}

fn number() -> u32 {
    return 7 * 2;
}