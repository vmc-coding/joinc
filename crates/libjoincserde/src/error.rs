use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalState,
    Io(std::io::Error),
    UnexpectedXml(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::UnexpectedXml(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::UnexpectedXml(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedXml(msg) => formatter.write_str(msg),
            // TODO implement this
            _ => formatter.write_str(&format!("{:?}", self)),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
