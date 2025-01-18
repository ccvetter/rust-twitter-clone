use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub(crate) enum CustomError {
    DatabaseError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::DatabaseError(ref message) => write!(f, "Database Error: {}", message),
        }
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::DatabaseError(_) => "Database Error",
        }
    }
}
