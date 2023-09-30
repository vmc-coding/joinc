use crate::defs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Message {
    pub body: String,
    #[serde(rename = "pri")]
    pub priority: MsgInfo,
    pub project: String,
    pub seqno: i32,
    #[serde(rename = "time")]
    pub timestamp: i64,
}

impl std::default::Default for Message {
    fn default() -> Self {
        Self {
            body: String::new(),
            priority: MsgInfo::UnknownToWoinc,
            project: String::new(),
            seqno: 0,
            timestamp: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "version")]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub release: i32,
}

impl std::default::Default for Version {
    fn default() -> Self {
        Self {
            major: 7,
            minor: 22,
            release: 0,
        }
    }
}
