use std::{sync::{mpsc, Arc, Mutex},thread};


#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
// Recall the thread-safe smart pointers discussed in Chapter 16:
// to share ownership across multiple threads and allow the threads to mutate the value,
// we need to use Arc<Mutex<T>>. The Arc type will let multiple Worker instances own the receiver,
// and Mutex will ensure that only one Worker gets a job from the receiver at a time.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }
// From https://doc.rust-lang.org/book/ch21-02-multithreaded.html#building-threadpool-using-compiler-driven-development
//```rust
//     pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T,
//         F: Send + 'static,
//         T: Send + 'static,
//```
// The F type parameter is the one we’re concerned with here;
// the T type parameter is related to the return value, and we’re not concerned with that.
// We can see that spawn uses FnOnce as the trait bound on F.
// This is probably what we want as well,
// because we’ll eventually pass the argument we get in execute to spawn.
// We can be further confident that FnOnce is the trait we want to use because
// the thread for running a request will only execute that request’s closure one time,
// which matches the Once in FnOnce.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// We’ll also change Job from a struct to a type alias for a trait object
// that holds the type of closure that execute receives.
// As discussed in “Creating Type Synonyms with Type Aliases” in Chapter 20,
// type aliases allow us to make long types shorter for ease of use.
type Job = Box<dyn FnOnce() + Send + 'static>;

// 7/10/2025 TODO: Review Arc<Mutex>
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
                
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
