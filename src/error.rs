use std::{
    error,
    fmt::{self, Display},
    io,
};

use scroll;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Zip(zip::result::ZipError),
    MalFormed(String),
    Scroll(scroll::Error),
    InvalidId(String),
    BadOffset(usize, String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(_) => "IO error",
            Error::Zip(_) => "Zip error",
            Error::MalFormed(_) => "Entity is malformed in some way",
            Error::Scroll(_) => "Scroll error",
            Error::InvalidId(_) => "Invalid index",
            Error::BadOffset(_, _) => "Invalid offset",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::IO(ref io) => io.source(),
            Error::Zip(ref zip) => zip.source(),
            Error::Scroll(ref err) => err.source(),
            Error::MalFormed(_) => None,
            Error::InvalidId(_) => None,
            Error::BadOffset(_, _) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<scroll::Error> for Error {
    fn from(err: scroll::Error) -> Error {
        Error::Scroll(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Error {
        Error::Zip(err)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => write!(fmt, "{}", err),
            Error::Zip(ref err) => write!(fmt, "{}", err),
            Error::Scroll(ref err) => write!(fmt, "{}", err),
            Error::MalFormed(ref msg) => write!(fmt, "Malformed entity: {}", msg),
            Error::InvalidId(ref msg) => write!(fmt, "{}", msg),
            Error::BadOffset(offset, ref msg) => write!(fmt, "{}: {}", msg, offset),
        }
    }
}
