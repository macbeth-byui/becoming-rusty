use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io;


pub trait PasswordString {
    fn contains_number(&self) -> bool;
    fn contains_upper(&self) -> bool;
}

// All String objects now have these functions

impl PasswordString for String {
    fn contains_number(&self) -> bool {
        for letter in self.chars() {
            if letter.is_ascii_digit() {
                return true;
            }
        }
        for letter in self.chars() {
            if letter.is_ascii_digit() {
                return true;
            }
        }
        false
    }

    fn contains_upper(&self) -> bool {
        for letter in self.chars() {
            if letter.is_ascii_uppercase() {
                return true;
            }
        }
        false
    }
}

// Pass vector &mut which is by mutable reference (change)
fn keep_evens(numbers : &mut Vec<i32>, value : i32) {
    if value % 2 == 0 {
        numbers.push(value);
    }
}

// Return back None if there is a problem
fn int_divide(a : i32, b : i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

// Return back sha256 string or error message
fn password_valid(password : &String) -> Result<String, String> {
    if password.len() < 8 {
        return Err(String::from("Password too short."));
    }
    if password.contains("password") {
        return Err(String::from("Password cannot contain common phrases."));
    }
    if !password.contains_number() {
        return Err(String::from("Password must contain at least 1 number."));
    }
    if !password.contains_upper() {
        return Err(String::from("Password must have at least 1 upper case letter."))
    }
    let mut sha256 = Sha256::new();
    sha256.input_str(password);
    Ok(sha256.result_str())
}

fn main() {
    // Hello World
    println!("Hello, world!");

    // Variables, Types, Mutability
    let x: i32 = 5;
    let mut y: i8 = 10;

    // y += 300;
    // y += 120;
    y += 1;

    println!("x={} y={}", x, y);

    // Vectors, range loops, functions (&mut)

    let mut evens: Vec<i32> = Vec::new();
    for i in 0..20 {
        keep_evens(&mut evens, i);
    }
    for n in evens.iter() {
        print!("{} ",n)
    }
    println!();

    // Iterators

    // let mut sum = 0;
    // for number in evens {
    //     sum += number;
    // }
    let sum = evens.iter().sum::<i32>();
    println!("Sum = {}", sum);




    // Functions that return Option

    match int_divide(5,0) {
        None => println!("Can not divide by 0."),
        Some(value) => println!("Result = {}", value)
    }

    // While loops, input, functions that return Result, Traits, Dependencies

    loop {
        println!("Enter new password: ");
        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        password = password.trim().to_string();
        if password.is_empty() {
            break;
        }
        let result = password_valid(&password);
        match result {
            Ok(hash) => println!("Valid! sha256 = {}",hash),
            Err(error) => println!("Error! {}",error)
        }
    }
}
