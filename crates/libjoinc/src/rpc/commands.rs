use crate::error::{Error, Result};
use crate::rpc::connection::{Connection, Operation};
use crate::rpc::xml_serde::SerializeInto;
use crate::types::*;
use crate::xml;

pub trait Command {
    fn execute(&mut self, connection: &mut Connection) -> Result<()>;

    fn needs_authorization(&self) -> bool {
        false
    }
}

// ----- AuthorizeCommand -----

#[derive(Default)]
struct Auth1Operation {
    nonce: Option<String>,
}

impl Operation for Auth1Operation {
    fn serialize(&self) -> xml::Node {
        xml::Node::new("auth1")
    }

    fn deserialize(&mut self, root: xml::Node) -> Result<()> {
        self.nonce = Some(root.try_child_into_content("nonce")?);
        Ok(())
    }
}

struct Auth2Operation {
    password: String,
    nonce: String,
}

impl Operation for Auth2Operation {
    fn serialize(&self) -> xml::Node {
        let hash = md5::compute((self.nonce.to_owned() + &self.password).as_bytes());

        let mut node = xml::Node::new("auth2");
        node.add_new_content_child("nonce_hash", format!("{:x}", hash));
        node
    }

    fn deserialize(&mut self, root: xml::Node) -> Result<()> {
        assert_child_exists(&root, "authorized")
    }
}

pub struct AuthorizeCommand {
    password: String,
}

impl AuthorizeCommand {
    pub fn new<T: Into<String>>(password: T) -> Self {
        AuthorizeCommand {
            password: password.into(),
        }
    }
}

impl Command for AuthorizeCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<()> {
        let mut auth1 = Auth1Operation::default();
        connection.do_rpc_operation(&mut auth1)?;

        let mut auth2 = Auth2Operation {
            password: self.password.clone(),
            nonce: auth1.nonce.ok_or(Error::Rpc(
                "The nonce for authorization is missing".to_string(),
            ))?,
        };
        connection.do_rpc_operation(&mut auth2)
    }
}

// ----- ExchangeVersionsCommand -----

#[derive(Default)]
pub struct ExchangeVersionsCommand {
    request: Version,
    pub version: Option<Version>,
}

impl ExchangeVersionsCommand {
    pub fn new(request: Version) -> Self {
        Self {
            request,
            version: None,
        }
    }
}

impl Command for ExchangeVersionsCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<()> {
        connection.do_rpc_operation(self)
    }
}

impl Operation for ExchangeVersionsCommand {
    fn serialize(&self) -> xml::Node {
        let node = xml::Node::new("exchange_versions");
        self.request.serialize_into(node)
    }

    fn deserialize(&mut self, root: xml::Node) -> Result<()> {
        let version_node = need_child(&root, "server_version")?;
        self.version = Some(version_node.try_into()?);
        Ok(())
    }
}

// ----- ReadCCConfigCommand -----

pub struct ReadCCConfigCommand {}

impl ReadCCConfigCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ReadCCConfigCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<()> {
        connection.do_rpc_operation(self)
    }

    fn needs_authorization(&self) -> bool {
        true
    }
}

impl Operation for ReadCCConfigCommand {
    fn serialize(&self) -> xml::Node {
        xml::Node::new("read_cc_config")
    }

    fn deserialize(&mut self, root: xml::Node) -> Result<()> {
        assert_child_exists(&root, "success")
    }
}

// ----- some helper function -----

fn assert_child_exists(node: &xml::Node, tag: &str) -> Result<()> {
    node.find_child(tag)
        .map(|_| ())
        .ok_or(Error::Rpc(format!("Expected tag '{}' not found.", tag)))
}

fn need_child<'a, 'b>(node: &'a xml::Node, tag: &'b str) -> Result<&'a xml::Node> {
    node.find_child(tag)
        .ok_or(Error::Rpc(format!("Expected tag '{}' not found.", tag)))
}
