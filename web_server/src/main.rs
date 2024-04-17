// extern crate termion;
mod request;
mod response;
mod method;
mod client;
mod server;
mod file_system;
mod thread_family;

use clap::Parser;
use std::io::{self, BufRead};
use std::net::TcpListener;
use std::str;
use std::thread;
use file_system::FileSystem;
use server::Server;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "File Writer")]
struct Args {
    #[clap(help = "IP Address")]
    ip_address : String,

    #[clap(help = "Port Number")]
    port : u16,

    #[clap(help = "Root Path")]
    root_path : String
}

fn start(ip_address : &str, port : u16, root_path : &str) -> Result<(),String> {
    // Must have a valid root path
    let file_system = FileSystem::new(root_path);
    file_system.check_folder()
        .map_err(|err| format!("Root path does not exist\n{}",err))?;

    // Must successfully create the server socket
    let listener = TcpListener::bind(format!("{}:{}", ip_address, port))
        .map_err(|err| format!("Unable to create server socket\n{}",err))?;

    let server = Server::new(listener, file_system.clone());
    let _ = thread::spawn(move || server.run());

    run_shell();

    Ok(())
}


fn run_shell() {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        println!("> ");
        input.clear();
        stdin.lock().read_line(&mut input).unwrap_or(0);
        input = input.trim().to_uppercase();
        match input.as_str() {
            "EXIT" => break,
            "LOG" => println!("LOG DISPLAY"),
            "ACTIVE" => println!("ACTIVE DISPLAY"),
            _ => ()
        }
    }

    // TODO: If Exit shell, should i wait for threads to close?
    // TODO: If server thread dies, should I panic?
    // TODO: Support Pictures
    // TODO: Log and Active commands
    // TODO: Debug command
    // TODO: Config File


}

fn main() {
    let args = Args::parse();
    if let Err(err) = start(&args.ip_address, args.port, &args.root_path) {
        println!("Error: {}", err);
    }
}

