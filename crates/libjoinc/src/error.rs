#[derive(Debug)]
pub enum Error {
    Client(String),
    Deserialization(libjoincserde::Error),
    Io(std::io::Error),
    Rpc(String),
    Unauthorized,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<libjoincserde::Error> for Error {
    fn from(err: libjoincserde::Error) -> Error {
        Error::Deserialization(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
