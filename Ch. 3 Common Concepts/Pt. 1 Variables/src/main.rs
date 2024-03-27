fn main() {
    // Mutability
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // Constants
    const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60;

    // Shadowing
    let x = 5;
    {
        let x = 6 * SECONDS_IN_A_DAY;
        println!("inner x = {x}");
        
        let x = "hello";
        println!("inner x now = {x}");
    }
    println!("outer x = {x}");

    // Floating Points
    let _pi: f32 = 3.14;  
    let _e: f64 = 2.7182818;
    
    // Note: A variable name that starts with an underscore 
    // tells rust that we don't intend to use it later

    // Numeric Operations
    let _sum = 1 + 2;
    let _diff = 16 - 3;
    let _prod = 4 * 5;
    let _div = 7 / 8;
    let _rem = 10 % 9;

    // Booleans
    let _active: bool = true;

    // Characters
    let _letter: char = 'S';
    let _funny: char = '😂';

    // Tuples
    let tup = (1, 2.5, 'Y');
    let (_a, _b, _c) = tup; // Destructuring!
    let _middle = tup.1;

    // Arrays
    let a: [i32; 5] = [1, 4, 9, 16, 25];
    let _sevens = [7; 5]; // Short hand for [7, 7, 7, 7, 7]
    let _head = a[0];
}
