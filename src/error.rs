use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidFormat,
    OutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::InvalidFormat => "Invalid format",
            Error::OutOfRange => "Out of range",
        };
        f.write_str(s)
    }
}

impl error::Error for Error {}
