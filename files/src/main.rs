mod app;
mod errors;
mod args;

use std::process::exit;
use crate::args::parse_args;
use crate::app::run;

// Entry Point for the App.  Will read the arguments from the
// command line and then run the app.  Any error detected while
// running the App will be displayed to stderr prior to exiting.
fn main() {
    let args = parse_args();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        exit(1);
    }
}
