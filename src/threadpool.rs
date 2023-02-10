use std::sync::{Arc, mpsc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Sync + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {

    pub fn new(count: usize) -> Self {
        assert!(count > 0);
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(count);
        let receiver= Arc::new(Mutex::new(receiver));
        for i in 0..count {
            workers.push(Worker::new(i,Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self,f: F)
        where F: FnOnce() + Sync + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workders.");
        for i in self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}",worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


enum  Message {
    NewJob(Job),
    Terminate
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker: {} got a job; executing.",id);
                    },
                    Message::Terminate => {
                        println!("Worker: {} was told to terminate", id);
                    }
                }
            }
            receiver
        });

        Self {
            id,
            thread
        }
    }
}