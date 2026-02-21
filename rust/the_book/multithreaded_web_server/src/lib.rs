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
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread_handle = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job()
            }
        });

        Self {
            id,
            thread: thread_handle,
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
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
            sender: tx,
        }
    }

    pub fn execute(&self, func: impl FnOnce() + 'static + Send) {
        self.sender.send(Box::new(func)).unwrap();
    }
}
