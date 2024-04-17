use clap::Parser;
use num_bigint::BigInt;
use std::str::FromStr;
use num_traits::Zero;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "Prime Checker")]
struct Args {
    #[clap(help = "Number to Check")]
    n : String,
}

fn is_prime(n_str : &str) -> Result<bool, String> {
    // map_err will convert the error to the type I need
    // The ? will immediately return if error.
    let n = BigInt::from_str(n_str)
        .map_err(|e| format!("Parse Error: {}", e))?;
    
    if n <= BigInt::zero() {
        return Err("Value Error: Must be positive".to_string());
    }

    // Will truncate
    let max = n.sqrt();

    // range_inclusive gives actual values that get consumed each iteration
    for x in num_iter::range_inclusive(BigInt::from(2), max) {
        // mod with BigInt supports working with a reference (borrow n from outside of the loop)
        if &n % x == BigInt::zero() {
            return Ok(false);
        }
    }
    Ok(true)
}

fn main() {
    let args = Args::parse();

    let result = is_prime(&args.n);
    match result {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not Prime"),
        Err(err) => println!("{}",err)
    }
}
