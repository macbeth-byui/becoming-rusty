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

    fn process_request(&self, request : Request) -> Response {
        let mut target = request.target;
        if target == "/" {
            target = "/index.html".to_string();
        }
        let file = self.file_system.get_file(&target);
        let mut response = Response::new();
        if let Ok(text) = file 
        {
            response.version(&request.version)
                    .ok()
                    .body_html(&text);        
        } 
        else {
            response.version(&request.version)
                    .not_found();
        }
        response
    }
    

}