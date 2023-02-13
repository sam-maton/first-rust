use colored::*;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    //Guessing Game
    let number = thread_rng().gen_range(0..100);

    loop {
        println!("Enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a number".red());
                continue;
            }
        };

        println!("Your guessed: {} ", guess.to_string().blue());

        match guess.cmp(&number) {
            Ordering::Less => println!("{}", "Too small!".bold()),
            Ordering::Greater => println!("{}", "Too big!".bold()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
        }
    }
}
