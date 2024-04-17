use std::collections::HashMap;
use std::net::TcpStream;
use crate::request::Request;
use crate::response::Response;
use crate::file_system::FileSystem;

pub struct Client {
    stream : TcpStream,
    file_system : FileSystem
}

impl Client {

    pub fn new(stream : TcpStream, file_system : FileSystem) -> Self {
        Client { stream, file_system }
    }

    pub fn run(&mut self) {
        let request = match Request::read_from_stream(&mut self.stream) {
            Ok(request) => request,
            Err(_) => return 
        };

        // println!("Request: {:?}", request);

        let mut response = self.process_request(request);

        // println!("Response: {:?}", response);

        let _ = response.write_to_stream(&mut self.stream);
       
    }

    // TODO: SUpport index.htm (config file)
    // TODO: Use chaining for building response

    fn process_request(&self, request : Request) -> Response {
        let mut target = request.target;
        if target == "/" {
            target = "/index.html".to_string();
        }
        let file = self.file_system.get_file(&target);
        if let Ok(text) = file {
            let version = request.version.clone();
            let status_code = String::from("200");
            let status_text = String::from("OK");
            let mut headers = HashMap::<String, String>::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            headers.insert("Content-Length".to_string(), text.len().to_string());
            return Response::new(version, status_code, status_text, headers, text);
        } 
        let version = request.version.clone();
        let status_code = String::from("404");
        let status_text = String::from("NOT FOUND");
        let headers = HashMap::<String, String>::new();
        let body = String::new();        
        Response::new(version, status_code, status_text, headers, body)
    }
    

}