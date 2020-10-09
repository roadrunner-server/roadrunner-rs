use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Error {
    WaitError { cause: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::WaitError { cause } => write!(f, "wait error, reason: {}", cause),
        }
    }
}
