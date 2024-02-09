use crate::error::{Error, Result};
use crate::rpc::connection::Connection;
use crate::types::*;
use libjoincserde::{from_str, to_vec};
use serde::{Deserialize, Serialize};

pub trait Command<RESP> {
    fn execute(&mut self, connection: &mut Connection) -> Result<RESP>;
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UnauthorizedReply {
    #[serde(rename = "unauthorized")]
    _unauthorized: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ErrorReply {
    error: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SuccessReply {
    #[serde(rename = "success")]
    _success: String,
}

fn execute_rpc_operation<REQ, RESP>(connection: &mut Connection, request: &REQ) -> Result<RESP>
where
    REQ: Serialize,
    RESP: for<'de> Deserialize<'de>,
{
    let raw_response = connection.do_rpc(&to_vec(request)?)?;
    // the root tag is a workaround for proper expected tag matching during deserialization
    let response = "<root>".to_string()
        + &String::from_utf8(raw_response)
            .map_err(|_| Error::Rpc("Recieved a non-UTF8 response from the client".to_string()))?
        + "</root>";

    match from_str(&response) {
        Ok(deserialized) => Ok(deserialized),
        Err(de_err) => match from_str::<ErrorReply>(&response) {
            Ok(error) => Err(Error::Client(error.error)),
            _ => match from_str::<UnauthorizedReply>(&response) {
                Ok(_) => Err(Error::Unauthorized),
                _ => Err(Error::Deserialization(de_err)),
            },
        },
    }
}

// ----- AuthorizeCommand -----

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "auth1"))]
struct Auth1Operation {
    #[serde(skip_serializing)]
    nonce: String,
}

impl Command<String> for Auth1Operation {
    fn execute(&mut self, connection: &mut Connection) -> Result<String> {
        let response: Auth1Operation = execute_rpc_operation(connection, self)?;
        Ok(response.nonce)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename(serialize = "auth2"))]
struct Auth2Operation {
    #[serde(skip_serializing)]
    authorized: Option<String>,
    #[serde(skip_deserializing)]
    nonce_hash: String,
}

impl Auth2Operation {
    fn new(password: &str, nonce: &str) -> Self {
        Self {
            authorized: None,
            nonce_hash: format!("{:x}", md5::compute(nonce.to_owned() + password)),
        }
    }
}

impl Command<bool> for Auth2Operation {
    fn execute(&mut self, connection: &mut Connection) -> Result<bool> {
        let response: Auth2Operation = execute_rpc_operation(connection, self)?;
        Ok(response.authorized.is_some())
    }
}

pub struct AuthorizeCommand {
    password: String,
}

impl AuthorizeCommand {
    pub fn new<T>(password: T) -> Self
    where
        T: Into<String>,
    {
        AuthorizeCommand {
            password: password.into(),
        }
    }
}

impl Command<()> for AuthorizeCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<()> {
        let mut auth1 = Auth1Operation::default();
        let nonce = auth1.execute(connection)?;

        let mut auth2 = Auth2Operation::new(&self.password, &nonce);
        if auth2.execute(connection)? {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }
}

// ----- ExchangeVersionsCommand -----

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "exchange_versions"))]
pub struct ExchangeVersionsCommand {
    #[serde(rename(deserialize = "server_version"))]
    version: Version,
}

impl ExchangeVersionsCommand {
    pub fn new(version: Version) -> Self {
        Self { version }
    }
}

impl Command<Version> for ExchangeVersionsCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<Version> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.version)
    }
}

// ----- GetCCStatusCommand -----

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "get_cc_status"))]
pub struct GetCCStatusCommand {
    #[serde(skip_serializing)]
    cc_status: CCStatus,
}

impl Command<CCStatus> for GetCCStatusCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<CCStatus> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.cc_status)
    }
}

// ----- GetMessagesCommand -----

#[derive(Default, Deserialize)]
struct MessagesDto {
    msg: Vec<Message>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "get_messages"))]
pub struct GetMessagesCommand {
    #[serde(skip_deserializing)]
    seqno: u32,
    #[serde(skip_serializing)]
    msgs: MessagesDto,
}

impl GetMessagesCommand {
    pub fn new(seqno: u32) -> Self {
        Self {
            seqno,
            msgs: MessagesDto { msg: vec![] },
        }
    }
}

impl Command<Vec<Message>> for GetMessagesCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<Vec<Message>> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.msgs.msg)
    }
}

// ----- GetNoticesCommand -----

#[derive(Default, Deserialize)]
struct NoticesDto {
    notice: Vec<Notice>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "get_notices"))]
pub struct GetNoticesCommand {
    #[serde(skip_deserializing)]
    seqno: u32,
    #[serde(skip_serializing)]
    notices: NoticesDto,
}

impl GetNoticesCommand {
    pub fn new(seqno: u32) -> Self {
        Self {
            seqno,
            notices: NoticesDto { notice: vec![] },
        }
    }
}

impl Command<Vec<Notice>> for GetNoticesCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<Vec<Notice>> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.notices.notice)
    }
}

// ----- GetProjectStatusCommand -----

#[derive(Default, Deserialize)]
struct ProjectsDto {
    project: Vec<Project>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename(serialize = "get_project_status"))]
pub struct GetProjectStatusCommand {
    #[serde(skip_serializing)]
    projects: ProjectsDto,
}

impl Command<Vec<Project>> for GetProjectStatusCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<Vec<Project>> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.projects.project)
    }
}

// ----- GetResultsCommand -----

#[derive(Default, Deserialize)]
struct ResultsDto {
    result: Vec<Task>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename(serialize = "get_results"))]
pub struct GetResultsCommand {
    #[serde(skip_deserializing)]
    active_only: bool,
    #[serde(skip_serializing)]
    results: ResultsDto,
}

impl GetResultsCommand {
    pub fn new(active_only: bool) -> Self {
        Self {
            active_only,
            results: ResultsDto { result: vec![] },
        }
    }
}

impl Command<Vec<Task>> for GetResultsCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<Vec<Task>> {
        let response: Self = execute_rpc_operation(connection, self)?;
        Ok(response.results.result)
    }
}

// ----- ReadCCConfigCommand -----

#[derive(Default, Serialize)]
#[serde(rename(serialize = "read_cc_config"))]
pub struct ReadCCConfigCommand {}

impl Command<()> for ReadCCConfigCommand {
    fn execute(&mut self, connection: &mut Connection) -> Result<()> {
        let _: SuccessReply = execute_rpc_operation(connection, self)?;
        Ok(())
    }
}
