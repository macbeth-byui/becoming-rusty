use std::thread::{self, JoinHandle, ThreadId};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;

// TODO: Prevent Panics to keep running

enum ThreadMsg {
    Terminate,
    Heartbeat,
    Panic
}

pub struct ThreadFamily<F> 
    where F: FnOnce() + Send + 'static
{
    threads : Arc<Mutex<HashMap<ThreadId, JoinHandle<()>>>>,
    queue : Arc<Mutex<Vec<F>>>,
    max : usize,
    tx : Sender<(ThreadId, ThreadMsg)>,
}

impl<F> Drop for ThreadFamily<F>
    where F: FnOnce() + Send + 'static
{
    fn drop(&mut self) {
        let _ = self.tx.send((thread::current().id(), ThreadMsg::Panic));
        // println!("Forcing ThreadFamily Handler to Exit");
    }
}

impl<F> ThreadFamily<F> 
    where F: FnOnce() + Send + 'static
{
    pub fn new(max : usize) -> Self {
        let threads = Arc::new(Mutex::new(HashMap::new()));
        let queue = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = channel();
        let mut thread_family = ThreadFamily {threads, queue, max, tx};
        thread_family.handler(rx);
        thread_family
    }

    fn handler(&mut self, rx : Receiver<(ThreadId, ThreadMsg)>) {
        let threads = Arc::clone(&self.threads);
        let queue = Arc::clone(&self.queue);
        let tx = self.tx.clone();
        let _ = thread::spawn(move || {
            for (id, msg) in rx {
                match msg {
                    ThreadMsg::Terminate => {
                      
                        let mut threads = match threads.lock() {
                            Ok(guard) => guard,
                            Err(_) => break
                        };

                        let mut queue = match queue.lock() {
                            Ok(guard) => guard,
                            Err(_) => break
                        };

                        threads.remove(&id); // Ignore error

                        if queue.len() > 0 {
                            let closure = queue.remove(0);
                            let tx = tx.clone();
                            let thread = thread::spawn(move || {
                                closure();
                                tx.send((thread::current().id(), ThreadMsg::Terminate)).ok(); // Ignore Error
                            });
                            threads.insert(thread.thread().id(), thread);
                        }
                    }
                    ThreadMsg::Panic => break,
                    ThreadMsg::Heartbeat => ()
                }
            }
            println!("ThreadFamily Handler Closing.");
        });
    }

    pub fn request(&mut self, closure: F) -> Option<()> {

        if self.tx.send((thread::current().id(), ThreadMsg::Heartbeat)).is_err() {
            return None;
        }
        
        let mut threads = match self.threads.lock() {
            Ok(guard) => guard,
            Err(_) => return None
        };
        
        if threads.len() == self.max {
            let mut queue = match self.queue.lock() {
                Ok(guard) => guard,
                Err(_) => return None
            };
            queue.push(closure);
        }
        else {
            let tx = self.tx.clone();
            let thread = thread::spawn(move || {
                closure();
                tx.send((thread::current().id(), ThreadMsg::Terminate)).ok();
            });
            threads.insert(thread.thread().id(), thread);
        }
        Some(())
    }

}



