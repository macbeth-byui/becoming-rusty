use std::fs::{self, File};
use std::io::{self, BufReader, Read, Error, ErrorKind};

#[derive(Clone)]
pub struct FileSystem {
    path : String
}

impl FileSystem {
    pub fn new(path : &str) -> Self {
        FileSystem {path : path.to_string()}
    }

    pub fn check_folder(&self) -> io::Result<()> {
        fs::read_dir(&self.path)?;
        Ok(())
    }    

    pub fn get_file(&self, target : &str) -> io::Result<String> {
        let file = File::open(format!("{}/{}",self.path, target))?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::<u8>::new();
        reader.read_to_end(&mut bytes)?;
        let text = String::from_utf8(bytes).
            map_err(|_| Error::new(ErrorKind::InvalidData, "Unable to convert body bytes"))?;
        Ok(text)
    }
    
}