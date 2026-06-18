use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, AxumError>;

#[derive(Debug)]
pub enum AxumError {
    Io(io::Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl AxumError {
    pub fn not_found(&self) -> bool {
        if let AxumError::Io(ref io_error) = *self {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
        false
    }
}

impl error::Error for AxumError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            AxumError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for AxumError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxumError::Io(err) => write!(fmt, "{}", err),
            _ => unreachable!(),
        }
    }
}
