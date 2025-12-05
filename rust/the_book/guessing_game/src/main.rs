use colored::{Color, Colorize};
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Enter yout guessed number:");

    let secret_number = rand::random_range(0..100);

    loop {
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("could not read user input");

        let user_input: u8 = match user_input.trim().to_string().parse() {
            Ok(user_input_number) if user_input_number <= 100 => user_input_number,
            _ => {
                println!("Enter a number between 0 and 100");
                continue;
            }
        };

        match user_input.cmp(&secret_number) {
            Ordering::Greater => {
                println!("{}", "Too HIGH!".color(Color::Red));
                continue;
            }
            Ordering::Less => {
                println!("{}", "Too LOW!".color(Color::Yellow));
                continue;
            }
            Ordering::Equal => {
                println!("{}", "You WON!".color(Color::Green));
                break;
            }
        }
    }
}
