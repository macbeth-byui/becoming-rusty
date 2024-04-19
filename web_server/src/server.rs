
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

    pub fn new(listener : TcpListener, file_system : FileSystem) -> Self {

        Server { listener, file_system }
    }

    pub fn run(&self) {
        let mut thread_family = ThreadFamily::new(5);
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                if stream.set_read_timeout(Some(Duration::from_secs(10))).is_err() {
                    // println!("Failure to set read timeout");
                    break;
                }
                let mut client = Client::new(stream, self.file_system.clone());
                if thread_family.request(move || client.run()).is_none() {
                    // println!("ThreadFamily Failure.");
                    break;
                }
            } 
            else {
                // println!("Server failed to connect.");
                break;
            }
        }
        println!("Server Closing");
    }
}
