/*
    there is a set of items in the standard library Rust brings them into the
    scope of every program. this set is called prelude

    - if you need something in the standard library which isn't is prelude,
    you should bring it into the scope using 'use' keyword
    - io (input/output) library comes from standard library (std)

    - the 'main' function is the entry point to every Rust program

    - binding a name (variable or identifier) to a value using 'let' keyword
    and '=' sign (maybe can be called assignment operator as in C)
    - immutable by default
    - mut keyword for making it mutable

    - comment syntax include: // -> for one line and /* */ -> for multiline
    - associated function
    - std::io::stdin
    - the stdin function return an instance of std::io::Stdin type

    - single logical line of code
    - we call each possible state of an enumeration (enum), a variant
    - read_line method, if successful, returns Ok of the numbers of bytes
    in the users's input

    - the set of curly brackets inside the format string in println! macro is
    called placeholder

    - println!("{}", expression);
    - println!("{:?}", expression);
    - println!("{:#?}", expression);
    - println!("{expression:?}");
    - println!("{expression:#?}");
    - println!("{0}, {1}", expression1, expression2);
*/

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
