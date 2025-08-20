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

    - binary crate -> executable
    - library crate -> intended to use by others, cannot be executed on its own

    - semantic versioning (SemVer)

    - registry -> a copy of data from crates.io ???
    - updating the registry ???

    - reproducible builds with cargo.lock
    - cargo.lock file somehow locks the versions of dependencies
    - the project remains at versions indicated at cargo.lock until explicitly
    upgraded
    - the command 'cargo update' will ignore the cargo.lock and upgrade versions
    - cargo follows SemVer semantic versioning
    - for example specifying 0.8.5 for a crate (which is a shorthand for ^0.8.5)
    and then run 'cargo update' will upgrade the package to a version higher than
    0.8.5 but below 0.9.0
    - to upgrade the package to 0.9.x series, the cargo.toml file should be
    changed and the version of the package in the [dependencies] section should
    be changed

    - projects can be assembled from a number of packages

    - being local to the current thread of execution ???

    - 'cargo doc --open' command will build documentation provided by all the
    project's dependencies locally and open it into browser

    - a match expression is made up of arms
    - an arm consists of a pattern to match against

    - on windows pressing Enter return a carriage return and a newline (\r\n)
    - catch-all -> _ in match expressions
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
