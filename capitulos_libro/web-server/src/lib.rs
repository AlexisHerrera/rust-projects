use std::{thread, sync::{mpsc, Mutex, Arc}};

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender: mpsc::Sender<Job>
}
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Create a new ThreadPool
/// The size is the number of threads in the pool
/// 
/// # Panics
/// 
/// The 'new' function will panic if the size is zero
impl ThreadPool {
    pub fn new(size: usize)-> Self {
        // Change panic macro with a Result type
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}
struct Worker {
    id: usize,
    thread : thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver:  Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // ver que este thread se bloquea hasta que recibe un job, si no pongo el loop
            // solo trabaja 1 vez este worker
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }

}