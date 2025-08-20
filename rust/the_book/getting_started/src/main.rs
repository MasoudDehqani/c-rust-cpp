/*
    Installation of Rust using rustup:
    rustup is command-line tool for managing Rust versions and associated tools

    Rust needs a linker to work. Linker is a program Rust uses to join its compiled
    outputs into one file.
    a C compiler usually includes linker -> for example GCC or Clang
    a C compiler is also useful because some common Rust packages depend on C code and
    need a C compiler

    - local documentation: rustup doc

    rustc is the rust compiler which can be installed via rustup
    the convention for rust file names is to use underscore between words if the name
    contains more than one word

    by using this command:
    rustc main.rs
    outside a cargo project, an executable file is produced by the compiler wich can
    be executed by:
    ./main

    on windows it is: .\main

    as the file is an executable file, regardless of the OS, it should work

    - rustfmt is a formatter for Rust
    - standard Rust distribution

    - the 'main' function is special in Rust: it is always the first code that runs in
    every executable Rust program
    - Rust macros are a way to write code that generates code to extend Rust syntax

    - compiling and running are separate steps in Rust

*/

fn main() {
    println!("Hello, world!");
}
