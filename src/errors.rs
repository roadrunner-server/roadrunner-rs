use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Error {
    WaitError { cause: String },
    IoError { cause: String },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError { cause: err.to_string() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::WaitError { cause } => write!(f, "wait error, reason: {}", cause),
        }
    }
}
