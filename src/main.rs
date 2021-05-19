// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // create random number
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    // when win then then true
    let mut win_or_lose_flag: bool = false;

    // rev() to reverse the range
    for count in (0..11).rev() {
        if 0 == count {
            break;
        }

        println!("Please input your guess.");
        println!("{} times remaining", count);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_error_message: String = String::from("please enter number!");
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
            Ordering::Less => {
                println!("Too small!");
                continue;
            }
            Ordering::Greater => {
                println!("Too big!");
                continue;
            }
            Ordering::Equal => {
                win_or_lose_flag = true;
                // quit loop
                break;
            }
        }
    }

    message_of_win_or_lose(secret_number, win_or_lose_flag);
}

fn message_of_win_or_lose(secret_number: u32, win_or_lose_flag: bool) {
    if win_or_lose_flag {
        println!("You win! secret number is {}", secret_number);
    } else {
        println!(
            "You lose! secret number is {}, please try again.",
            secret_number
        );
    }
}
