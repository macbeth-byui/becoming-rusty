use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;
use deflate::deflate_bytes;

#[derive(Clone)]
pub struct FileSystem {
    path : String
}

impl FileSystem {

    /* Create a FileSystem using the path.
     * 
     * TODO: Can we make this a singleton?
     */
    pub fn new(path : &str) -> Self {
        FileSystem {path : path.to_string()}
    }

    /* Verify if the folder path is valid
     */
    pub fn check_folder(&self) -> io::Result<()> {
        fs::read_dir(&self.path)?;
        Ok(())
    }    

    /* Get file type
     */
    fn get_type(&self, target : &str) -> Option<String> {
        if let Some(extension) = Path::new(target).extension() {
            return Some(extension.to_string_lossy().into_owned());
        }
        None
        
    }

    /* Obtain a file and return bytes and mime type.  For impages,
     * compress the file.
     */
    pub fn get_file(&self, target : &str) -> io::Result<(Vec<u8>,&str)> {
        let file = File::open(format!("{}/{}",self.path, target))?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::<u8>::new();
        reader.read_to_end(&mut bytes)?;
        let (mime_type, compress) = match self.get_type(target) {
            Some(ext) if ext == "html" => ("text/html",false),
            Some(ext) if ext == "jpeg" => ("image/jpeg",true),
            Some(_) => ("application/octet-stream",false),
            None => ("application/octet-stream",false)
        };


        // TODO: Reserach compression (is jpeg already compressed?)
        
        // if compress {
        //     let compressed = deflate_bytes(bytes.as_slice());
        //     return Ok((compressed.to_vec(), mime_type));
        // }

        Ok((bytes, mime_type))
    }
    
}