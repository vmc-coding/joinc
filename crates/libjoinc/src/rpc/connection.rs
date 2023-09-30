use crate::error::{Error, Result};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

const REPLY_OPENING: &[u8] = b"<boinc_gui_rpc_reply>";
const REPLY_CLOSING: &[u8] = b"</boinc_gui_rpc_reply>";

const REQUEST_OPENING: &[u8] = b"<boinc_gui_rpc_request>\n";
const REQUEST_CLOSING: &[u8] = b"\n</boinc_gui_rpc_request>\x03";

const EOM: u8 = 0x03;

pub const DEFAULT_PORT: u16 = 31416;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn open(host: &str, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((host, port)).map_err(|err| Error::Io(err))?;
        Ok(Connection { stream })
    }

    pub fn do_rpc(&mut self, request: &[u8]) -> Result<Vec<u8>> {
        self.stream.write(REQUEST_OPENING)?;
        self.stream.write(request)?;
        self.stream.write(REQUEST_CLOSING)?;

        let mut result: Vec<u8> = vec![];

        {
            let mut buffer = [0; 4096];
            loop {
                let bytes_read = self
                    .stream
                    .read(&mut buffer)
                    .map_err(|err| Error::Io(err))?;

                if bytes_read == 0 {
                    return Err(Error::Io(std::io::Error::from(ErrorKind::UnexpectedEof)));
                }

                let to_write = if buffer[bytes_read - 1] == EOM {
                    bytes_read - 1
                } else {
                    bytes_read
                };

                result.extend_from_slice(&buffer[..to_write]);

                if bytes_read != to_write {
                    break;
                }
            }
        }

        let opening = result
            .windows(REPLY_OPENING.len())
            .position(|w| w == REPLY_OPENING)
            .ok_or(Error::Rpc(format!("Not a GUI RPC response: {:?}", result)))?;
        let closing = result
            .windows(REPLY_CLOSING.len())
            .rposition(|w| w == REPLY_CLOSING)
            .ok_or(Error::Rpc(format!("Not a GUI RPC response: {:?}", result)))?;

        result.drain(closing..);
        result.drain(..opening + REPLY_OPENING.len());

        Ok(result)
    }
}
