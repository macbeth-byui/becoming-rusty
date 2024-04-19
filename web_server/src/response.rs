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

    pub fn new() -> Self {
        Response {
            version: "".to_string(), 
            status_code: "".to_string(), 
            status_text: "".to_string(), 
            headers: HashMap::<String,String>::new(), 
            body: "".to_string()
        }
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

    pub fn version(&mut self, version : &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    pub fn ok(&mut self) -> &mut Self {
        self.status_code = "200".to_string();
        self.status_text = "OK".to_string();
        self
    }

    pub fn not_found(&mut self) -> &mut Self {
        self.status_code = "404".to_string();
        self.status_text = "NOT FOUND".to_string();
        self
    }

    pub fn header(&mut self, key : &str, value : &str) -> &mut Self {
        let _ = self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body_html(&mut self, text : &str) -> &mut Self {
        let length = text.len();
        self.header("Content-Type", "text/html")
            .header("Content-Length", &length.to_string());
        self.body = text.to_string();
        self
    }
}