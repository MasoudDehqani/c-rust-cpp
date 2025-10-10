/*
  The comments with triple slash (///) are doc comments
  By running the "cargo doc --open" command, an html file will be opened, and
  there would be a section for ThreadPool struct which can be clicked then
  these explanations inside the doc comment will appear there

  worker -> In a worker pool implementation, a worker is a unit of execution responsible for
  performing a specific task from a job queue
*/

// struct ThreadPool;

// impl ThreadPool {
//     /// Create a new ThreadPool.
//     ///
//     /// The size is the number of threads in the pool.
//     ///
//     /// # Panics
//     ///
//     /// The `new` function will panic if the size is zero.
//     pub fn new(size: usize) -> Self {
//         assert!(size > 0);
//     }
// }

// TODO: change new associated function to "build" with this signature
// pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>
//
// pub struct ThreadPool;
// impl ThreadPool {
//     pub fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);
//         ThreadPool
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//     }
// }
