//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    Job(Job),
    Halt
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Job(job) => {
                        debug!("Worker {} picked up a job, executing...", id);
                        job.call_box();
                        debug!("Worker {} finished job.", id);
                    },

                    Message::Halt => {
                        debug!("Worker {} terminating.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn assign<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let message = Message::Job(Box::new(f));
        self.sender.send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::Halt).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}