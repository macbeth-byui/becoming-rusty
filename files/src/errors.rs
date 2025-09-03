use std::{fmt,io};

// Errors that this App can produce
pub enum AppError {
    IOError(io::Error),
}

// Convert io::Error to AppError
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IOError(e)
    }
}

// Custom display of the Errors
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IOError(arg) => write!(f, "Error (IO): {}", arg),
        }
    }
}
