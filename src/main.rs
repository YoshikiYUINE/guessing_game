// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_error_message: &str = "please enter number!";
        // u32 type is 32 bit unsigned integers
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", guess_error_message);
                continue;
            }
        };

        println!("You guessed: {}", guess);
        // The first new bit here is another use statement, bringing a type called
        // std::cmp::Ordering into scope from the standard library. Like Result, Ordering is
        // another enum, but the variants for Ordering are Less, Greater, and Equal.
        // These are the three outcomes that are possible when you compare two values.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! secret number is {}", secret_number);
                // quit loop
                break;
            }
        }
    }
}

/*
fn main() {
    println!("Hello, world!");
}
*/
