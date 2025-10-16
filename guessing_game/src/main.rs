// Guessing game. User will guess a number and the program will tell them if they are too high or too low. It will stop if correct. 

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if parse is successful, return num else put again
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // match the guess to the secret number 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Its too Small"),
            Ordering::Greater => println!("Its too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
            
    }
}
    
