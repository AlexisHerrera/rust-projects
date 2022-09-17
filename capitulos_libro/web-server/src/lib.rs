use std::{thread, sync::{mpsc, Mutex, Arc}};

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
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
        ThreadPool{workers, sender: Some(sender)}
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}
struct Worker {
    id: usize,
    thread : Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver:  Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // ver que este thread se bloquea hasta que recibe un job, si no pongo el loop
            // solo trabaja 1 vez este worker
            match receiver.lock().unwrap().recv() {
                Ok(job) =>  {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} is disconnected; shutting down.");
                    break;
                }
            };
        });
        Worker { id, thread: Some(thread) }
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropea al sender para que los otros hilos al hacer recv, lanze error
        // Nuevamente se dropea utilizando la tecnica option + take
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // Cambia el thread por un None, increible (Option + take)
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}