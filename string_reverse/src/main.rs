use clap::Parser;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "Reverse text")]
struct Args {
    text: String,
    #[clap(short, long, default_value_t = false, help = "Remove whitespaces")]
    remove: bool,
}

// Reverse a string using rev and collect
// Rust auto converts &String to &str
fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

// Remove whitespace using filter and collect
fn remove_ws(s: &str) -> String {
    // Can't compare &char and char.  Need to deref with *
    s.chars().filter(|c| *c != ' ').collect::<String>()
}

fn main() {
    let args = Args::parse();
    let mut result = reverse(&args.text);
    if args.remove {
        result = remove_ws(&result);
    }
    println!("{}", result);
}
