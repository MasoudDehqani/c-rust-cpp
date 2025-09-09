/*
    mutex -> mutual exclusion

    - the mutex is described as guarding the data it holds via the locking system
    - process -> OS
    - thread -> in the program
    - spawning threads come at a cost
    - there is no guarantee about the order in which parts of your code on different threads
    will run

    - race condition -> inconsistency in the order different threads access data or resources
    - deadlock -> thread mutually wait for each other preventing both thread from continuing

    - the Rust standard library uses 1:1 model for thread implementation whereby a program uses
    one operating system thread per one language thread. there are crates that implements other
    models

    - Rust async system provides another approach to concurrency

    - concurrency
    - parallelism

    - using sleep on the main thread let the other thread to run
    - the threads will probably take turns but there is no guarantee. it depends on how the
    operating system schedules threads
    - when the main thread of the Rust program completes, all spawned threads are shut down

    - a JoinHandle<T> is an owned value that when can call join method on it, will wait for
    its thread to finish.
    - calling join on a handle, block the thread currently running until the thread represented by
    the handle terminates
    - the T type would be the type of the return value of the closure passed to the spawn

*/

use std::thread;
use std::time::Duration;
mod data_sharing_by_message_passing;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 0..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    /*
        closures decides whether they take ownership of the values from their environment or borrow
        them, based on the code inside its body. so here without "move", the closure won't take the
        ownership of the v vector. it only take a reference to v vector (as the code inside its body
        does not need ownership of the vector). here Rust in fact infer how to capture v.
        Rust compiler does not allow the closures passed to spawned threads take a reference to values
        in their environment. Because there is no guarantee that the values live long enough until the
        thread finishes its work. Even using join on the handle won't work if we remove the move
        before the closure, because the main thread immediately will drop the vector after its last
        usage (at the end of its lifetime) in the current scope
        - note that here there is no value passed as argument to the closure. the closure only
        uses its ability to capture values from its environment
        - ??? If Rust allowed us to run this code (without using move), there’s a possibility that the
        spawned thread would be immediately put in the background without running at all ???
    */
    let v = vec![1, 2, 3];
    let handle_2 = thread::spawn(move || println!("vector: {v:?}"));

    // drop(v);

    handle_2.join().unwrap();

    data_sharing_by_message_passing::pass_message();
}
