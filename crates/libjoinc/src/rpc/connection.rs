use crate::error::{Error, Result};
use crate::xml;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

const EOM: u8 = 0x03;

pub const DEFAULT_PORT: u16 = 31416;

pub trait Operation {
    fn serialize(&self) -> xml::Node;
    fn deserialize(&mut self, root: xml::Node) -> Result<()>;
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn open(host: &str, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((host, port)).map_err(|err| Error::Io(err))?;
        Ok(Connection { stream })
    }

    pub fn do_rpc_operation<T: Operation>(&mut self, op: &mut T) -> Result<()> {
        let mut request_tree = xml::boinc_request_tree();
        request_tree.add_child(op.serialize());

        let response = self.do_rpc(request_tree.pretty_print(2).as_bytes())?;
        let response_tree = xml::parse(&response)?;

        let not_boinc_reply = Some(response_tree.tag == xml::BOINC_GUI_RPC_REPLY_TAG)
            .filter(|is_reply| !is_reply)
            .map(|_| {
                Error::Rpc(format!(
                    "Unexpected root element in reply: {}",
                    response_tree.tag
                ))
            });
        let unauthorized = response_tree
            .find_child("unauthorized")
            .map(|_| Error::Unauthorized);
        let error = response_tree
            .find_child("error")
            .map(|err_node| err_node.try_into_content::<&str>())
            .map(|msg_or_error| match msg_or_error {
                Ok(msg) => Error::Rpc(msg.to_string()),
                Err(err) => err,
            });

        not_boinc_reply
            .or(unauthorized)
            .or(error)
            .map(|err| Err(err))
            .unwrap_or(op.deserialize(response_tree))
    }

    pub fn do_rpc(&mut self, request: &[u8]) -> Result<Vec<u8>> {
        self.stream.write(request)?;
        self.stream.write(&[EOM; 1])?;

        let mut buffer = [0; 4096];
        let mut result: Vec<u8> = vec![];

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

        Ok(result)
    }
}
