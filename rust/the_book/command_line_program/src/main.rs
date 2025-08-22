/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {query} in the file {file_path}");

    let mut file_content = String::new();
    let file = File::open(file_path);

    file.iter().for_each(|mut f| {
        let r = f.read_to_string(&mut file_content);
        r.expect("");
    });

    let found = file_content.find(query);
    match found {
        Some(i) => println!("{}", file_content.split_off(i)),
        None => println!("Not Found"),
    };
}
