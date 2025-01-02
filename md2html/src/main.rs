use clap::Parser;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::str::Chars;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "File Reader")]
struct Args {
    #[clap(help = "Filename")]
    filename: String
}

enum State {
    Para,
    Header(u8),
    None,
}

struct Mapper {
    in_para : bool,
    in_header : bool
}

impl Mapper {

}

/*
 * Start with:
 * 1) <p>
 * 2) <h#>
 * 
 * If in para and # then look forward to match a header (#+ T*\n)
 *     If matched, then close para and start a header
 *     If not matched, then continue processing para
 * 
 * If in para and \n then look to match a new para (\n\n)
 *     If matched, then close para
 *     If not matched, then continue processing para
 * 
 * If in header and \n then close header
 * 
 * If in neither and # then look forwad to match a header 
 *     If matched, then start a header
 *     If not matched, then start a para
 * 
 * If in neither and T then start a para
 * 
 * If EOF then close header or para
 */

fn forward_check(mut stream : Chars, target : &str) -> bool {
    let mut peek = String::new();
    for _ in 1..target.len() {
        match stream.next() {
            Some(c) => peek.push(c),
            None => return false
        }
    }
    target == peek
}

fn convert(filename : &str) -> io::Result<()> {
    let input = File::open(filename)?;
    let output = File::create(PathBuf::from(filename).with_extension("html"))?;
    
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut in_para = false;
    let mut in_header = false;
    let mut header = 0_u8;
    let mut skip = 0_u8;

    for line in reader.lines() {
        let md_text = line?;
        let mut stream = md_text.chars();
        while let Some(c) = stream.next() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if c == '#' {
                if in_para || (!in_para && !in_header) {
                    if forward_check(stream.clone(), "# ") {
                        print!("</p>");
                        in_header = true;
                        skip = 2;
                        header = 1;
                        print!("\n<h1>")
                    }
                    else if forward_check(stream.clone(), "## ") {
                        print!("</p>");
                        in_header = true;
                        skip = 3;
                        header = 2;
                        print!("\n<h2>")
                    }
                    else if forward_check(stream.clone(), "### ") {
                        print!("</p>");
                        in_header = true;
                        skip = 4;
                        header = 3;
                        print!("\n<h3>")
                    }
                    else if forward_check(stream.clone(), "#### ") {
                        print!("</p>");
                        in_header = true;
                        skip = 5;
                        header = 4;
                        print!("\n<h4>")
                    }
                    else if forward_check(stream.clone(), "##### ") {
                        print!("</p>");
                        in_header = true;
                        skip = 6;
                        header = 5;
                        print!("\n<h5>")
                    }
                    else if forward_check(stream.clone(), "###### ") {
                        print!("</p>");
                        in_header = true;
                        skip = 7;
                        header = 6;
                        print!("\n<h6>")
                    }
                    else {
                        in_para = true;
                        print!("{}",c);
                    }
                } 
                else {
                    print!("{}",c);
                }
            }   
            else if c == '\n' {
                if in_para {
                    
                } else if in_header {

                }
                else {

                }
            }
        
        }
        if in_para {

        } 
        else if in_header {

        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(error) = convert(&args.filename) {
        println!("Error: {}", error)
    }
}


// fn convert_newline(md_text : &str) -> Option<String> {
//     if md_text == "\n" {
//         return Some(md_text.to_string());
//     }
//     None
// }

// fn convert_headers(md_text : &str) -> Option<String> {
//     if let Some(text) = md_text.strip_prefix("# ") {
//         return Some(format!("<h1>{}</h1>", text).to_string());
//     }
//     if let Some(text) = md_text.strip_prefix("## ") {
//         return Some(format!("<h2>{}</h2>", text).to_string());
//     }
//     if let Some(text) = md_text.strip_prefix("### ") {
//         return Some(format!("<h3>{}</h3>", text).to_string());
//     }
//     if let Some(text) = md_text.strip_prefix("#### ") {
//         return Some(format!("<h4>{}</h4>", text).to_string());
//     }
//     if let Some(text) = md_text.strip_prefix("##### ") {
//         return Some(format!("<h5>{}</h5>", text).to_string());
//     }
//     if let Some(text) = md_text.strip_prefix("###### ") {
//         return Some(format!("<h6>{}</h6>", text).to_string());
//     }
//     None
// }

// fn convert_line(md_text : &str) -> String {
//     /* Using:
//      * https://www.markdownguide.org/basic-syntax/
//      */
//     if let Some(html) = convert_headers(md_text) { return html; }
//     md_text.to_string()

// }
