/*
    Rust categorizes errors as recoverable and unrecoverable

    - unrecoverable errors are always symptoms of bugs and the program
    should immediately stopped due to the error
    - some languages don't distinguish these two types of errors and have
    exception
    - Rust does not have exception
    - Rust has panic! for unrecoverable errors
    - Rust has Result<T, E> for recoverable errors


    Panics

    panics, by default, print the failure message, unwind, clean up the stack and quit

    - the stack can be traced by setting an environment variable: RUST_BACKTRACE=1
    - unwinding the stack VS aborting
    - walking back (unwinding) and cleaning up is a lot of work
    - by aborting, the memory the program was using needs to be cleaned up by OS

    in cargo.toml by setting this config, the program won't do unwinding on panic
    in release mode:
    [profile.release]
    panic = 'abort'

    switching from unwinding to abort upon panic, make resultant binary as small as
    possible

    - src/main.rs:2:5 -> second line, fifth character of main.rs file

    - in C, attempting to read beyond the end of a data structure is undefined bahavior.
    an attcker can manipulate the index in a way to read a memory location after the
    data structure and they may be able to read some data they shouldn't allow to
    - buffer overread
    - buffer overflow
    - the converse of buffer overflow is buffer over-read
    - buffer bound vulnerabilities
    - buffer over-run
*/

fn main() {
    println!("Hello, world!");
}
