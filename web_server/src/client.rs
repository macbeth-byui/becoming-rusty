use std::net::TcpStream;
use crate::request::Request;
use crate::response::Response;
use crate::file_system::FileSystem;

pub struct Client {
    stream : TcpStream,
    file_system : FileSystem
}

impl Client {

    /* Create a new Client from an already created TcpStream and FileSystem
     */
    pub fn new(stream : TcpStream, file_system : FileSystem) -> Self {
        Client { stream, file_system }
    }

    /* The Client will read a request, process the request, and send a response.
     * The Client will disconnect immediately.
     */
    pub fn run(&mut self) {
        // If an invalid request was read, then we will exit the client.
        let request = match Request::read_from_stream(&mut self.stream) {
            Ok(request) => request,
            Err(_) => return 
        };

        // Process the request
        let mut response = self.process_request(request);

        // Send a response (ignore any errors since we exiting the client anyways)
        let _ = response.write_to_stream(&mut self.stream);
    }

    /* Process a Request and produce a Response.  If requesting root, then we will
     * return index.html.  
     */
    fn process_request(&self, request : Request) -> Response {
        let mut target = request.target;

        // TODO: Check to see if no file was requested and then default 
        if target == "/" {
            target = "/index.html".to_string();
        }

        // TODO: Return back binary or text
        let file = self.file_system.get_file(&target);

        // TODO: Validate the file type with any headers requiring a specific type

        // Send back success if found or error if not found
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