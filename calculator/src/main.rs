use clap::Parser;
use std::fmt;
use std::error;

// Command Line Setup

#[derive(Parser, Debug)]
#[command(version, about = "A simple post-fix calculator (+, -, *, /)")]
struct Args {
    /// Equation
    equation: String,
}

// Components of an Equation

#[derive(Debug)]
enum EqComponent {
    Value(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Error conditions for an Equation

#[derive(Debug)]
enum CalcError {
    InvalidNumber(String),
    InvalidOperator(char),
    MissingOperator,
    MissingOperand(char),
    DivideByZero
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivideByZero => write!(f, "Error: Divide By Zero."),
            CalcError::InvalidNumber(value) => write!(f, "Error: Invalid Number: {}", value),
            CalcError::InvalidOperator(value) => write!(f, "Error: Invalid Operator: {}", value),
            CalcError::MissingOperand(value) => write!(f, "Error: Missing Operand for: {}", value),
            CalcError::MissingOperator => write!(f, "Error: Missing Operator")
        }
    }
}

impl error::Error for CalcError {}

// Parse equation string into a vector of EqComponents
fn parse_equation(equation: &str) -> Result<Vec<EqComponent>, CalcError> {
    let mut in_digit = false;
    let mut components = Vec::<EqComponent>::new();
    let mut component = String::new();
    for c in equation.chars() {
        // TODO: Handling negative numbers
        if c.is_numeric() || c == '.' {
            in_digit = true;
            component.push(c);
        } else {
            if in_digit {
                match component.parse::<f64>() {
                    Ok(number) => components.push(EqComponent::Value(number)),
                    Err(_) => return Err(CalcError::InvalidNumber(component)),
                }
                component.clear();
                in_digit = false;
            }
            match c {
                '+' => components.push(EqComponent::Add),
                '-' => components.push(EqComponent::Subtract),
                '*' => components.push(EqComponent::Multiply),
                '/' => components.push(EqComponent::Divide),
                ' ' => (), // Skip whitespace
                _ => return Err(CalcError::InvalidOperator(c)),
            }
        }
    }
    // End of the string but I still have a number to add
    if in_digit {
        match component.parse::<f64>() {
            Ok(number) => components.push(EqComponent::Value(number)),
            Err(_) => return Err(CalcError::InvalidNumber(component)),
        }
    }
    Ok(components)
}

// Utility function to pop 2 items from a stack
fn pop2(stack: &mut Vec<f64>) -> Option<(f64, f64)> {
    let v1 = stack.pop();
    let v2 = stack.pop();
    if v1.is_none() || v2.is_none() {
        return None;
    }
    Some((v2.unwrap(), v1.unwrap()))
}


// Solve the parsed equation using a stack.  
// Numbers go on the stack.
// Operators will pop two values, perform the operation, and push the result
// Stack should have 1 thing at the end (the result)
fn solve(equation: &Vec<EqComponent>) -> Result<f64, CalcError> {
    let mut stack = Vec::<f64>::new();
    for component in equation {
        match component {
            // *value => push needs a value and not a reference.  This will dereference
            EqComponent::Value(value) => stack.push(*value),
            EqComponent::Add => {
                if let Some((v1, v2)) = pop2(&mut stack) {
                    stack.push(v1 + v2);
                } else {
                    return Err(CalcError::MissingOperand('+'));
                }
            }
            EqComponent::Subtract => {
                if let Some((v1, v2)) = pop2(&mut stack) {
                    stack.push(v1 - v2);
                } else {
                    return Err(CalcError::MissingOperand('-'));
                }
            }
            EqComponent::Multiply => {
                if let Some((v1, v2)) = pop2(&mut stack) {
                    stack.push(v1 * v2);
                } else {
                    return Err(CalcError::MissingOperand('*'));
                }
            }
            EqComponent::Divide => {
                if let Some((v1, v2)) = pop2(&mut stack) {
                    if v2 == 0.0 {
                        return Err(CalcError::DivideByZero);
                    }
                    stack.push(v1 / v2);
                } else {
                    return Err(CalcError::MissingOperand('/'));
                }
            }
        }
    }
    if stack.len() == 1 {
        return Ok(stack.pop().unwrap());
    }
    Err(CalcError::MissingOperator)

}

fn main() {
    // Read command line arguments
    let args = Args::parse();
    
    // and_then will run closure if ok otherwise maintain the err result
    let result = parse_equation(&args.equation).and_then(|a| solve(&a));
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err)
    }
}
