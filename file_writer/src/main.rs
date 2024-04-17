// extern crate termion;

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Write};

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "File Writer")]
struct Args {
    #[clap(help = "Filename")]
    filename: String,
}


fn write_file(filename : &str) -> io::Result<()> {
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(filename)?;
    let stream = io::stdin();
    let reader = stream.lock();
    for line in reader.lines() {
        file.write_all(format!("{}\n",line?).as_bytes())?;
    }
    
    Ok(())
}

fn main() {
    let args = Args::parse();
    write_file(&args.filename).unwrap_or_else(|err| println!("{}", err));
}
