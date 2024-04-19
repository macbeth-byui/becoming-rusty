use std::thread::{self, JoinHandle, ThreadId};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;

enum ThreadMsg {
    Closing,
    Heartbeat,
    Terminate
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
    /* If the ThreadFamily goes out of scope, then we need to request
     * the handler thread to exit. 
     */
    fn drop(&mut self) {
        // Notify handler to stop
        let _ = self.tx.send((thread::current().id(), ThreadMsg::Terminate));

        // Just in case, clean up mutexes
        self.threads.clear_poison();
        self.queue.clear_poison();

        // Clear the queue
        if let Ok(mut queue) = self.queue.lock() {
            queue.clear();
        };

        // Wait for all threads to complete
        // TODO: What if one is blocking forever??
        if let Ok(mut threads) = self.threads.lock() {
            // The join will consume the JoinHandle.  This requires that I
            // transfer ownership out of the HashMap.  This can only
            // happen if use remove_entry which requires a mutable reference to the
            // HashMap.  However, this also means I can't iterate through the 
            // HashMaps via keys or values since that will require a reference as well.
            // To solve this we create a new list of keys and then loop through
            // those keys to call remove_entry.  No borrowing problems.
            let mut keys = Vec::<ThreadId>::new();
            for key in threads.keys() {
                keys.push(*key);
            }
            for key in keys {
                if let Some((_,handler)) = threads.remove_entry(&key) {
                    let _ = handler.join();
                }
            }
        }

    }
}

impl<F> ThreadFamily<F> 
    where F: FnOnce() + Send + 'static
{
    /* Create a new ThreadFamily.  After initializing data, the handler
     * thread will be started to receive messages send by threads
     * in the ThreadFamily.
     */
    pub fn new(max : usize) -> Self {
        // Create Atomically Reference Counted (ARC) data for 
        // shared use between the threads in the ThreadFamily.
        let threads = Arc::new(Mutex::new(HashMap::new()));
        let queue = Arc::new(Mutex::new(Vec::new()));

        // Setup communication channel between the threads and the ThreadFamily
        let (tx, rx) = channel();

        // Create the ThreadFamily object
        let mut thread_family = ThreadFamily {threads, queue, max, tx};

        // Start the handler thread 
        thread_family.handler(rx);

        thread_family
    }

    /* The handler thraed will receive all messages sent from threads to the 
     * ThreadFamily.  The threads include ones started by the ThreadFamily and
     * also from the owner of the ThreadFamily object.  The following messages
     * are processed:
     * 
     *    - ThreadMsg::Closing - Message from a ThreadFamily managed thread
     *            indicating the thread is closing.  The thread handle
     *            will be removed from the threads map using the provided
     *            thread id.  If the queue is non-empty, then the next
     *            pending thread will be spawned.
     *     - ThreadMsg::Terminate - Message sent to the handler to indicate
     *            the handler should terminate.  This is sent exclusively
     *            by the drop function in ThreadFamily.
     *     - ThreadMsg::Heartbeat - Sent by thread to ensure that the 
     *            handler is still running.  No acknowledgement is sent.
     *            The intent is that the tx function will fail if the
     *            handler process is no longer running.  Note that ownership
     *            of rx is sent to the handler thread.
     */
    fn handler(&mut self, rx : Receiver<(ThreadId, ThreadMsg)>, ) {
        // Create new shared references to threads, queue, and tx
        let threads = Arc::clone(&self.threads);
        let queue = Arc::clone(&self.queue);
        let tx = self.tx.clone();

        // Spawn the handler thread transfering ownership of the the 
        // shared references.
        let _ = thread::spawn(move || {
            for (id, msg) in rx {
                match msg {
                    ThreadMsg::Closing => {
                        // Lock the shared resoures.  If failed, then the implication
                        // is that the owner of the ThreadFamily object has panicked.
                        // Therefore the handler here will exit.
                        let mut threads = match threads.lock() {
                            Ok(guard) => guard,
                            Err(_) => break
                        };

                        let mut queue = match queue.lock() {
                            Ok(guard) => guard,
                            Err(_) => break
                        };

                        // Remove the closing thread from the map.  If for some reason
                        // the thread id does not exist (unexpected situation), the 
                        // error will just be ignored.
                        let _ = threads.remove(&id);

                        // If there are pending thread requests, then spawn the 
                        // next one from the queue.
                        if queue.len() > 0 {
                            // Dequeue the next requent and spawn the thread.
                            let closure = queue.remove(0);
                            // Create a new shared reference to the tx to allow the 
                            // thread to communicate to the handler when it is finished.
                            let thread = ThreadFamily::spawn_thread(closure, &tx);

                            // Add the newly created thread to the threads map
                            threads.insert(thread.thread().id(), thread);
                        }

                        // Locks will unlock automatically here
                    }
                    ThreadMsg::Terminate => break,
                    ThreadMsg::Heartbeat => ()
                }
            }
            // println!("ThreadFamily Handler Closing.");
        });
    }

    /* Utility to spawn the thread, run the closure, and then notify the
     * ThreadFamily handler that the thread is completed.
     * 
     * Returns the JoinHandle of the thread.
     */
    fn spawn_thread(closure: F, tx : &Sender<(ThreadId, ThreadMsg)>, ) -> JoinHandle<()> {
        // Create a new shared reference to the tx to allow the 
        // thread to communicate to the handler when it is finished.
        let tx = tx.clone();
        
        // Spawn the thread
        thread::spawn(move || {
            // Run the request code in the thread
            closure();

            // When it is completed, notify the handler that is done.
            // We are ignoring any error since that implies the handler was
            // already closed.
            let _ = tx.send((thread::current().id(), ThreadMsg::Closing)); 
        })
    }

    /* Request a thread to be managed by the ThreadFamily.  If there is 
     * availability to spawn the thread, then it will be created immediately.
     * Otherwise, it will be added to the queue.  Once a thread completes, then
     * one of the requests in the queue will be processed.
     * 
     * Returns None if one of the following errors occurred:
     *     - Failed to send a ThreadMsg::Heartbeat to the handler.  This implies
     *       that the handler thread closed. 
     *     - Locking of shared resources failed.  This implies that the handler
     *       thread closed.
     * 
     * If error occurs, the the owner of the ThreadFamily should drop the 
     * ThreadFamily object.
     */
    pub fn request(&mut self, closure: F) -> Option<()> {
        // Verify that the handler thread is still running
        if self.tx.send((thread::current().id(), ThreadMsg::Heartbeat)).is_err() {
            return None;
        }
        
        // Lock the threads shared resource
        let mut threads = match self.threads.lock() {
            Ok(guard) => guard,
            Err(_) => return None
        };
        
        // If there is no room for a new thread, then add the request to the queue.
        if threads.len() == self.max {
            // Lock the queue shared resource
            let mut queue = match self.queue.lock() {
                Ok(guard) => guard,
                Err(_) => return None // threads lock will autommatically unlock here
            };

            // Enqueue the request
            queue.push(closure);
        }
        // If there is room for the thread
        else {
            let thread = ThreadFamily::spawn_thread(closure, &self.tx);

            // Add the newly created thread to the threads map
            threads.insert(thread.thread().id(), thread);
        }
        // Locks will automatically unlock here
        Some(())
    }

}



