use std::fmt::Formatter;

pub type GoridgeResult<T> = Result<T, Error>;

pub enum Error {
    PipeError { cause: String },

    PrefixValidationError { cause: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PipeError { cause } => write!(f, "pipe send error, cause: {}", cause),
            Error::PrefixValidationError { cause } => {
                write!(f, "prefix validation error: {}", cause)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::PipeError {
            cause: error.to_string(),
        }
    }
}
