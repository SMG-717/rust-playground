use rand::Rng;

fn main() {
    println!("Hello, world!");
    let x = number();

    another();
    another_params(10, "yo!");
    another_params(x, "man.");
    another_params(successor(x), "dude?");
}

// Function
fn another() {
    println!("Another function!")
}

// Function with parameters
fn another_params(x: i32, phrase: &str) {
    println!("X is {x}, {phrase}")
}

// Function with a return value
fn number() -> i32 {
    return rand::thread_rng().gen_range(1..=100);
}

// Function with parameters and a return value
fn successor(x: i32) -> i32 { 
    x + 1
}