/*
  All of these can be categorized as different approach of asynchronous programming:
  concurrency
  parallelism
  futures
  streams
  async/await syntax
  in general, tools for managing and coordinating between asynchronous programming

  - CPU-bound or compute-bound operations -> limited by the computer's potential data
  processing speed within the CPU or GPU
  - I/O bound operations -> limited by the speed of the computer's input and output ->
  example: network traffic, file system read, etc.

  - Operating system interrupts provide a form of concurrency
  - As we understand our program at a much more granular level than the OS does, we can
  spot opportunities for concurrency that th OS can't see

  - creating threads comes with its own overhead
  - writing non-blocking (asynchronous) code with the style of blocking code
  - async codes with futures or async/await syntax is easier to read and probably easier to reason

  - communication over channel VS communication over static memory (if the data is read-only)
  - awaiting on a join handle is one way of communicate the outcome of the spawned operation
  back to the caller

  - ??? spawning in tokio -> spawning a future ???
  - spawning from std::thread -> spawning a thread
*/

fn main() {
    println!("Hello, world!");
}
