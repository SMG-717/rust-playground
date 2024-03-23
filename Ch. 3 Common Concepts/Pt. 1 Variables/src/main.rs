fn main() {
    // Mutability
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // Constants
    const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60

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
    let pi: f32 = 3.14;
    let e: f64 = 2.7182818;

    // Numeric Operations
    let sum = 1 + e;
    let diff = e - 2;
    let prod = e * pi;
    let div = pi / e;
    let rem = pi % 3;

    // Booleans
    let active: bool = true;

    // Characters
    let letter: char = 'S';
    let funny: char = '😂';

    // Tuples
    let tup: (i32, f64, char) = (1, 2.5, 'Y');
    let (a, b, c) = tup; // Destructuring!
    let middle = tup.1;

    // Arrays
    let a: [i32; 5] = [1, 4, 9, 16, 25];
    let sevens = [7; 5]; // Short hand for [7, 7, 7, 7, 7]
    let head = a[0];
}
