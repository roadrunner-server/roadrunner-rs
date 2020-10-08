use std::fmt::Formatter;

pub enum Error {
    PipeError { cause: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PipeError { cause } => {
                write!(f, "pipe send error, cause: {}", cause)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::PipeError { cause: error.to_string() }
    }
}