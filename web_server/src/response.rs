use std::collections::HashMap;
use std::io::{self, Write, BufWriter};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    version : String,
    status_code : String,
    status_text : String,
    headers : HashMap<String, String>,
    body : Vec<u8>
}

impl Response {

    /* Create an HTTP response.  The initial response is empty.  The user
     * must use the functions below to populate the message.
     */
    pub fn new() -> Self {
        Response {
            version: "".to_string(), 
            status_code: "".to_string(), 
            status_text: "".to_string(), 
            headers: HashMap::<String,String>::new(), 
            body: Vec::<u8>::new()
        }
    }
    
    /* Write the current response to the provided TcpStream.  Any IO
     * error that occurs will be returned.
     */
    pub fn write_to_stream(&mut self, client : &mut TcpStream) -> io::Result<()> {
        let mut writer = BufWriter::new(client);

        // Create a byte vector for the response
        let mut data = Vec::<u8>::new();

        // Add the command response
        data.extend(format!("{} {} {}\r\n",
            self.version, 
            self.status_code, 
            self.status_text).as_bytes().to_vec());

        // Add the headers
        for (key,value) in self.headers.iter() {
            data.extend(format!("{}: {}\r\n", key, value).as_bytes().to_vec());
        }
        // Provide a blank line after the headers
        data.push(b'\r');
        data.push(b'\n');

        // Add the body
        // data.extend(self.body.as_bytes().to_vec());
        data.extend(self.body.iter());

        // Send the byte vector to the client
        writer.write_all(&data)?;

        Ok(())
    }

    /* Set the version in the HTTP Response.  This function supports
     * chaining.
     */
    pub fn version(&mut self, version : &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    /* Sets the status code and text for an OK (200) response.  This 
     * function supports chaining.
     */
    pub fn ok(&mut self) -> &mut Self {
        self.status_code = "200".to_string();
        self.status_text = "OK".to_string();
        self
    }

    /* Sets the status code and text for a Not Found (404) response.
     * This function supports chaining.
     */
    pub fn not_found(&mut self) -> &mut Self {
        self.status_code = "404".to_string();
        self.status_text = "NOT FOUND".to_string();
        self
    }

    /* Adds a key/value pair to the headers.  This function supports
     * chaining.
     */
    pub fn header(&mut self, key : &str, value : &str) -> &mut Self {
        let _ = self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /* Adds body text as HTML format.  The content type and length
     * will be set in the headers.  This function supports chaining.
     */
    pub fn body(&mut self, body : &[u8], mime_type : &str) -> &mut Self {
        let length = body.len();
        self.header("Content-Type", mime_type)
            .header("Content-Length", &length.to_string());
        self.body = body.to_owned();
        self
    }

}