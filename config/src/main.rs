use std::io::{self, BufReader, Read};
use std::fs::File;
use std::collections::HashMap;

/// Read a config file using the name specified.
/// The format of the config file is:
/// 
/// NAME = VALUE
/// 
/// Blank lines are ignored.  Lines starting with # are ignored.
/// Starting and ending whitespace for names and values are ignored.
/// If a value is contained within double quotes, then ending 
/// whitespace is used.
/// 
/// If a name is redefined, then the new value is used.
/// 
/// Returns an io::Error if the config file is invalid.  Otherwise,
/// a HashMap of Strings is returned.
fn read_config(filename : &str) -> io::Result<HashMap<String, String>> {
    // Read the config file
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let mut config = HashMap::new();

    // Process each line of the config 
    for (line, text) in buffer.lines().enumerate() {

        // Skip blank lines and comments
        if text.trim().is_empty() ||
           text.trim().starts_with("#") {
            continue;
        }

        // Parse the line into name and value using equal sign
        // Note that the split function will produce only one item
        // if there is no equal sign in the text.
        let mut parsed = text.split("=");
        let name = parsed.next().unwrap().trim();
        let value = parsed.next()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Missing Equal (line {})", line + 1)))?
            .trim();

        // Missing Value
        if value.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Missing Value (line {})", line + 1)));
        }

        // Value using double quotes
        if value.starts_with("\"") && value.ends_with("\"") {
            // Remove the double quotes from the value
            config.insert(name.to_string(), value[1..value.len()-1].to_string());
        }

        // Value missing starting or ending double quote
        else if value.starts_with("\"") || value.ends_with("\"") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Missing Double Quote (line {})", line + 1)));
        }

        // Value without double quotes
        else {
            config.insert(name.to_string(), value.to_string());
        }
    }
    Ok(config)
}

fn main() {
    let config = read_config("config.cfg").unwrap();
    println!("{:?}", config);
}
