use clap::Parser;

// Command Line Arguments for the App
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct AppArgs {
    #[arg(short = 'l')]
    long : bool,
}

// Parse the arguments from the command line
pub fn parse_args() -> AppArgs {
    AppArgs::parse()
}