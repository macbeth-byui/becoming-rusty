use std::fs::{self, Metadata, ReadDir};
use std::path::PathBuf;
use crate::args::AppArgs;
use crate::errors::AppError;

#[derive(Debug, Clone)]
struct DirRec {
    pub name : String,
    pub path : PathBuf,
    pub file_type : OrderedFileType,
    pub meta : Metadata,
}

impl DirRec {
    fn from(entries : ReadDir) -> Result<Vec<Self>, AppError> {
        let mut results = vec!();
        for entry in entries {
            let entry = entry?;
            let entry_type = entry.file_type()?;
            results.push(DirRec {
                name : entry.file_name().to_string_lossy().to_string(),
                path : entry.path(),
                meta : entry.metadata()?,
                file_type : match (entry_type.is_file(), entry_type.is_dir(), entry_type.is_symlink()) {
                    (true, false, false) => OrderedFileType::File,
                    (false, true, false) => OrderedFileType::Dir,
                    (false, false, true) => OrderedFileType::Symlink,
                    _                    => OrderedFileType::Other,
                }
            });
        }
        Ok(results)
    }

    fn display(&self) {
        let disp_name = match self.file_type {
            OrderedFileType::File    => format!("{}", self.name),
            OrderedFileType::Dir     => format!("[{}]", self.name),
            OrderedFileType::Symlink => format!("<-{}", self.name),
            OrderedFileType::Other   => format!("?{}", self.name),
        };
        let flags_d = if self.file_type == OrderedFileType::Dir { "d" } else { "-" };
        
        println!("{}", disp_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum OrderedFileType {
    File = 0, 
    Dir = 1, 
    Symlink = 2, 
    Other = 255,
}

// Functional Entry Point for the App
pub fn run(_args : AppArgs) -> Result<(), AppError> {
    let mut records = DirRec::from(
        fs::read_dir(".")?
    )?;

    records.sort_by(|r1, r2|
      r1.file_type.cmp(&r2.file_type)
        .then_with(|| r1.name.to_uppercase().cmp(&r2.name.to_uppercase()))  
    );

    for record in records.iter() {
        record.display();
    }

    Ok(())
} 