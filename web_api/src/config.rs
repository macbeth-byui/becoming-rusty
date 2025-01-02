use std::collections::HashMap;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::macros::err;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub current_courses : HashMap<String, i32>,
    pub trends_config : HashMap<String, Vec<(i32, Vec<i32>)>>,
}


impl Config {
    pub fn load_config() -> Result<Self,String> {
        let file = File::open("Horizons.toml")
            .map_err(|e| err!("TOML File Failure",e))?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)
            .map_err(|e| err!("TOML File Read Failure",e))?;
        let contents = String::from_utf8(buffer)
            .map_err(|e| err!("TOML Parsing Failure", e))?;
        toml::from_str(&contents)
            .map_err(|e| err!("TOML Parsing Failure",e))
    }
}

// See Horizons.toml
