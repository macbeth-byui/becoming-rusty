use parsed_args::{ArgState, ParsedArgs};
use serde_json::Value;
use std::io::Read;

fn read_api(url: &str) -> String {
    // reqwest library provides HTTP request capability
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut result = String::new();
    response.read_to_string(&mut result).unwrap();
    return result;
}

fn parse_json(data: &String) -> Value {
    // serde_json does JSON parsing
    let parsed: Value = serde_json::from_str(data).unwrap();
    return parsed;
}

fn display_json(json: &Value) {
    // Assuming that the result starts with a dictionary map
    println!("JSON");
    display_json_map(json, "|--");
}

fn display_json_map(json: &Value, indent: &str) {
    // Convert Value from serde to an object (map)
    // Using unwrap everywhere but should look at handling errors better
    let map = json.as_object().unwrap();

    // Loop through each key/value pair
    for (count, pair) in map.iter().enumerate() {
        let key = pair.0;
        let value = pair.1;

        // Draw indentation lines
        let new_indent = if count == (map.len() - 1) {
            " ".repeat(indent.len()) + "|--"
        } else {
            indent.replace("-", " ") + "|--"
        };

        // Recurse to either an array, map or no recurse with a value
        match value {
            Value::Array(_) => {
                println!("{}{}", indent, key);
                display_json_array(key, value, new_indent.as_str());
            }
            Value::Object(_) => {
                println!("{}{}", indent, key);
                display_json_map(value, new_indent.as_str());
            }
            _ => println!("{}{} = {}", indent, key, value),
        };
    }
}

fn display_json_array(key: &str, json: &Value, indent: &str) {
    // Convert Value from serde to an array
    // Using unwrap everywhere but should look at handling errors better
    let array = json.as_array().unwrap();

    // Loop through each element of the array
    for (count, value) in array.iter().enumerate() {
        // Display the index
        print!("{}[{}]", indent, count + 1);

        // Draw indentation lines
        let new_indent = if count == (array.len() - 1) {
            " ".repeat(indent.len()) + "|--"
        } else {
            indent.replace("-", " ") + "|--"
        };

        // Recurse to either an array, map or no recurse with a value
        match value {
            Value::Array(_) => {
                println!();
                display_json_array(key, value, new_indent.as_str())
            }
            Value::Object(_) => {
                println!();
                display_json_map(value, new_indent.as_str());
            }
            _ => println!(" {}", value),
        };
    }
}

fn main() {
    let time = 5;
    println!("{time}");
    let args = ParsedArgs::new();
    match args.get_key_arg::<String>("url") {
        ArgState::Value(url) => {
            let result = read_api(&url);
            let parsed = parse_json(&result);
            display_json(&parsed);
        }
        ArgState::Invalid => {
            println!("Invalid url string");
        }
        ArgState::None => {
            println!("Missing -url <string>");
        }
    }
}
