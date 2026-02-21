use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
};

type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    id: usize,
    maybe_thread_handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread_handle = thread::spawn(move || {
            loop {
                let receiver_result = receiver.lock().unwrap().recv();

                match receiver_result {
                    Ok(job) => {
                        println!("Worket {id} got a job; executing...");
                        job()
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            }
        });

        Self {
            id,
            maybe_thread_handle: Some(thread_handle),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    maybe_sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(number_of_workers: usize) -> Self {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = vec![];
        for n in 1..=number_of_workers {
            let rx = Arc::clone(&rx);
            let worker = Worker::new(n, Arc::clone(&rx));
            workers.push(worker);
        }

        Self {
            workers,
            maybe_sender: Some(tx),
        }
    }

    pub fn execute(&self, func: impl FnOnce() + 'static + Send) {
        self.maybe_sender
            .as_ref()
            .unwrap()
            .send(Box::new(func))
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.maybe_sender.take());

        for worker in &mut self.workers {
            println!("Shutting down thread {}", worker.id);

            if let Some(thread_handle) = worker.maybe_thread_handle.take() {
                thread_handle.join().unwrap();
            }
        }
    }
}
