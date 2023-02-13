use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    //Guessing Game
    let number = thread_rng().gen_range(0..10);

    loop {
        println!("Enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("Your guessed: {} ", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
