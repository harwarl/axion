use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Error {
    pub fn not_found(&self) -> bool {
        if let Error::Io(ref io_error) = *self {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
        false
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(fmt, "{}", err),
            _ => unreachable!(),
        }
    }
}
