
use std::net::TcpListener;
use std::time::Duration;
use crate::file_system::FileSystem;
use crate::client::Client;
use crate::thread_family::ThreadFamily;

pub struct Server 
{
    listener : TcpListener,
    file_system : FileSystem,
}

impl Server
{

    /* Create a new server which is defined by an already created
     * TCPListener and a FileSystem.
     */
    pub fn new(listener : TcpListener, file_system : FileSystem) -> Self {
        Server { listener, file_system }
    }

    /* The server thread will start by creating a ThreadFamily to manage
     * all active and pending client threads.  The server will block waiting for
     * a client to connect.  
     */
    pub fn run(&self) {
        // Create the ThreadFamily
        let mut thread_family = ThreadFamily::new(5);

        // Listen for client connections
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                // Set client read timeout to 10 seconds (client thread will not block forever)
                if stream.set_read_timeout(Some(Duration::from_secs(10))).is_err() {
                    // If this fails then someone is wrong.  Close the server.
                    break;
                }

                // Create a new client object
                // TODO: Is there any reason we want to put a mutex on this?  Or is there a 
                // way to do a singleton?
                let mut client = Client::new(stream, self.file_system.clone());

                // Give the client thread function to the thread family.  Note that we are 
                // transfering ownership of the client to the thread.
                if thread_family.request(move || client.run()).is_none() {
                    // If the ThreadFamily fails, then it is not recoverable.  Restart the server.
                    // TODO: Create a new ThreadFamily?
                    break;
                }
            } 
            else {
                // If we fail to listen for a new client then the server socket has been broken.
                break;
            }
        }
        println!("Server Closing");
    }
}
