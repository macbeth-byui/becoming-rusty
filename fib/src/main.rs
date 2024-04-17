use clap::Parser;
use num_bigint::BigInt;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "Fib Generator")]
struct Args {
    #[clap(short, help = "Nth Fib")]
    n : u16,
}

// BigInt supports larger results
fn fib(n : u16) -> BigInt {
    if n == 0 {
        return BigInt::from(0);
    }
    if n == 1 || n == 2 {
        return BigInt::from(1);
    }
    let mut fib1 = BigInt::from(1);
    let mut fib2 = BigInt::from(1);
    for _ in 3..=n {
        // Avoid a temp variable
        // With tuple destructing, ownership of fib1 and fib2 is passed back (no & needed)
        // We need to clone fib2 so that the original value can be used in the fib1 + fib2
        // With a simple swap like (fib1, fib2) = (fib2, fib1) no clone is needed
        (fib1, fib2) = (fib2.clone(), fib1 + fib2);
    }
    fib2
}

fn main() {
    let args = Args::parse();
    let result = fib(args.n);
    println!("{}", result);
}
