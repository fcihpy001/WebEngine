use std::fmt::Debug;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;


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
            workers.push(
                Worker::new(
                    i,
                    Arc::clone(&receiver))
            );
        }
        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self,f: F)
        where
            F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        // 向通道发送任务消息
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workders.");
        for _ in self.workers.iter() {
            self.sender.send(Message::Terminate).unwrap()
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}",worker.id);

            // worker.thread.join().unwrap();
            // if let Some(thread) = worker.thread.take(3) {
            //     thread.join().unwrap();
            // }
        }
    }
}

pub enum  Message {
    NewJob(Job),
    Terminate
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn( move ||{
            loop {
                // 从通道接收端提取出任务
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(_job) => {
                        // job.call_box();
                        println!("Worker: {} got a job; executing.",id);
                    },
                    Message::Terminate => {
                        println!("Worker: {} was told to terminate", id);
                    }
                }
            }
        });

        Self {
            id,
            thread
        }
    }
}