fn main() {
    println!("Hello, world!");
    another();

    let x: i32 = 10;
    {
        let x = false; 
        println!("{}", x);
    }

    println!("{}", x);
    let y: i32 = 15;

    println!("{}", y);
}

fn another() {
    let x = successor(10);
    println!("Another {x}!")
}

fn successor(x: i32) -> i32 { return x + 1; }