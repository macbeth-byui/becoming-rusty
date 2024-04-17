use std::collections::HashMap;
use std::io::{self, Write, BufWriter};
use std::net::TcpStream;


#[derive(Debug)]
pub struct Response {
    version : String,
    status_code : String,
    status_text : String,
    headers : HashMap<String, String>,
    body : String
}

impl Response {

    // TODO: Add functions to create status code and text

    // TODO: Add functions to set version

    // TODO: Add functions set Content-Type & Content-Size

    pub fn new(version : String, status_code : String,
        status_text : String, headers : HashMap<String,String>,
        body : String) -> Self {
            Response {version, status_code, status_text, headers, body}
    }
    
    pub fn write_to_stream(&mut self, client : &mut TcpStream) -> io::Result<()> {
        let mut writer = BufWriter::new(client);
        let mut data = Vec::<u8>::new();
        data.extend(format!("{} {} {}\r\n",
            self.version, 
            self.status_code, 
            self.status_text).as_bytes().to_vec());
        for (key,value) in self.headers.iter() {
            data.extend(format!("{}: {}\r\n", key, value).as_bytes().to_vec());
        }
        data.push(b'\r');
        data.push(b'\n');
        data.extend(self.body.as_bytes().to_vec());

        writer.write_all(&data)?;

        Ok(())
    }
}