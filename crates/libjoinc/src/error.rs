#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    InvalidXml(String),
    Io(std::io::Error),
    Rpc(String),
    Unauthorized,
    UnsupportedEncoding,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
