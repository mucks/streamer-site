use std::io;
use std::option;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    NoneError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}
