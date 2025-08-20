use colored::{Color, Colorize};
use rand::random_range;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Enter your guess: ");
    let secret_number: u8 = random_range(0..=100);

    loop {
        let mut user_input_guess = String::new();

        stdin()
            .read_line(&mut user_input_guess)
            .expect("cannot read standard input");

        let processed_user_guess: u8 = match user_input_guess.trim().parse() {
            Ok(guess) if guess <= 100 => guess,
            _ => {
                println!("Enter a number between 0 and 100");
                continue;
            }
        };

        match processed_user_guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You WON!".color(Color::BrightGreen));
                break;
            }
            Ordering::Greater => {
                println!("{}", "TOO BIG!".color(Color::Red));
            }
            Ordering::Less => {
                println!("{}", "TOO SMALL!".color(Color::BrightYellow));
            }
        }
    }
}
