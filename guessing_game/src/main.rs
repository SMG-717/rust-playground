use std::{io, process::exit};
use rand::Rng;
use colored::Colorize;
use std::cmp::Ordering;

fn main() {
    println!("Guess the secret number!");

    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        let message: String;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        if guess.trim() == "exit" {
            message = "Bye Loser.".red().to_string();
            println!("{message}");
            exit(0)
        }
        
        let guess: i32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_e) => {
                guess = guess.trim().to_string();
                message = format!("\"{guess}\" is not a number!").red().to_string();
                println!("{message}");
                continue;
            }
        };
        
        match guess.cmp(&secret) {
            Ordering::Greater => message = "Try smaller.".yellow().to_string(),
            Ordering::Less => message = "Try bigger.".yellow().to_string(),
            Ordering::Equal => {
                message = "You guessed correctly!".green().to_string();
                println!("{message}");
                exit(0)
            },
        }

        println!("{message}");
    }
}
