/*
  - Parsing has a long, rich history in computer science that is closely tied to the
  artificial intelligence community. Many of the techniques used today to parse programming
  languages were originally conceived to parse human languages by AI researchers who were
  trying to get computers to talk to us

  - scanning - lexing - lexical analysis - scanner - lexer - token
  - parsing - grammar - parser - parse tree - abstract syntax tree (AST) - syntax tree - syntax error
  - static analysis - binding - resolution - identifier - scope - type error - attributes -
  symbol table
  - everything up to this point is considered the font end of the implementation
  - middle end and back end remains
  - intermediate representation (IR)
  - optimization - constant folding
  - code generation -> back end

  - There are 2 main techniques for compilers/interpreters for managing memory:
  1. reference counting
  2. tracing garbage collection (usually just called garbage collection or GC)

  - clean up object cycles

  - Where an expression’s main job is to produce a value, a statement’s job is to produce an effect

  - expression statement

  - argument (actual parameter)
  - parameter (formal parameter or formals)

  - lexical (static) scoping VS dynamic scoping

  Classes or Prototypes:
  - In class-based languages, there are two core concepts: instances and classes.
  - Prototype-based languages merge these two concepts. Objects can directly inherit from each other (or “delegate to” in prototypal lingo)

  - base class or super class
  - derived class or subclass

  - indirection

  - The first step in any compiler or interpreter is scanning -> produces tokens -> will then be fed to the parser

  - side note form print! macro usage:
  Note that stdout is frequently line-buffered by default so it may be necessary to
  use io::stdout().flush() to ensure the output is emitted immediately
*/

use lox_rs::Scanner;
use std::{
    env, fs,
    io::{self, Write},
    process,
};

fn main() {
    let mut args = env::args();

    match args.len() {
        0 => panic!("How is it possible???"),
        1 => {
            let app_binary_name = args.nth(0).unwrap();
            println!("{}", app_binary_name);
            run_prompt();
        }
        2 => run_file(args.nth(1).unwrap()),
        3.. => {
            println!("Usage: lox_rs [script]");
            process::exit(1);
        }
    }
}

fn run_file(path: String) {
    let source = fs::read_to_string(path).unwrap();
    run(source)
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_bytes_read) => run(line),
            Err(e) => {
                println!("Error reading line {e}");
                break;
            }
        }
    }
}

fn run(source: String) {
    Scanner::scan(source);
}
