use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct PoolCreationError {
    message: String,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                // println!("worker {} got a job. executing...", id);
                job();
            }
        });
        Worker { id, thread }
    }
}

impl ThreadPool {
    // Create a new ThreadPool
    //
    // `size` is the number of threads in the pool.
    //
    // # Panics
    //
    // `new` will panic if the size is 0.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                let worker = Worker::new(id, Arc::clone(&receiver));
                workers.push(worker);
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError {
                message: "size must be greater than 0".to_string(),
            })
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
