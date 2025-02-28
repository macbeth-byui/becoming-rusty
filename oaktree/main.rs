// extern crate termion;

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use console::{Term,Key,Style};

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "File Reader")]
struct Args {
    #[clap(short, long, default_value_t = false, help = "Display Pages")]
    page: bool,

    #[clap(help = "Filename")]
    filename: String
}

fn display_screen(reader : &mut BufReader::<File>, term: &Term, row : u16) -> Option<u64> {
    let mut first = true;
    let mut next_pos = 0;
    let (rows, cols) = term.size();
    let mut eof = false;
    let mut eof_row = 0;
    term.move_cursor_to(0,0).ok()?;
    for i in 0..(rows-1) {
        term.move_cursor_to(0, i as usize).ok()?;
        term.clear_line().ok()?;
        let mut line = String::new();
        let size = reader.read_line(&mut line).unwrap();
        if size == 0 {
            if !eof {
                eof = true;
                eof_row = i;
            } 
            print!("");
            io::stdout().flush().unwrap();   
        }
        else {
            line.truncate(cols as usize - 5);
            print!("{:04} {}",
                Style::new().green().apply_to(row+i),
                line);
            io::stdout().flush().unwrap();   
        }
        if first {
            next_pos = reader.stream_position().unwrap();
            first = false;
        }
    }
    
    if eof {
        term.move_cursor_to(0, eof_row as usize).ok()?;
        print!("{}",Style::new().red().apply_to("--EOF--"));
        return None;
    }
    Some(next_pos)
    

}

fn display_file(filename : &str) -> io::Result<()> {
    let term = Term::stdout();
    let (rows, _) = term.size();

    let file = File::open(filename)?;
    let mut at_bottom = false;
    let mut row_pos = vec![0_u64];
    let mut row = 0;

    let mut reader = io::BufReader::new(file);
    loop {
        reader.seek(SeekFrom::Start(row_pos[row]))?;            

        let next_pos = display_screen(&mut reader, &term, (row+1) as u16).unwrap_or_else(|| { at_bottom = true; 0});

        let mut valid_cmd = false;
        while !valid_cmd {
            term.move_cursor_to(0, rows as usize - 1)?;
            print!("{} ESC: exit",Style::new().yellow().apply_to(filename));
            if row > 0 {
                print!(" {}: up",24 as char);
            }
            if !at_bottom {
                print!(" {}: down",25 as char);
            }
            io::stdout().flush()?;
    
            let key = term.read_key()?;
            if key == Key::ArrowUp && row > 0 {
                valid_cmd = true;
                row -= 1;
                at_bottom = false;
            }
            else if key == Key::ArrowDown && !at_bottom {
                valid_cmd = true;
                row += 1;
                if row == row_pos.len() {
                    row_pos.push(next_pos);
                }
            }
            else if key == Key::Escape {
                term.clear_line()?;
                return Ok(());
            }
            term.clear_line()?;
        }
    }
}

fn main() {
    let args = Args::parse();
    display_file(&args.filename).unwrap_or_else(|err| println!("{}", err));
}
