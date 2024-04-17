use std::collections::HashMap;
use std::env;

pub enum ArgState<T> {
    Value(T),
    Invalid,
    None
}

pub struct ParsedArgs {
    key_args: HashMap<String,String>,
    free_args: Vec<String>
}

impl Default for ParsedArgs {
    fn default() -> Self {
        ParsedArgs::new()
    }
}

impl ParsedArgs {
    pub fn new() -> Self {
        // Obtain the command line arguments but ignore the first one
        // which is the executable name.
        let args : Vec<String> = env::args().skip(1).collect();

        // Crate memory for the results.  We will transfer
        // ownership of these when they are returned from the
        // function.
        let mut key_args: HashMap<String, String> = HashMap::new();
        let mut free_args: Vec<String> = Vec::new();

        // State variables
        let mut in_key = false;
        let mut prev_arg= "";

        // Parse each argument looking for -{key}
        // Zero or one value is assigned to each key in the key arguments
        // Values not associated with a key are stored as free arguments
        for arg in args.iter() {
            if arg.starts_with('-') {
                // String slicing produces a reference &str
                let key = arg.strip_prefix('-').unwrap();
                key_args.insert(key.to_string(), String::new());
                in_key = true; // The next argument is the value
                prev_arg = key;
            } else if in_key {
                key_args.insert(prev_arg.to_string(), arg.to_string());
                in_key = false;
            } else {
                free_args.push(arg.to_string());
            }
        }

        // Transfer ownership
        ParsedArgs { key_args, free_args }
    }

    pub fn get_key_arg<T: std::str::FromStr>(&self, key: &str) -> ArgState<T>
    {
        match self.key_args.get(key) {
            None => ArgState::None,
            Some(value) => ParsedArgs::parse_arg::<T>(value)
        }
    }


    pub fn get_free_arg<T: std::str::FromStr>(&self, index: usize) -> ArgState<T>
    {
        match self.free_args.get(index) {
            None => ArgState::None,
            Some(value) => ParsedArgs::parse_arg::<T>(value)
        }
    }

    // Utility function to parse the argument and return the proper state
    fn parse_arg<T: std::str::FromStr>(value : &str) -> ArgState<T>
    {
        match value.parse::<T>() {
            Ok(value) => ArgState::Value(value),
            Err(_) => ArgState::Invalid
        }
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}