mod first_module;
mod second_module;
mod third_module;

use crate::first_module::inside_first_module::some_function;
use crate::second_module::inside_second_module::second_inner_module::s;
use crate::third_module::inside_third_module::some_other_func;

fn main() {
    some_function();
    s();
    some_other_func();
    println!("Hello, world!");
}
