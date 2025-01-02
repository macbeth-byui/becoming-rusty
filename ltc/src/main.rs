use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader, Write, BufWriter};

use rand::prelude::*;
use clap::{Parser, ArgGroup};



#[derive(Parser, Debug)]
#[command(version, about = "Look to Christ")]
#[clap(group(
    ArgGroup::new("group")
        .required(true)
        .args(&["add", "display"])
))]
struct Args {
    #[clap(short, long, num_args = 2, value_names = &["REFERENCE", "TEXT"], help = "Add New Quote",)]
    add: Option<Vec<String>>,

    #[clap(short, long, help = "Display Random Quote")]
    display: bool,

    #[clap(short, long, num_args = 1, help = "Specify quote file (default c:\\data\\quotes.txt)")]
    filename: Option<String>,
}


#[derive(Debug)]
enum Errors {
    FileNotFound,
    FileEmpty,
    ReadError(usize),
    WriteError,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::FileNotFound => write!(f, "File Not Found"),
            Errors::FileEmpty => write!(f, "File Empty"),
            Errors::ReadError(row) => write!(f, "Read Error on Row {}", row),
            Errors::WriteError => write!(f, "Write Error"),
        }
    }
}

#[derive(Debug, Clone)]
struct Quote {
    pub reference : String,
    pub text : String,
}

fn add(filename : &str, reference : &str, text : &str) -> Result<(), Errors> {
    let file = File::options()
        .append(true)
        .create(true)
        .open(filename)
        .map_err(|_| Errors::FileNotFound)?;
    let mut buf_writer = BufWriter::new(file);
    writeln!(buf_writer, "{}|{}", reference, text)
        .map_err(|_| Errors::WriteError)?;
    Ok(())
}

fn select(filename : &str) -> Result<Quote, Errors> {
    let mut quotes = Vec::new();
    let file = File::open(filename).map_err(|_| Errors::FileNotFound)?;
    let buf_reader = BufReader::new(file);
    for (row, line) in buf_reader.lines().enumerate() {
        let line = line.map_err(|_| Errors::ReadError(row))?;
        let (reference, text) = line
            .split_once('|')
            .ok_or(Errors::ReadError(row))?;
        quotes.push(Quote { 
            reference : reference.to_string(), 
            text : text.to_string(),
        });
    }
    if quotes.is_empty() {
        return Err(Errors::FileEmpty);
    }
    let index = thread_rng().gen_range(0..quotes.len());
    Ok(quotes[index].clone())
}

fn main() {
    let args = Args::parse();
    let filename = match args.filename {
        Some(filename) => filename,
        None => "c:\\data\\quotes.txt".to_string()
    };
    if let Some(params) = args.add {
        match add(&filename, &params[0], &params[1]) {
            Ok(()) => println!("Added"),
            Err(error) => println!("Error: {}", error)
        }
    }
    else if args.display {
        match select(&filename) {
            Ok(quote) => println!("{} - {}", quote.reference, quote.text),
            Err(error) => println!("Error: {}", error) 
        };        
    }

}
