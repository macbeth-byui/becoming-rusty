use crate::method::Method;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read, Error, ErrorKind};
use std::net::TcpStream;
use std::str;
use std::cmp;

const STREAM_MAX_READ: u32 = 1024;

#[derive(Debug)]
pub struct Request {
    pub method : Method,
    pub target : String,
    pub version : String,
    pub headers : HashMap<String, String>,
    pub body : String
}

impl Request {

    /* This function is the only way to create a Request object.  The provided
     * TcpStream for the client is used to read the command, the headers,
     * and the body of the request.
     */
    pub fn read_from_stream(stream : &mut TcpStream) -> io::Result<Request> {
        let mut reader = BufReader::new(stream);

        // Read the command line (required)
        let (method, target, version) = 
            Request::read_request_command(&mut reader)?;

        // Read the headers which might return back as empty.
        let headers = 
            Request::read_request_headers(&mut reader)?;

        // Read the body only if there is Content-Length in the headers
        let body = match headers.get("Content-Length") {
            Some(str_value) => {
                let length = match str::parse::<u32>(str_value) {
                    Ok(value) => value,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, 
                        format!("Invalid Content Length: {}", str_value)))
                };
                Request::read_request_body(&mut reader, length)?
            }
            None => String::new() // Default body is empty string
        };

        // Create the Request object
        Ok(Request {method, target, version, headers, body})
    }

    fn read_request_command(stream : &mut BufReader<&mut TcpStream>) -> io::Result<(Method, String, String)> {
        let mut data = String::new();
    
        // Read the one command line
        stream.read_line(&mut data)?;
        if data.is_empty() {
            return Err(Error::new(ErrorKind::ConnectionReset, 
                "No Request (Closed)"));
        }
    
        // Should have 3 parts separated by whitespace: method, target, version
        let parsed = data.trim().split(' ').collect::<Vec<&str>>();
        let method = match parsed.first() {
            // Store enumerated command type
            Some(&"GET") => Method::Get,
            Some(&"POST") => Method::Post,
            _ => return Err(Error::new(ErrorKind::InvalidData, 
                format!("Invalid Command in Request: {}", data)))
        };
        let target = match parsed.get(1) {
            Some(&value) => value,
            _ => return Err(Error::new(ErrorKind::InvalidData, 
                format!("Invalid Target in Request: {}", data)))
        };
        let version = match parsed.get(2) {
            Some(&value) => value,
            _ => return Err(Error::new(ErrorKind::InvalidData, 
                format!("Invalid Version in Request: {}", data)))
        };        
        Ok((method, target.to_string(), version.to_string()))
    }
    
    fn read_request_headers(stream : &mut BufReader<&mut TcpStream>) -> io::Result<HashMap<String,String>> {
        let mut headers = HashMap::<String,String>::new();
    
        // Read lines until we get to an empty line (end of the headers)
        for line_result in stream.lines() {
    
            let line = line_result?;
    
            // End of the header section
            if line.is_empty() {
                break;
            }
    
            // Put key/value pair into the map
            if let Some((key, value)) = line.trim().split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            } 
            else {
                return Err(Error::new(ErrorKind::InvalidData, 
                    format!("Invalid Header: {}", line)));
            }
        }
        Ok(headers)
    }
    
    fn read_request_body(stream : &mut BufReader<&mut TcpStream>, expected : u32) -> io::Result<String> {
        let mut body = String::new();
        let mut bytes_total = 0;
    
        // Read upto 1024 bytes at a time and combine together
        while bytes_total < expected {
    
            // Buffer to store upto 1024 bytes
            let mut buffer = [0_u8; STREAM_MAX_READ as usize];
    
            // Only read into the part that we still need (expected-bytes_total)
            let max_bytes = cmp::min(STREAM_MAX_READ, expected-bytes_total) as usize;
            let bytes_read = stream.read(&mut buffer[..max_bytes])? as u32;
    
            // Error Conditions
            if bytes_read == 0 || bytes_total + bytes_read > expected {
                return Err(Error::new(ErrorKind::InvalidData, 
                    format!("Invalid Body Length: Actual={} Expected={}", 
                        bytes_total+bytes_read, expected)));
            }
            
            // Convert bytes to a str and append to the body 
            let new_str = str::from_utf8(&buffer[..bytes_read as usize])
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Unable to convert body bytes"))?;
    
            body.push_str(new_str);
    
            bytes_total += bytes_read;
        }
        Ok(body)
    }
    
    
}