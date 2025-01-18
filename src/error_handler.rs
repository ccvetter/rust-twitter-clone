use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub(crate) enum CustomError {
    NotFound,
    BadRequest,
    InternalServerError,
    DatabaseError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::NotFound => write!(f, "Not Found"),
            CustomError::BadRequest => write!(f, "Bad Request"),
            CustomError::InternalServerError => write!(f, "Internal Server Error"),
            CustomError::DatabaseError(ref message) => write!(f, "Database Error: {}", message),
        }
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::NotFound => "Not Found",
            CustomError::BadRequest => "Bad Request",
            CustomError::InternalServerError => "Internal Server Error",
            CustomError::DatabaseError(_) => "Database Error",
        }
    }
}
